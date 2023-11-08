//! Modules dedicated to the Cortex-A7 cores MPU0 and MPU1.

pub mod gic;
pub mod irq;
pub mod iwdg;

mod critical_section_impl;

use core::arch::global_asm;
use core::sync::atomic::{AtomicBool, Ordering};

pub use cortex_a7::memory::cache::clean_dcache_by_range;
use cortex_a7::memory::mmu::{TranslationTable, TRANSLATION_TABLE_LENGTH};
pub use cortex_a7::memory::MemoryRegion;

use crate::pac;

// Startup code for both Cortex-A cores.
global_asm!(include_str!("mpu_ca7/startup-vectors.s"));
global_asm!(include_str!("mpu_ca7/startup-mpu0.s"));
global_asm!(include_str!("mpu_ca7/startup-mpu1.s"));

/// CPU id for both MPUs. Also referred as bus master id for hardware semaphores.
pub const CPU_ID: u32 = 1;

/// Configuration settings.
#[derive(Debug)]
pub struct HalConfig {
    /// Function to return the memory region for an address.
    pub memory_region_mapper: fn(u32) -> MemoryRegion,
}

/// Returns the core id for the current core.
/// - `0`: MPU0
/// - `1`: MPU1
pub fn core_id() -> u32 {
    cortex_a7::core_id()
}

/// Initializes the HAL.
///
/// This function must be called once at the beginning of the main function for each MPU.
pub fn init(config: HalConfig) {
    match core_id() {
        0 => init_mpu0(config),
        1 => init_mpu1(config),
        _ => panic!("Invalid core id {}", core_id()),
    }
}

/// Flag for MPU0 being initialzed.
static MPU0_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Flag for MPU1 being initialzed.
static MPU1_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Returns if MPU0 is initialized.
pub fn is_mpu0_initialized() -> bool {
    MPU0_INITIALIZED.load(Ordering::Relaxed)
}

/// Returns if MPU1 is initialized.
pub fn is_mpu1_initialized() -> bool {
    MPU1_INITIALIZED.load(Ordering::Relaxed)
}

/// Initializes MPU0.
///
/// This function is called from `init` for MPU0.
/// It performs the following tasks:
/// - Enables the MMU of MPU0 with a translation table.
/// - Initializes the IRQs and GIC for MPU0.
fn init_mpu0(config: HalConfig) {
    cortex_a7::enable_scu();
    unsecure_peripherals();

    critical_section_impl::init();

    unsafe {
        cortex_a7::memory::mmu::init_translation_table(
            &mut MMU_TRANSLATION_TABLES.mpu0,
            config.memory_region_mapper,
        );
        cortex_a7::memory::mmu::enable(&MMU_TRANSLATION_TABLES.mpu0);
    }

    crate::gpio::init();
    crate::dma::init();
    irq::init();

    MPU0_INITIALIZED.store(true, Ordering::Relaxed);
}

/// Initializes MPU1.
///
/// This function is called from `init` for MPU1.
/// It performs the following tasks:
/// - Enables the MMU of MPU1 with a translation table.
/// - Initializes the GIC for MPU1.
fn init_mpu1(config: HalConfig) {
    unsafe {
        cortex_a7::memory::mmu::init_translation_table(
            &mut MMU_TRANSLATION_TABLES.mpu1,
            config.memory_region_mapper,
        );
        cortex_a7::memory::mmu::enable(&MMU_TRANSLATION_TABLES.mpu1);
    }

    gic::cpu_interface_init();

    MPU1_INITIALIZED.store(true, Ordering::Relaxed);
}

/// Starts MPU1.
///
/// This function can only called after MPU0 is initialized and will panic otherwise.
/// It generates a software interrupt to wakeup MPU1 out of WFI, which will then run some
/// startup code and pass execution to `mpu1_main`.
pub fn start_mpu1() {
    if !is_mpu0_initialized() {
        panic!("MPU1 can only be started when MPU0 is initialized.");
    }

    unsafe {
        // Turn on the Disable Backup Protection bit, to allow us to write to
        // the TAMP backup registers This is already on if we just booted from
        // U-boot, but if we are debugging and manually reset the code, we'll
        // want to be sure.
        let pwr = &(*pac::PWR::ptr());
        pwr.pwr_cr1.write(|w| w.dbp().set_bit());
        while pwr.pwr_cr1.read().dbp().bit_is_clear() {}

        // Turn off Write protection on backup registers (BOOTROM seems to turn
        // it on for Backup registers 0-4 during MPU1 boot-up)
        let tamp = &(*pac::TAMP::ptr());
        tamp.smcr.write(|w| {
            w.bkprwdprot()
                .bits(0)
                .bkpwdprot()
                .bits(0)
                .tampdprot()
                .set_bit()
        });

        extern "C" {
            /// Entry point for MPU1, defined in startup code.
            static mpu1_start: u32;
        }

        // Write the entry point address to TAMP backup register 5.
        let branch_address_register = &tamp.bkpr[5];
        let start_address = &mpu1_start as *const u32 as u32;
        branch_address_register.write(|w| w.bits(start_address));

        // Write the magic number 0xCA7FACE1 to backup register 4.
        let magic_number_register = &tamp.bkpr[4];
        magic_number_register.write(|w| w.bits(0xCA7FACE1));
    }

    // Enable Software Generated Interrupt 0.
    gic::enable_irq(irq::Irqn::SGI0 as u32);

    // Send SGI
    let filter_use_cpu_sel_bits = 0b00;
    let cpu1 = 1 << 1;
    gic::send_sgi(irq::Irqn::SGI0 as u32, cpu1, filter_use_cpu_sel_bits);
}

/// Resets MPU1.
pub fn reset_mpu1() {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.rcc_mp_grstcsetr.modify(|_, w| w.mpup1rst().set_bit());
    }
}

/// Starts the MCU.
///
/// The MCU starts execution in RETRAM at address 0x00000000,
/// which is mapped to virtual address 0x38000000 for the MPUs.
/// After a reset, the MCU will be on hold again.
pub fn start_mcu() {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.rcc_mp_gcr.modify(|_, w| w.boot_mcu().set_bit());
        rcc.rcc_mp_gcr.modify(|_, w| w.boot_mcu().clear_bit());
    }
}

/// Resets the MCU.
///
/// The MCU will not be started again after reset before calling `start_mcu`.
pub fn reset_mcu() {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.rcc_mp_grstcsetr.modify(|_, w| w.mcurst().set_bit());
    }
}

/// Resets the system.
pub fn reset_system() {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.rcc_mp_grstcsetr.modify(|_, w| w.mpsysrst().set_bit());
    }
}

/// Sets the whole RAM and secured peripherals unsecure in TrustZone controller
/// The RAM and some peripherals (e.g. USART1, I2C4, SPI6) are secured
/// by default after reset.
fn unsecure_peripherals() {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.rcc_tzcr
            .write(|w| w.tzen().clear_bit().mckprot().clear_bit());

        let etzpc = &(*pac::ETZPC::ptr());
        etzpc.etzpc_tzma1_size.write(|w| w.bits(0));
        etzpc.etzpc_decprot0.write(|w| w.bits(0xFFFFFFFF));
    }
}

/// MMU translation tables for both MPUs.
#[repr(C, align(16384))]
#[derive(Debug)]
struct TranslationTables {
    mpu0: TranslationTable,
    mpu1: TranslationTable,
}

/// MMU translation tables instance.
static mut MMU_TRANSLATION_TABLES: TranslationTables = TranslationTables {
    mpu0: [0; TRANSLATION_TABLE_LENGTH],
    mpu1: [0; TRANSLATION_TABLE_LENGTH],
};

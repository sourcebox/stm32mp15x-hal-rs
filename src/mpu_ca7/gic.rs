//! Global interrupt controller.

use crate::pac;

/// Enable the interrupt distributor using the GIC's CTLR register.
pub fn enable_distributor() {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        gicd.ctlr().modify(|_, w| w.enablegrp0().set_bit());
    }
}

/// Disable the interrupt distributor using the GIC's CTLR register.
pub fn disable_distributor() {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        gicd.ctlr().modify(|_, w| w.enablegrp0().clear_bit());
    }
}

/// Reads the GIC's TYPER register.
pub fn distributor_info() -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        gicd.typer().read().bits()
    }
}

/// Reads the GIC's IIDR register.
pub fn distributor_implementer() -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        gicd.iidr().read().bits()
    }
}

/// Sets the GIC's ITARGETSR register for the given interrupt.
/// - `irqn`: Interrupt to be configured.
/// - `cpu_target`: CPU interfaces to assign this interrupt to.
pub fn set_target(irqn: u32, cpu_target: u32) {
    let mask = itargetsr((irqn / 4) as usize) & !(0xFF << ((irqn % 4) * 8));
    set_itargetsr(
        (irqn / 4) as usize,
        mask | ((cpu_target & 0xFF) << ((irqn % 4) * 8)),
    );
}

/// Reads the GIC's ITARGETSR register.
/// - `irqn`: Interrupt to acquire the configuration for.
pub fn get_target(irqn: u32) -> u32 {
    (itargetsr((irqn / 4) as usize) >> ((irqn % 4) * 8)) & 0xFF
}

/// Enables the CPU's interrupt interface.
pub fn enable_interface() {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.ctlr().modify(|_, w| w.enablegrp0().set_bit());
    }
}

/// Disables the CPU's interrupt interface.
pub fn disable_interface() {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.ctlr().modify(|_, w| w.enablegrp0().clear_bit());
    }
}

/// Reads the CPU's IAR register.
pub fn acknowledge_pending() -> u32 {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.iar().read().bits()
    }
}

/// Writes the given interrupt number to the CPU's EOIR register.
/// - `irqn`: The interrupt to be signaled as finished.
pub fn end_interrupt(irqn: u32) {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.eoir().write(|w| w.bits(irqn));
    }
}

/// Enables the given interrupt using GIC's ISENABLER register.
/// - `irqn`: The interrupt to be enabled.
pub fn enable_irq(irqn: u32) {
    set_isenabler((irqn / 32) as usize, 1 << (irqn % 32));
}

/// Gets interrupt enable status using GIC's ISENABLER register.
/// - `irqn`: The interrupt to be queried.
/// - Returns:
///   - 0 - interrupt is not enabled
///   - 1 - interrupt is enabled
pub fn get_enable_irq(irqn: u32) -> u32 {
    (isenabler((irqn / 32) as usize) >> (irqn % 32)) & 1
}

/// Disables the given interrupt using GIC's ICENABLER register.
/// - `irqn`: The interrupt to be disabled.
pub fn disable_irq(irqn: u32) {
    set_icenabler((irqn / 32) as usize, 1 << (irqn % 32));
}

/// Gets interrupt pending status from GIC's ISPENDR register.
/// - `irqn`: The interrupt to be queried.
/// - Returns:
///   - 0 - interrupt is not pending
///   - 1 - interrupt is pendig.
pub fn get_pending_irq(irqn: u32) -> u32 {
    let mut pend;

    if irqn >= 16 {
        pend = (ispendr((irqn / 32) as usize) >> (irqn % 32)) & 1;
    } else {
        // INTID 0-15 Software Generated Interrupt
        pend = (spendsgir((irqn / 4) as usize) >> ((irqn % 4) * 8)) & 0xFF;

        // No CPU identification offered
        if pend != 0 {
            pend = 1;
        } else {
            pend = 0;
        }
    }

    pend
}

/// Sets the given interrupt as pending using GIC's ISPENDR register.
/// - `irqn`: The interrupt to be enabled.
pub fn set_pending_irq(irqn: u32) {
    if irqn >= 16 {
        set_ispendr((irqn / 32) as usize, 1 << (irqn % 32));
    } else {
        // INTID 0-15 Software Generated Interrupt
        set_spendsgir((irqn / 4) as usize, 1 << ((irqn % 4) * 8));
    }
}

/// Clears the given interrupt from being pending using GIC's ICPENDR register.
/// - `irqn`: The interrupt to be cleared.
pub fn clear_pending_irq(irqn: u32) {
    if irqn >= 16 {
        set_icpendr((irqn / 32) as usize, 1 << (irqn % 32));
    } else {
        // INTID 0-15 Software Generated Interrupt
        set_cpendsgir((irqn / 4) as usize, 1 << ((irqn % 4) * 8));
    }
}

/// Clears the given interrupt from being active using GIC's ICACTIVER register.
/// - `irqn`: The interrupt to be cleared.
pub fn clear_active_irq(irqn: u32) {
    if irqn >= 16 {
        set_icactiver((irqn / 32) as usize, 1 << (irqn % 32));
    }
}

/// Sets the interrupt configuration using GIC's ICFGR register.
/// - `irqn`: The interrupt to be configured.
/// - `int_config`: Int_config field value.
///   - Bit 0: Reserved (0 - N-N model, 1 - 1-N model for some GIC before v1)
///   - Bit 1: 0 - level sensitive, 1 - edge triggered
pub fn set_configuration(irqn: u32, int_config: u32) {
    let mut icfgr = icfgr((irqn / 16) as usize);
    let shift = (irqn % 16) << 1;

    icfgr &= !(3 << shift);
    icfgr |= int_config << shift;

    set_icfgr((irqn / 16) as usize, icfgr);
}

/// Gets the interrupt configuration from the GIC's ICFGR register.
/// - `irqn`: Interrupt to acquire the configuration for.
/// - Returns Int_config field value.
///   - Bit 0: Reserved (0 - N-N model, 1 - 1-N model for some GIC before v1)
///   - Bit 1: 0 - level sensitive, 1 - edge triggered
pub fn get_configuration(irqn: u32) -> u32 {
    icfgr((irqn / 16) as usize) >> ((irqn % 16) >> 1)
}

/// Sets the priority for the given interrupt in the GIC's IPRIORITYR register.
/// - `irqn`: The interrupt to be configured.
/// - `priority`: The priority for the interrupt, lower values denote higher priorities.
pub fn set_priority(irqn: u32, priority: u32) {
    let mask = ipriorityr((irqn / 4) as usize) & !(0xFF << ((irqn % 4) * 8));
    set_ipriorityr(
        (irqn / 4) as usize,
        mask | ((priority & 0xFF) << ((irqn % 4) * 8)),
    );
}

/// Reads the current interrupt priority from GIC's IPRIORITYR register.
/// - `irqn`: The interrupt to be queried.
pub fn get_priority(irqn: u32) -> u32 {
    (ipriorityr((irqn / 4) as usize) >> ((irqn % 4) * 8)) & 0xFF
}

/// Sets the interrupt priority mask using CPU's PMR register.
/// - `priority`: Priority mask to be set.
pub fn set_interface_priority_mask(priority: u32) {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.pmr().write(|w| w.bits(priority & 0xFF));
    }
}

/// Reads the current interrupt priority mask from CPU's PMR register.
pub fn get_interface_priority_mask() -> u32 {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.pmr().read().bits()
    }
}

/// Configures the group priority and subpriority split point using CPU's BPR register.
/// - `binary_point`: Amount of bits used as subpriority.
pub fn set_binary_point(binary_point: u32) {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.bpr().write(|w| w.bits(binary_point & 7));
    }
}

/// Reads the current group priority and subpriority split point from CPU's BPR register.
pub fn get_binary_point() -> u32 {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.bpr().read().bits()
    }
}

/// Gets the status for a given interrupt.
/// - `irqn`: The interrupt to get status for.
/// - Returns:
///   - 0 - not pending/active
///   - 1 - pending
///   - 2 - active
///   - 3 - pending and active
pub fn get_irq_status(irqn: u32) -> u32 {
    let active = ((isactiver((irqn / 32) as usize)) >> (irqn % 32)) & 1;
    let pending = ((ispendr((irqn / 32) as usize)) >> (irqn % 32)) & 1;

    (active << 1) | pending
}

/// Generates a software interrupt using GIC's SGIR register.
/// - `irqn` -  Software interrupt to be generated.
/// - `target_list`: List of CPUs the software interrupt should be forwarded to.
/// - `filter_list`: Filter to be applied to determine interrupt receivers.
pub fn send_sgi(irqn: u32, target_list: u32, filter_list: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        gicd.sgir().write(|w| {
            w.bits(((filter_list & 0x03) << 24) | ((target_list & 0xFF) << 16) | (irqn & 0x0F))
        });
    }
}

/// Gets the interrupt number of the highest interrupt pending from CPU's HPPIR register.
pub fn get_high_pending_irq() -> u32 {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.hppir().read().bits()
    }
}

/// Provides information about the implementer and revision of the CPU interface.
pub fn get_interface_id() -> u32 {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.iidr().read().bits()
    }
}

/// Sets the interrupt group from the GIC's IGROUPR register.
/// - `irqn`: The interrupt to be queried.
/// - `group`:  Interrupt group number:
///   - 0 - Group 0
///   - 1 - Group 1
pub fn set_group(irqn: u32, group: u32) {
    let mut igroupr = igroupr((irqn / 32) as usize);
    let shift = irqn % 32;

    igroupr &= !(1 << shift);
    igroupr |= (group & 1) << shift;

    set_igroupr((irqn / 32) as usize, igroupr);
}

/// Gets the interrupt group from the GIC's IGROUPR register.
/// - `irqn`:  The interrupt to be queried.
/// - Returns:
///   - 0 - Group 0
///   - 1 - Group 1
pub fn gic_get_group(irqn: u32) -> u32 {
    (igroupr((irqn / 32) as usize) >> (irqn % 32)) & 1
}

/// Initializes the interrupt distributor.
pub fn dist_init() {
    // A reset sets all bits in the IGROUPRs corresponding to the SPIs to 0,
    // configuring all of the interrupts as Secure.

    // Disable interrupt forwarding.
    disable_distributor();

    // Get the maximum number of interrupts that the GIC supports.
    let num_irq = 32 * ((distributor_info() & 0x1F) + 1);

    // Priority level is implementation defined.
    // To determine the number of priority bits implemented write 0xFF to an IPRIORITYR
    // priority field and read back the value stored.
    set_priority(0, 0xFF);
    let priority_field = get_priority(0);

    for i in 32..num_irq {
        // Disable the SPI interrupt.
        disable_irq(i);

        // Set level-sensitive (and N-N model).
        set_configuration(i, 0);

        // Set priority
        set_priority(i, priority_field / 2);

        // Set target list to CPU0
        set_target(i, 1);
    }

    // Enable distributor
    enable_distributor();
}

/// Initializes the CPU's interrupt interface.
pub fn cpu_interface_init() {
    // A reset sets all bits in the IGROUPRs corresponding to the SPIs to 0,
    // configuring all of the interrupts as Secure.

    // Disable interrupt forwarding.
    disable_interface();

    // Priority level is implementation defined.
    // To determine the number of priority bits implemented write 0xFF to an IPRIORITYR
    // priority field and read back the value stored.
    set_priority(0, 0xFF);
    let priority_field = get_priority(0);

    // SGI and PPI.
    for i in 0..32 {
        if i > 15 {
            // Set level-sensitive (and N-N model) for PPI.
            set_configuration(i, 0);
        }

        // Disable SGI and PPI interrupts.
        disable_irq(i);

        // Set priority.
        set_priority(i, priority_field / 2);
    }

    // Enable interface.
    enable_interface();

    // Set binary point to 0.
    set_binary_point(0);

    // Set priority mask.
    set_interface_priority_mask(0xFF);
}

/// Initializes and enable the GIC.
pub fn enable() {
    dist_init();
    cpu_interface_init(); // per CPU
}

/// Reads the ISENABLER register for an index.
fn isenabler(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.isenabler0().read().bits(),
            1 => gicd.isenabler1().read().bits(),
            2 => gicd.isenabler2().read().bits(),
            3 => gicd.isenabler3().read().bits(),
            4 => gicd.isenabler4().read().bits(),
            5 => gicd.isenabler5().read().bits(),
            6 => gicd.isenabler6().read().bits(),
            7 => gicd.isenabler7().read().bits(),
            8 => gicd.isenabler8().read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the ISENABLER register for an index.
fn set_isenabler(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.isenabler0().write(|w| w.bits(value)),
            1 => gicd.isenabler1().write(|w| w.bits(value)),
            2 => gicd.isenabler2().write(|w| w.bits(value)),
            3 => gicd.isenabler3().write(|w| w.bits(value)),
            4 => gicd.isenabler4().write(|w| w.bits(value)),
            5 => gicd.isenabler5().write(|w| w.bits(value)),
            6 => gicd.isenabler6().write(|w| w.bits(value)),
            7 => gicd.isenabler7().write(|w| w.bits(value)),
            8 => gicd.isenabler8().write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        };
    }
}

/// Sets the ICENABLER register for an index.
fn set_icenabler(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.icenabler0().write(|w| w.bits(value)),
            1 => gicd.icenabler1().write(|w| w.bits(value)),
            2 => gicd.icenabler2().write(|w| w.bits(value)),
            3 => gicd.icenabler3().write(|w| w.bits(value)),
            4 => gicd.icenabler4().write(|w| w.bits(value)),
            5 => gicd.icenabler5().write(|w| w.bits(value)),
            6 => gicd.icenabler6().write(|w| w.bits(value)),
            7 => gicd.icenabler7().write(|w| w.bits(value)),
            8 => gicd.icenabler8().write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        };
    }
}

/// Reads the ISPENDR register for an index.
fn ispendr(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.ispendr0().read().bits(),
            1 => gicd.ispendr1().read().bits(),
            2 => gicd.ispendr2().read().bits(),
            3 => gicd.ispendr3().read().bits(),
            4 => gicd.ispendr4().read().bits(),
            5 => gicd.ispendr5().read().bits(),
            6 => gicd.ispendr6().read().bits(),
            7 => gicd.ispendr7().read().bits(),
            8 => gicd.ispendr8().read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the ISPENDR register for an index.
fn set_ispendr(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.ispendr0().write(|w| w.bits(value)),
            1 => gicd.ispendr1().write(|w| w.bits(value)),
            2 => gicd.ispendr2().write(|w| w.bits(value)),
            3 => gicd.ispendr3().write(|w| w.bits(value)),
            4 => gicd.ispendr4().write(|w| w.bits(value)),
            5 => gicd.ispendr5().write(|w| w.bits(value)),
            6 => gicd.ispendr6().write(|w| w.bits(value)),
            7 => gicd.ispendr7().write(|w| w.bits(value)),
            8 => gicd.ispendr8().write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        };
    }
}

/// Sets the ICPENDR register for an index.
fn set_icpendr(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.icpendr0().write(|w| w.bits(value)),
            1 => gicd.icpendr1().write(|w| w.bits(value)),
            2 => gicd.icpendr2().write(|w| w.bits(value)),
            3 => gicd.icpendr3().write(|w| w.bits(value)),
            4 => gicd.icpendr4().write(|w| w.bits(value)),
            5 => gicd.icpendr5().write(|w| w.bits(value)),
            6 => gicd.icpendr6().write(|w| w.bits(value)),
            7 => gicd.icpendr7().write(|w| w.bits(value)),
            8 => gicd.icpendr8().write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        };
    }
}

/// Sets the ICACTIVER register for an index.
fn set_icactiver(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.icactiver0().write(|w| w.bits(value)),
            1 => gicd.icactiver1().write(|w| w.bits(value)),
            2 => gicd.icactiver2().write(|w| w.bits(value)),
            3 => gicd.icactiver3().write(|w| w.bits(value)),
            4 => gicd.icactiver4().write(|w| w.bits(value)),
            5 => gicd.icactiver5().write(|w| w.bits(value)),
            6 => gicd.icactiver6().write(|w| w.bits(value)),
            7 => gicd.icactiver7().write(|w| w.bits(value)),
            8 => gicd.icactiver8().write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        };
    }
}

/// Reads the ISACTIVER register for an index.
fn isactiver(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.isactiver0().read().bits(),
            1 => gicd.isactiver1().read().bits(),
            2 => gicd.isactiver2().read().bits(),
            3 => gicd.isactiver3().read().bits(),
            4 => gicd.isactiver4().read().bits(),
            5 => gicd.isactiver5().read().bits(),
            6 => gicd.isactiver6().read().bits(),
            7 => gicd.isactiver7().read().bits(),
            8 => gicd.isactiver8().read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Reads the SPENDSGIR register for an index.
fn spendsgir(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.spendsgir0().read().bits(),
            1 => gicd.spendsgir1().read().bits(),
            2 => gicd.spendsgir2().read().bits(),
            3 => gicd.spendsgir3().read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the SPENDSGIR register for an index.
fn set_spendsgir(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.spendsgir0().write(|w| w.bits(value)),
            1 => gicd.spendsgir1().write(|w| w.bits(value)),
            2 => gicd.spendsgir2().write(|w| w.bits(value)),
            3 => gicd.spendsgir3().write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        }
    };
}

/// Sets the CPENDSGIR register for an index.
fn set_cpendsgir(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.cpendsgir0().write(|w| w.bits(value)),
            1 => gicd.cpendsgir1().write(|w| w.bits(value)),
            2 => gicd.cpendsgir2().write(|w| w.bits(value)),
            3 => gicd.cpendsgir3().write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        };
    }
}

/// Returns the ICFGR register for an index.
fn icfgr(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.icfgr0().read().bits(),
            1 => gicd.icfgr1().read().bits(),
            2 => gicd.icfgr2().read().bits(),
            3 => gicd.icfgr3().read().bits(),
            4 => gicd.icfgr4().read().bits(),
            5 => gicd.icfgr5().read().bits(),
            6 => gicd.icfgr6().read().bits(),
            7 => gicd.icfgr7().read().bits(),
            8 => gicd.icfgr8().read().bits(),
            9 => gicd.icfgr9().read().bits(),
            10 => gicd.icfgr10().read().bits(),
            11 => gicd.icfgr11().read().bits(),
            12 => gicd.icfgr12().read().bits(),
            13 => gicd.icfgr13().read().bits(),
            14 => gicd.icfgr14().read().bits(),
            15 => gicd.icfgr15().read().bits(),
            16 => gicd.icfgr16().read().bits(),
            17 => gicd.icfgr17().read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the ICFGR register for an index.
fn set_icfgr(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.icfgr0().write(|w| w.bits(value)),
            1 => gicd.icfgr1().write(|w| w.bits(value)),
            2 => gicd.icfgr2().write(|w| w.bits(value)),
            3 => gicd.icfgr3().write(|w| w.bits(value)),
            4 => gicd.icfgr4().write(|w| w.bits(value)),
            5 => gicd.icfgr5().write(|w| w.bits(value)),
            6 => gicd.icfgr6().write(|w| w.bits(value)),
            7 => gicd.icfgr7().write(|w| w.bits(value)),
            8 => gicd.icfgr8().write(|w| w.bits(value)),
            9 => gicd.icfgr9().write(|w| w.bits(value)),
            10 => gicd.icfgr10().write(|w| w.bits(value)),
            11 => gicd.icfgr11().write(|w| w.bits(value)),
            12 => gicd.icfgr12().write(|w| w.bits(value)),
            13 => gicd.icfgr13().write(|w| w.bits(value)),
            14 => gicd.icfgr13().write(|w| w.bits(value)),
            15 => gicd.icfgr15().write(|w| w.bits(value)),
            16 => gicd.icfgr16().write(|w| w.bits(value)),
            17 => gicd.icfgr17().write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        };
    }
}

/// Reads the ITARGETSR register for an index.
fn itargetsr(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.itargetsr0().read().bits(),
            1 => gicd.itargetsr1().read().bits(),
            2 => gicd.itargetsr2().read().bits(),
            3 => gicd.itargetsr3().read().bits(),
            4 => gicd.itargetsr4().read().bits(),
            5 => gicd.itargetsr5().read().bits(),
            6 => gicd.itargetsr6().read().bits(),
            7 => gicd.itargetsr7().read().bits(),
            8 => gicd.itargetsr8().read().bits(),
            9 => gicd.itargetsr9().read().bits(),
            10 => gicd.itargetsr10().read().bits(),
            11 => gicd.itargetsr11().read().bits(),
            12 => gicd.itargetsr12().read().bits(),
            13 => gicd.itargetsr13().read().bits(),
            14 => gicd.itargetsr14().read().bits(),
            15 => gicd.itargetsr15().read().bits(),
            16 => gicd.itargetsr16().read().bits(),
            17 => gicd.itargetsr17().read().bits(),
            18 => gicd.itargetsr18().read().bits(),
            19 => gicd.itargetsr19().read().bits(),
            20 => gicd.itargetsr20().read().bits(),
            21 => gicd.itargetsr21().read().bits(),
            22 => gicd.itargetsr22().read().bits(),
            23 => gicd.itargetsr23().read().bits(),
            24 => gicd.itargetsr24().read().bits(),
            25 => gicd.itargetsr25().read().bits(),
            26 => gicd.itargetsr26().read().bits(),
            27 => gicd.itargetsr27().read().bits(),
            28 => gicd.itargetsr28().read().bits(),
            29 => gicd.itargetsr29().read().bits(),
            30 => gicd.itargetsr30().read().bits(),
            31 => gicd.itargetsr31().read().bits(),
            32 => gicd.itargetsr32().read().bits(),
            33 => gicd.itargetsr33().read().bits(),
            34 => gicd.itargetsr34().read().bits(),
            35 => gicd.itargetsr35().read().bits(),
            36 => gicd.itargetsr36().read().bits(),
            37 => gicd.itargetsr37().read().bits(),
            38 => gicd.itargetsr38().read().bits(),
            39 => gicd.itargetsr39().read().bits(),
            40 => gicd.itargetsr40().read().bits(),
            41 => gicd.itargetsr41().read().bits(),
            42 => gicd.itargetsr42().read().bits(),
            43 => gicd.itargetsr43().read().bits(),
            44 => gicd.itargetsr44().read().bits(),
            45 => gicd.itargetsr45().read().bits(),
            46 => gicd.itargetsr46().read().bits(),
            47 => gicd.itargetsr47().read().bits(),
            48 => gicd.itargetsr48().read().bits(),
            49 => gicd.itargetsr49().read().bits(),
            50 => gicd.itargetsr50().read().bits(),
            51 => gicd.itargetsr51().read().bits(),
            52 => gicd.itargetsr52().read().bits(),
            53 => gicd.itargetsr53().read().bits(),
            54 => gicd.itargetsr54().read().bits(),
            55 => gicd.itargetsr55().read().bits(),
            56 => gicd.itargetsr56().read().bits(),
            57 => gicd.itargetsr57().read().bits(),
            58 => gicd.itargetsr58().read().bits(),
            59 => gicd.itargetsr59().read().bits(),
            60 => gicd.itargetsr60().read().bits(),
            61 => gicd.itargetsr61().read().bits(),
            62 => gicd.itargetsr62().read().bits(),
            63 => gicd.itargetsr63().read().bits(),
            64 => gicd.itargetsr64().read().bits(),
            65 => gicd.itargetsr65().read().bits(),
            66 => gicd.itargetsr66().read().bits(),
            67 => gicd.itargetsr67().read().bits(),
            68 => gicd.itargetsr68().read().bits(),
            69 => gicd.itargetsr69().read().bits(),
            70 => gicd.itargetsr70().read().bits(),
            71 => gicd.itargetsr71().read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the ITARGETSR register for an index.
fn set_itargetsr(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            // Indexes 0..7 are read-only.
            8 => gicd.itargetsr8().write(|w| w.bits(value)),
            9 => gicd.itargetsr9().write(|w| w.bits(value)),
            10 => gicd.itargetsr10().write(|w| w.bits(value)),
            11 => gicd.itargetsr11().write(|w| w.bits(value)),
            12 => gicd.itargetsr12().write(|w| w.bits(value)),
            13 => gicd.itargetsr13().write(|w| w.bits(value)),
            14 => gicd.itargetsr14().write(|w| w.bits(value)),
            15 => gicd.itargetsr15().write(|w| w.bits(value)),
            16 => gicd.itargetsr16().write(|w| w.bits(value)),
            17 => gicd.itargetsr17().write(|w| w.bits(value)),
            18 => gicd.itargetsr18().write(|w| w.bits(value)),
            19 => gicd.itargetsr19().write(|w| w.bits(value)),
            20 => gicd.itargetsr20().write(|w| w.bits(value)),
            21 => gicd.itargetsr21().write(|w| w.bits(value)),
            22 => gicd.itargetsr22().write(|w| w.bits(value)),
            23 => gicd.itargetsr23().write(|w| w.bits(value)),
            24 => gicd.itargetsr24().write(|w| w.bits(value)),
            25 => gicd.itargetsr25().write(|w| w.bits(value)),
            26 => gicd.itargetsr26().write(|w| w.bits(value)),
            27 => gicd.itargetsr27().write(|w| w.bits(value)),
            28 => gicd.itargetsr28().write(|w| w.bits(value)),
            29 => gicd.itargetsr29().write(|w| w.bits(value)),
            30 => gicd.itargetsr30().write(|w| w.bits(value)),
            31 => gicd.itargetsr31().write(|w| w.bits(value)),
            32 => gicd.itargetsr32().write(|w| w.bits(value)),
            33 => gicd.itargetsr33().write(|w| w.bits(value)),
            34 => gicd.itargetsr34().write(|w| w.bits(value)),
            35 => gicd.itargetsr35().write(|w| w.bits(value)),
            36 => gicd.itargetsr36().write(|w| w.bits(value)),
            37 => gicd.itargetsr37().write(|w| w.bits(value)),
            38 => gicd.itargetsr38().write(|w| w.bits(value)),
            39 => gicd.itargetsr39().write(|w| w.bits(value)),
            40 => gicd.itargetsr40().write(|w| w.bits(value)),
            41 => gicd.itargetsr41().write(|w| w.bits(value)),
            42 => gicd.itargetsr42().write(|w| w.bits(value)),
            43 => gicd.itargetsr43().write(|w| w.bits(value)),
            44 => gicd.itargetsr44().write(|w| w.bits(value)),
            45 => gicd.itargetsr45().write(|w| w.bits(value)),
            46 => gicd.itargetsr46().write(|w| w.bits(value)),
            47 => gicd.itargetsr47().write(|w| w.bits(value)),
            48 => gicd.itargetsr48().write(|w| w.bits(value)),
            49 => gicd.itargetsr49().write(|w| w.bits(value)),
            50 => gicd.itargetsr50().write(|w| w.bits(value)),
            51 => gicd.itargetsr51().write(|w| w.bits(value)),
            52 => gicd.itargetsr52().write(|w| w.bits(value)),
            53 => gicd.itargetsr53().write(|w| w.bits(value)),
            54 => gicd.itargetsr54().write(|w| w.bits(value)),
            55 => gicd.itargetsr55().write(|w| w.bits(value)),
            56 => gicd.itargetsr56().write(|w| w.bits(value)),
            57 => gicd.itargetsr57().write(|w| w.bits(value)),
            58 => gicd.itargetsr58().write(|w| w.bits(value)),
            59 => gicd.itargetsr59().write(|w| w.bits(value)),
            60 => gicd.itargetsr60().write(|w| w.bits(value)),
            61 => gicd.itargetsr61().write(|w| w.bits(value)),
            62 => gicd.itargetsr62().write(|w| w.bits(value)),
            63 => gicd.itargetsr63().write(|w| w.bits(value)),
            64 => gicd.itargetsr64().write(|w| w.bits(value)),
            65 => gicd.itargetsr65().write(|w| w.bits(value)),
            66 => gicd.itargetsr66().write(|w| w.bits(value)),
            67 => gicd.itargetsr67().write(|w| w.bits(value)),
            68 => gicd.itargetsr68().write(|w| w.bits(value)),
            69 => gicd.itargetsr69().write(|w| w.bits(value)),
            70 => gicd.itargetsr70().write(|w| w.bits(value)),
            71 => gicd.itargetsr71().write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        };
    }
}

/// Reads the IPRIORITYR register for an index.
fn ipriorityr(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.ipriorityr0().read().bits(),
            1 => gicd.ipriorityr1().read().bits(),
            2 => gicd.ipriorityr2().read().bits(),
            3 => gicd.ipriorityr3().read().bits(),
            4 => gicd.ipriorityr4().read().bits(),
            5 => gicd.ipriorityr5().read().bits(),
            6 => gicd.ipriorityr6().read().bits(),
            7 => gicd.ipriorityr7().read().bits(),
            8 => gicd.ipriorityr8().read().bits(),
            9 => gicd.ipriorityr9().read().bits(),
            10 => gicd.ipriorityr10().read().bits(),
            11 => gicd.ipriorityr11().read().bits(),
            12 => gicd.ipriorityr12().read().bits(),
            13 => gicd.ipriorityr13().read().bits(),
            14 => gicd.ipriorityr14().read().bits(),
            15 => gicd.ipriorityr15().read().bits(),
            16 => gicd.ipriorityr16().read().bits(),
            17 => gicd.ipriorityr17().read().bits(),
            18 => gicd.ipriorityr18().read().bits(),
            19 => gicd.ipriorityr19().read().bits(),
            20 => gicd.ipriorityr20().read().bits(),
            21 => gicd.ipriorityr21().read().bits(),
            22 => gicd.ipriorityr22().read().bits(),
            23 => gicd.ipriorityr23().read().bits(),
            24 => gicd.ipriorityr24().read().bits(),
            25 => gicd.ipriorityr25().read().bits(),
            26 => gicd.ipriorityr26().read().bits(),
            27 => gicd.ipriorityr27().read().bits(),
            28 => gicd.ipriorityr28().read().bits(),
            29 => gicd.ipriorityr29().read().bits(),
            30 => gicd.ipriorityr30().read().bits(),
            31 => gicd.ipriorityr31().read().bits(),
            32 => gicd.ipriorityr32().read().bits(),
            33 => gicd.ipriorityr33().read().bits(),
            34 => gicd.ipriorityr34().read().bits(),
            35 => gicd.ipriorityr35().read().bits(),
            36 => gicd.ipriorityr36().read().bits(),
            37 => gicd.ipriorityr37().read().bits(),
            38 => gicd.ipriorityr38().read().bits(),
            39 => gicd.ipriorityr39().read().bits(),
            40 => gicd.ipriorityr40().read().bits(),
            41 => gicd.ipriorityr41().read().bits(),
            42 => gicd.ipriorityr42().read().bits(),
            43 => gicd.ipriorityr43().read().bits(),
            44 => gicd.ipriorityr44().read().bits(),
            45 => gicd.ipriorityr45().read().bits(),
            46 => gicd.ipriorityr46().read().bits(),
            47 => gicd.ipriorityr47().read().bits(),
            48 => gicd.ipriorityr48().read().bits(),
            49 => gicd.ipriorityr49().read().bits(),
            50 => gicd.ipriorityr50().read().bits(),
            51 => gicd.ipriorityr51().read().bits(),
            52 => gicd.ipriorityr52().read().bits(),
            53 => gicd.ipriorityr53().read().bits(),
            54 => gicd.ipriorityr54().read().bits(),
            55 => gicd.ipriorityr55().read().bits(),
            56 => gicd.ipriorityr56().read().bits(),
            57 => gicd.ipriorityr57().read().bits(),
            58 => gicd.ipriorityr58().read().bits(),
            59 => gicd.ipriorityr59().read().bits(),
            60 => gicd.ipriorityr60().read().bits(),
            61 => gicd.ipriorityr61().read().bits(),
            62 => gicd.ipriorityr62().read().bits(),
            63 => gicd.ipriorityr63().read().bits(),
            64 => gicd.ipriorityr64().read().bits(),
            65 => gicd.ipriorityr65().read().bits(),
            66 => gicd.ipriorityr66().read().bits(),
            67 => gicd.ipriorityr67().read().bits(),
            68 => gicd.ipriorityr68().read().bits(),
            69 => gicd.ipriorityr69().read().bits(),
            70 => gicd.ipriorityr70().read().bits(),
            71 => gicd.ipriorityr71().read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the IPRIORITYR register for an index.
fn set_ipriorityr(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.ipriorityr0().write(|w| w.bits(value)),
            1 => gicd.ipriorityr1().write(|w| w.bits(value)),
            2 => gicd.ipriorityr2().write(|w| w.bits(value)),
            3 => gicd.ipriorityr3().write(|w| w.bits(value)),
            4 => gicd.ipriorityr4().write(|w| w.bits(value)),
            5 => gicd.ipriorityr5().write(|w| w.bits(value)),
            6 => gicd.ipriorityr6().write(|w| w.bits(value)),
            7 => gicd.ipriorityr7().write(|w| w.bits(value)),
            8 => gicd.ipriorityr8().write(|w| w.bits(value)),
            9 => gicd.ipriorityr9().write(|w| w.bits(value)),
            10 => gicd.ipriorityr10().write(|w| w.bits(value)),
            11 => gicd.ipriorityr11().write(|w| w.bits(value)),
            12 => gicd.ipriorityr12().write(|w| w.bits(value)),
            13 => gicd.ipriorityr13().write(|w| w.bits(value)),
            14 => gicd.ipriorityr14().write(|w| w.bits(value)),
            15 => gicd.ipriorityr15().write(|w| w.bits(value)),
            16 => gicd.ipriorityr16().write(|w| w.bits(value)),
            17 => gicd.ipriorityr17().write(|w| w.bits(value)),
            18 => gicd.ipriorityr18().write(|w| w.bits(value)),
            19 => gicd.ipriorityr19().write(|w| w.bits(value)),
            20 => gicd.ipriorityr20().write(|w| w.bits(value)),
            21 => gicd.ipriorityr21().write(|w| w.bits(value)),
            22 => gicd.ipriorityr22().write(|w| w.bits(value)),
            23 => gicd.ipriorityr23().write(|w| w.bits(value)),
            24 => gicd.ipriorityr24().write(|w| w.bits(value)),
            25 => gicd.ipriorityr25().write(|w| w.bits(value)),
            26 => gicd.ipriorityr26().write(|w| w.bits(value)),
            27 => gicd.ipriorityr27().write(|w| w.bits(value)),
            28 => gicd.ipriorityr28().write(|w| w.bits(value)),
            29 => gicd.ipriorityr29().write(|w| w.bits(value)),
            30 => gicd.ipriorityr30().write(|w| w.bits(value)),
            31 => gicd.ipriorityr31().write(|w| w.bits(value)),
            32 => gicd.ipriorityr32().write(|w| w.bits(value)),
            33 => gicd.ipriorityr33().write(|w| w.bits(value)),
            34 => gicd.ipriorityr34().write(|w| w.bits(value)),
            35 => gicd.ipriorityr35().write(|w| w.bits(value)),
            36 => gicd.ipriorityr36().write(|w| w.bits(value)),
            37 => gicd.ipriorityr37().write(|w| w.bits(value)),
            38 => gicd.ipriorityr38().write(|w| w.bits(value)),
            39 => gicd.ipriorityr39().write(|w| w.bits(value)),
            40 => gicd.ipriorityr40().write(|w| w.bits(value)),
            41 => gicd.ipriorityr41().write(|w| w.bits(value)),
            42 => gicd.ipriorityr42().write(|w| w.bits(value)),
            43 => gicd.ipriorityr43().write(|w| w.bits(value)),
            44 => gicd.ipriorityr44().write(|w| w.bits(value)),
            45 => gicd.ipriorityr45().write(|w| w.bits(value)),
            46 => gicd.ipriorityr46().write(|w| w.bits(value)),
            47 => gicd.ipriorityr47().write(|w| w.bits(value)),
            48 => gicd.ipriorityr48().write(|w| w.bits(value)),
            49 => gicd.ipriorityr49().write(|w| w.bits(value)),
            50 => gicd.ipriorityr50().write(|w| w.bits(value)),
            51 => gicd.ipriorityr51().write(|w| w.bits(value)),
            52 => gicd.ipriorityr52().write(|w| w.bits(value)),
            53 => gicd.ipriorityr53().write(|w| w.bits(value)),
            54 => gicd.ipriorityr54().write(|w| w.bits(value)),
            55 => gicd.ipriorityr55().write(|w| w.bits(value)),
            56 => gicd.ipriorityr56().write(|w| w.bits(value)),
            57 => gicd.ipriorityr57().write(|w| w.bits(value)),
            58 => gicd.ipriorityr58().write(|w| w.bits(value)),
            59 => gicd.ipriorityr59().write(|w| w.bits(value)),
            60 => gicd.ipriorityr60().write(|w| w.bits(value)),
            61 => gicd.ipriorityr61().write(|w| w.bits(value)),
            62 => gicd.ipriorityr62().write(|w| w.bits(value)),
            63 => gicd.ipriorityr63().write(|w| w.bits(value)),
            64 => gicd.ipriorityr64().write(|w| w.bits(value)),
            65 => gicd.ipriorityr65().write(|w| w.bits(value)),
            66 => gicd.ipriorityr66().write(|w| w.bits(value)),
            67 => gicd.ipriorityr67().write(|w| w.bits(value)),
            68 => gicd.ipriorityr68().write(|w| w.bits(value)),
            69 => gicd.ipriorityr69().write(|w| w.bits(value)),
            70 => gicd.ipriorityr70().write(|w| w.bits(value)),
            71 => gicd.ipriorityr71().write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        };
    }
}

/// Returns the IGROUPR register for an index.
fn igroupr(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.igroupr0().read().bits(),
            1 => gicd.igroupr1().read().bits(),
            2 => gicd.igroupr2().read().bits(),
            3 => gicd.igroupr3().read().bits(),
            4 => gicd.igroupr4().read().bits(),
            5 => gicd.igroupr5().read().bits(),
            6 => gicd.igroupr6().read().bits(),
            7 => gicd.igroupr7().read().bits(),
            8 => gicd.igroupr8().read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the IGROUPR register for an index.
fn set_igroupr(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.igroupr0().write(|w| w.bits(value)),
            1 => gicd.igroupr1().write(|w| w.bits(value)),
            2 => gicd.igroupr2().write(|w| w.bits(value)),
            3 => gicd.igroupr3().write(|w| w.bits(value)),
            4 => gicd.igroupr4().write(|w| w.bits(value)),
            5 => gicd.igroupr5().write(|w| w.bits(value)),
            6 => gicd.igroupr6().write(|w| w.bits(value)),
            7 => gicd.igroupr7().write(|w| w.bits(value)),
            8 => gicd.igroupr8().write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        };
    }
}

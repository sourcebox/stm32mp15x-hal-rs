#![doc=include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]

pub mod memory;
pub mod regs;

mod critical_section_impl;

use core::arch::asm;

/// Returns the 0-based core number.
pub fn core_id() -> u32 {
    regs::mpidr() & 0xFF
}

/// Enables the FPU and NEON if enabled by feature.
pub fn enable_fpu() {
    unsafe {
        asm! {
            // Permit access to VFP, registers by modifying CPACR.
            "mrc     p15, 0, r1, c1, c0, 2",
            "orr     r1, r1, #0x00F00000",
            "mcr     p15, 0, r1, c1, c0, 2",

            // Ensure that subsequent instructions occur in the context of VFP access permitted
            "isb",

            // Enable VFP.
            "vmrs    r1, fpexc",
            "orr     r1, r1, #0x40000000",
            "vmsr    fpexc, r1",

            // Initialise D16 registers to 0.
            "mov     r2, #0",
            "vmov    d0, r2, r2",
            "vmov    d1, r2, r2",
            "vmov    d2, r2, r2",
            "vmov    d3, r2, r2",
            "vmov    d4, r2, r2",
            "vmov    d5, r2, r2",
            "vmov    d6, r2, r2",
            "vmov    d7, r2, r2",
            "vmov    d8, r2, r2",
            "vmov    d9, r2, r2",
            "vmov    d10, r2, r2",
            "vmov    d11, r2, r2",
            "vmov    d12, r2, r2",
            "vmov    d13, r2, r2",
            "vmov    d14, r2, r2",
            "vmov    d15, r2, r2",

            // Initialise FPSCR to a known state.
            // Mask off all bits that do not have to be preserved.
            // Non-preserved bits can/should be zero.
            "vmrs    r2, fpscr",
            "movw    r3, #6060",
            "movt    r3, #8",
            "and     r2, r2, r3",
            "vmsr    fpscr, r2",
        }
    }

    #[cfg(feature = "neon")]
    enable_neon();
}

/// Enables the NEON extension. Called by `enable_fpu` if `neon` feature is enabled.
#[cfg(feature = "neon")]
fn enable_neon() {
    unsafe {
        asm! {
            // Set bits [11:10] of the NSACR for access to CP10 and CP11 from both
            // secure and non-secure states, and clear the NSASEDIS and NSD32DIS bits.
            "mrc    p15, 0, r0, c1, c1, 2",
            "orr    r0, r0, #0x0C00", // Enable NEON.
            "bic    r0, r0, #0xC000", // Clear NSASEDIS/NSD32DIS.
            "mcr    p15, 0, r0, c1, c1, 2",

            "isb",

            // Initialise NEON registers to 0.
            "mov     r2, #0",
            "vmov    d16, r2, r2",
            "vmov    d17, r2, r2",
            "vmov    d18, r2, r2",
            "vmov    d19, r2, r2",
            "vmov    d20, r2, r2",
            "vmov    d21, r2, r2",
            "vmov    d22, r2, r2",
            "vmov    d23, r2, r2",
            "vmov    d24, r2, r2",
            "vmov    d25, r2, r2",
            "vmov    d26, r2, r2",
            "vmov    d27, r2, r2",
            "vmov    d28, r2, r2",
            "vmov    d29, r2, r2",
            "vmov    d30, r2, r2",
            "vmov    d31, r2, r2",
        }
    }
}

/// Enables the Snoop Control Unit (SCU).
pub fn enable_scu() {
    let cbar = regs::cbar();
    let scu_ctrl = cbar;

    unsafe {
        let value = core::ptr::read_volatile(scu_ctrl as *const u32);
        core::ptr::write_volatile(scu_ctrl as *mut u32, value | 1);
    }
}

//! Critical section implementation.

#![allow(asm_sub_register)]

/// Implementation for single core.
mod cs_single {
    use core::sync::atomic::{AtomicU32, Ordering};
    use critical_section::{set_impl, Impl, RawRestoreState};

    /// Recursion counter. Used to make the critical section reentrant.
    static RECURSION_COUNT: AtomicU32 = AtomicU32::new(0);

    struct SingleCoreCriticalSection;

    set_impl!(SingleCoreCriticalSection);

    unsafe impl Impl for SingleCoreCriticalSection {
        unsafe fn acquire() -> RawRestoreState {
            let mut cpsr_old: u32;
            core::arch::asm!("mrs {}, cpsr", out(reg) cpsr_old);
            core::arch::asm!("cpsid i");

            RECURSION_COUNT.fetch_add(1, Ordering::Relaxed);

            cpsr_old
        }

        unsafe fn release(cpsr_old: RawRestoreState) {
            if RECURSION_COUNT.fetch_sub(1, Ordering::Relaxed) > 1 {
                return;
            }

            if cpsr_old & 0x80 == 0 {
                core::arch::asm!("cpsie i");
            }
        }
    }
}

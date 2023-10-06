//! Critical section implementations.

#[cfg(feature = "cs-single-core")]
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

#[cfg(feature = "cs-multi-core")]
/// Implementation for multi core using a spinlock.
mod cs_multi {
    use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
    use critical_section::{set_impl, Impl, RawRestoreState};

    /// Recursion counter. Used to make the critical section reentrant.
    static RECURSION_COUNT: AtomicU32 = AtomicU32::new(0);

    struct MultiCoreCriticalSection;

    set_impl!(MultiCoreCriticalSection);

    static LOCK: AtomicBool = AtomicBool::new(false);

    unsafe impl Impl for MultiCoreCriticalSection {
        unsafe fn acquire() -> RawRestoreState {
            let mut cpsr_old: u32;
            core::arch::asm!("mrs {}, cpsr", out(reg) cpsr_old);
            core::arch::asm!("cpsid i");

            core::sync::atomic::compiler_fence(Ordering::SeqCst);

            while match LOCK.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire) {
                Ok(v) => v,
                Err(_) => true,
            } {}

            RECURSION_COUNT.fetch_add(1, Ordering::Relaxed);

            cpsr_old
        }

        unsafe fn release(cpsr_old: RawRestoreState) {
            if RECURSION_COUNT.fetch_sub(1, Ordering::Relaxed) > 1 {
                return;
            }

            LOCK.store(false, Ordering::Release);

            core::sync::atomic::compiler_fence(Ordering::SeqCst);

            if cpsr_old & 0x80 == 0 {
                core::arch::asm!("cpsie i");
            }
        }
    }
}

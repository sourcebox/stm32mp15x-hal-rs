//! Critical section implementation using hardware semaphore (HSEM) 31.
//!
//! **Important:** the `init` function must be called before any use of the critical section
//! to enable the peripheral in the RCC.

#![allow(asm_sub_register)]

use core::sync::atomic::{AtomicU32, Ordering};

use critical_section::{set_impl, Impl, RawRestoreState};

use crate::pac;

/// Core id, identical to the cpu id.
const CORE_ID: u8 = crate::CPU_ID as u8;

/// Initialize the hardware semaphores by enabling the peripheral clocks in the RCC.
pub fn init() {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.mp_ahb3ensetr().modify(|_, w| w.hsemen().set_bit());
    }
}

/// Reentry counter.
static REENTRY_COUNT: AtomicU32 = AtomicU32::new(0);

/// The critital section itself.
struct MultiCoreCriticalSection;

set_impl!(MultiCoreCriticalSection);

unsafe impl Impl for MultiCoreCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let mut cpsr_old: u32;
        core::arch::asm!("mrs {}, cpsr", out(reg) cpsr_old);
        core::arch::asm!("cpsid i");

        core::sync::atomic::compiler_fence(Ordering::SeqCst);

        let proc_id = ((crate::core_id() & 0x3) + 1) as u8;
        let hsem = &(*pac::HSEM::ptr());

        loop {
            hsem.r31().write(|w| {
                w.coreid()
                    .bits(CORE_ID)
                    .procid()
                    .bits(proc_id)
                    .lock()
                    .set_bit()
            });

            let r = hsem.r31().read();

            if r.coreid().bits() == CORE_ID && r.procid().bits() == proc_id && r.lock().bit_is_set()
            {
                break;
            }
        }

        REENTRY_COUNT.fetch_add(1, Ordering::SeqCst);

        cpsr_old
    }

    unsafe fn release(cpsr_old: RawRestoreState) {
        if REENTRY_COUNT.fetch_sub(1, Ordering::SeqCst) > 1 {
            return;
        }

        let proc_id = ((crate::core_id() & 0x3) + 1) as u8;
        let hsem = &(*pac::HSEM::ptr());

        loop {
            hsem.r31().write(|w| {
                w.coreid()
                    .bits(CORE_ID)
                    .procid()
                    .bits(proc_id)
                    .lock()
                    .clear_bit()
            });

            if hsem.r31().read().lock().bit_is_clear() {
                break;
            }
        }

        core::sync::atomic::compiler_fence(Ordering::SeqCst);

        if cpsr_old & 0x80 == 0 {
            core::arch::asm!("cpsie i");
        }
    }
}

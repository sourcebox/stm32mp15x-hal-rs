//! Benchmark tests for numeric performance.
//!
//! Ported from <https://esp32.com/viewtopic.php?p=82090#p82090>

#![no_std]
#![no_main]

mod common;

use stm32mp15x_hal as hal;

use hal::HalConfig;

use common::{logger, memory_region_mapper};

/// Entry point for MPU0.
#[no_mangle]
pub extern "C" fn main() -> ! {
    let hal_config = HalConfig {
        memory_region_mapper,
    };
    hal::init(hal_config);

    logger::init();

    bench::run();

    loop {}
}

/// Entry point for MPU1.
#[no_mangle]
pub extern "C" fn mpu1_main() -> ! {
    loop {}
}

mod bench {
    use crate::hal::time;

    pub fn run() {
        i32_add();
        i32_mul();
        f32_add();
        f32_mul();
        f32_mul_add();
        f32_div();
    }

    fn i32_add() {
        const N: usize = 3200000;
        static mut TV: [i32; 8] = [2, 3, 4, 5, 6, 7, 8, 9];

        unsafe {
            let mut f0 = TV[0];
            let mut f1 = TV[1];
            let mut f2 = TV[2];
            let mut f3 = TV[3];
            let mut f4 = TV[4];
            let mut f5 = TV[5];
            let mut f6 = TV[6];
            let mut f7 = TV[7];

            let time = time::micros();

            for _ in 0..N / 8 {
                f0 = f1 + f2;
                f1 = f2 + f3;
                f2 = f3 + f4;
                f3 = f4 + f5;
                f4 = f5 + f6;
                f5 = f6 + f7;
                f6 = f7 + f0;
                f7 = f0 + f1;
            }

            let time = time::micros() - time;

            TV[0] = f0;
            TV[1] = f1;
            TV[2] = f2;
            TV[3] = f3;
            TV[4] = f4;
            TV[5] = f5;
            TV[6] = f6;
            TV[7] = f7;

            log::info!("i32 add: {}", N as f32 / time as f32);
        }
    }

    fn i32_mul() {
        const N: usize = 3200000;
        static mut TV: [i32; 8] = [2, 3, 4, 5, 6, 7, 8, 9];

        unsafe {
            let mut f0 = TV[0];
            let mut f1 = TV[1];
            let mut f2 = TV[2];
            let mut f3 = TV[3];
            let mut f4 = TV[4];
            let mut f5 = TV[5];
            let mut f6 = TV[6];
            let mut f7 = TV[7];

            let time = time::micros();

            for _ in 0..N / 8 {
                f0 = f1 * f2;
                f1 = f2 * f3;
                f2 = f3 * f4;
                f3 = f4 * f5;
                f4 = f5 * f6;
                f5 = f6 * f7;
                f6 = f7 * f0;
                f7 = f0 * f1;
            }

            let time = time::micros() - time;

            TV[0] = f0;
            TV[1] = f1;
            TV[2] = f2;
            TV[3] = f3;
            TV[4] = f4;
            TV[5] = f5;
            TV[6] = f6;
            TV[7] = f7;

            log::info!("i32 mul: {}", N as f32 / time as f32);
        }
    }

    fn f32_add() {
        const N: usize = 3200000;
        static mut TV: [f32; 8] = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

        unsafe {
            let mut f0 = TV[0];
            let mut f1 = TV[1];
            let mut f2 = TV[2];
            let mut f3 = TV[3];
            let mut f4 = TV[4];
            let mut f5 = TV[5];
            let mut f6 = TV[6];
            let mut f7 = TV[7];

            let time = time::micros();

            for _ in 0..N / 8 {
                f0 = f1 + f2;
                f1 = f2 + f3;
                f2 = f3 + f4;
                f3 = f4 + f5;
                f4 = f5 + f6;
                f5 = f6 + f7;
                f6 = f7 + f0;
                f7 = f0 + f1;
            }

            let time = time::micros() - time;

            TV[0] = f0;
            TV[1] = f1;
            TV[2] = f2;
            TV[3] = f3;
            TV[4] = f4;
            TV[5] = f5;
            TV[6] = f6;
            TV[7] = f7;

            log::info!("f32 add: {}", N as f32 / time as f32);
        }
    }

    fn f32_mul() {
        const N: usize = 3200000;
        static mut TV: [f32; 8] = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

        unsafe {
            let mut f0 = TV[0];
            let mut f1 = TV[1];
            let mut f2 = TV[2];
            let mut f3 = TV[3];
            let mut f4 = TV[4];
            let mut f5 = TV[5];
            let mut f6 = TV[6];
            let mut f7 = TV[7];

            let time = time::micros();

            for _ in 0..N / 8 {
                f0 = f1 * f2;
                f1 = f2 * f3;
                f2 = f3 * f4;
                f3 = f4 * f5;
                f4 = f5 * f6;
                f5 = f6 * f7;
                f6 = f7 * f0;
                f7 = f0 * f1;
            }

            let time = time::micros() - time;

            TV[0] = f0;
            TV[1] = f1;
            TV[2] = f2;
            TV[3] = f3;
            TV[4] = f4;
            TV[5] = f5;
            TV[6] = f6;
            TV[7] = f7;

            log::info!("f32 mul: {}", N as f32 / time as f32);
        }
    }

    fn f32_div() {
        const N: usize = 3200000;
        static mut TV: [f32; 8] = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

        unsafe {
            let mut f0 = TV[0];
            let mut f1 = TV[1];
            let mut f2 = TV[2];
            let mut f3 = TV[3];
            let mut f4 = TV[4];
            let mut f5 = TV[5];
            let mut f6 = TV[6];
            let mut f7 = TV[7];

            let time = time::micros();

            for _ in 0..N / 8 {
                f0 = f1 / f2;
                f1 = f2 / f3;
                f2 = f3 / f4;
                f3 = f4 / f5;
                f4 = f5 / f6;
                f5 = f6 / f7;
                f6 = f7 / f0;
                f7 = f0 / f1;
            }

            let time = time::micros() - time;

            TV[0] = f0;
            TV[1] = f1;
            TV[2] = f2;
            TV[3] = f3;
            TV[4] = f4;
            TV[5] = f5;
            TV[6] = f6;
            TV[7] = f7;

            log::info!("f32 div: {}", N as f32 / time as f32);
        }
    }

    fn f32_mul_add() {
        const N: usize = 3200000;
        static mut TV: [f32; 8] = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

        unsafe {
            let mut f0 = TV[0];
            let mut f1 = TV[1];
            let mut f2 = TV[2];
            let mut f3 = TV[3];
            let mut f4 = TV[4];
            let mut f5 = TV[5];
            let mut f6 = TV[6];
            let mut f7 = TV[7];

            let time = time::micros();

            for _ in 0..N / 8 {
                f0 += f1 * f2;
                f1 += f2 * f3;
                f2 += f3 * f4;
                f3 += f4 * f5;
                f4 += f5 * f6;
                f5 += f6 * f7;
                f6 += f7 * f0;
                f7 += f0 * f1;
            }

            let time = time::micros() - time;

            TV[0] = f0;
            TV[1] = f1;
            TV[2] = f2;
            TV[3] = f3;
            TV[4] = f4;
            TV[5] = f5;
            TV[6] = f6;
            TV[7] = f7;

            log::info!("f32 mul_add: {}", N as f32 / time as f32);
        }
    }
}

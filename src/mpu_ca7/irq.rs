//! Interrupts.

use core::arch::asm;

use int_enum::IntEnum;

use crate::gic;
use crate::pac;

/// User interrupt handler type. Takes the irq number as parameter.
pub type IrqHandler = fn(Irqn);

/// User IRQ handler function.
static mut IRQ_HANDLER: Option<IrqHandler> = None;

/// IRQ numbers.
#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, Clone, Copy, IntEnum)]
pub enum Irqn {
    // Cortex-A Processor Specific Interrupt Numbers.
    // Software Generated Interrupts.
    /// Software Generated Interrupt  0.
    SGI0 = 0,
    /// Software Generated Interrupt  1.
    SGI1 = 1,
    /// Software Generated Interrupt  2.
    SGI2 = 2,
    /// Software Generated Interrupt  3.
    SGI3 = 3,
    /// Software Generated Interrupt  4.
    SGI4 = 4,
    /// Software Generated Interrupt  5.
    SGI5 = 5,
    /// Software Generated Interrupt  6.
    SGI6 = 6,
    /// Software Generated Interrupt  7.
    SGI7 = 7,
    /// Software Generated Interrupt  8.
    SGI8 = 8,
    /// Software Generated Interrupt  9.
    SGI9 = 9,
    /// Software Generated Interrupt 10.
    SGI10 = 10,
    /// Software Generated Interrupt 11.
    SGI11 = 11,
    /// Software Generated Interrupt 12.
    SGI12 = 12,
    /// Software Generated Interrupt 13.
    SGI13 = 13,
    /// Software Generated Interrupt 14.
    SGI14 = 14,
    /// Software Generated Interrupt 15.
    SGI15 = 15,

    // Private Peripheral Interrupts.
    /// Virtual Maintenance Interrupt.
    VirtualMaintenanceInterrupt = 25,
    /// Hypervisor Timer Interrupt.
    HypervisorTimer = 26,
    /// Virtual Timer Interrupt.
    VirtualTimer = 27,
    /// Legacy nFIQ Interrupt.
    Legacy_nFIQ = 28,
    /// Secure Physical Timer Interrupt.
    SecurePhysicalTimer = 29,
    /// Non-Secure Physical Timer Interrupt.
    NonSecurePhysicalTimer = 30,
    /// Legacy nIRQ Interrupt.
    Legacy_nIRQ = 31,

    // STM32 specific Interrupt Numbers.
    /// Window WatchDog Interrupt.
    WWDG1 = 32,
    /// PVD & AVD detector through EXTI.
    PVD_AVD = 33,
    /// Tamper interrupts through the EXTI line.
    TAMP = 34,
    /// RTC Wakeup and Alarm (A & B) interrupt through the EXTI line.
    RTC_WKUP_ALARM = 35,
    /// TrustZone DDR address space controller.
    TZC_IT = 36,
    /// RCC global Interrupt.
    RCC = 37,
    /// EXTI Line0 Interrupt.
    EXTI0 = 38,
    /// EXTI Line1 Interrupt.
    EXTI1 = 39,
    /// EXTI Line2 Interrupt.
    EXTI2 = 40,
    /// EXTI Line3 Interrupt.
    EXTI3 = 41,
    /// EXTI Line4 Interrupt.
    EXTI4 = 42,
    /// DMA1 Stream 0 global Interrupt.
    DMA1_Stream0 = 43,
    /// DMA1 Stream 1 global Interrupt.
    DMA1_Stream1 = 44,
    /// DMA1 Stream 2 global Interrupt.
    DMA1_Stream2 = 45,
    /// DMA1 Stream 3 global Interrupt.
    DMA1_Stream3 = 46,
    /// DMA1 Stream 4 global Interrupt.
    DMA1_Stream4 = 47,
    /// DMA1 Stream 5 global Interrupt.
    DMA1_Stream5 = 48,
    /// DMA1 Stream 6 global Interrupt.
    DMA1_Stream6 = 49,
    /// ADC1 global Interrupts.
    ADC1 = 50,
    /// FDCAN1 Interrupt line 0.
    FDCAN1_IT0 = 51,
    /// FDCAN2 Interrupt line 0.
    FDCAN2_IT0 = 52,
    /// FDCAN1 Interrupt line 1.
    FDCAN1_IT1 = 53,
    /// FDCAN2 Interrupt line 1.
    FDCAN2_IT1 = 54,
    /// External Line[9:5] Interrupts.
    EXTI5 = 55,
    /// TIM1 Break interrupt.
    TIM1_BRK = 56,
    /// TIM1 Update interrupt.
    TIM1_UP = 57,
    /// TIM1 Trigger and Commutation interrupt.
    TIM1_TRG_COM = 58,
    /// TIM1 Capture Compare interrupt.
    TIM1_CC = 59,
    /// TIM2 global interrupt.
    TIM2 = 60,
    /// TIM3 global interrupt.
    TIM3 = 61,
    /// TIM4 global interrupt.
    TIM4 = 62,
    /// I2C1 Event interrupt.
    I2C1_EV = 63,
    /// I2C1 Error interrupt.
    I2C1_ER = 64,
    /// I2C2 Event interrupt.
    I2C2_EV = 65,
    /// I2C2 Error interrupt.
    I2C2_ER = 66,
    /// SPI1 global interrupt.
    SPI1 = 67,
    /// SPI2 global interrupt.
    SPI2 = 68,
    /// USART1 global interrupt.
    USART1 = 69,
    /// USART2 global interrupt.
    USART2 = 70,
    /// USART3 global interrupt.
    USART3 = 71,
    /// EXTI Line 10 interrupt.
    EXTI10 = 72,
    /// RTC TimeStamp through EXTI Line interrupt.
    RTC_TIMESTAMP = 73,
    /// EXTI Line 11 interrupt.
    EXTI11 = 74,
    /// TIM8 Break interrupt.
    TIM8_BRK = 75,
    /// TIM8 Update interrupt.
    TIM8_UP = 76,
    /// TIM8 Trigger and Commutation interrupt.
    TIM8_TRG_COM = 77,
    /// TIM8 Capture Compare interrupt.
    TIM8_CC = 78,
    /// DMA1 Stream7 interrupt.
    DMA1_Stream7 = 79,
    /// FMC global interrupt.
    FMC = 80,
    /// SDMMC1 global interrupt.
    SDMMC1 = 81,
    /// TIM5 global interrupt.
    TIM5 = 82,
    /// SPI3 global interrupt.
    SPI3 = 83,
    /// UART4 global interrupt.
    UART4 = 84,
    /// UART5 global interrupt.
    UART5 = 85,
    /// TIM6 global interrupt.
    TIM6 = 86,
    /// TIM7 global interrupt.
    TIM7 = 87,
    /// DMA2 Stream 0 global interrupt.
    DMA2_Stream0 = 88,
    /// DMA2 Stream 1 global interrupt.
    DMA2_Stream1 = 89,
    /// DMA2 Stream 2 global interrupt.
    DMA2_Stream2 = 90,
    /// GPDMA2 Stream 3 global interrupt.
    DMA2_Stream3 = 91,
    /// GPDMA2 Stream 4 global interrupt.
    DMA2_Stream4 = 92,
    /// Ethernet global interrupt.
    ETH1 = 93,
    /// Ethernet Wakeup through EXTI line interrupt.
    ETH1_WKUP = 94,
    /// CAN calibration unit interrupt.
    FDCAN_CAL = 95,
    /// EXTI Line 6 interrupt.
    EXTI6 = 96,
    /// EXTI Line 7 interrupt.
    EXTI7 = 97,
    /// EXTI Line 8 interrupt.
    EXTI8 = 98,
    /// EXTI Line 9 interrupt.
    EXTI9 = 99,
    /// DMA2 Stream 5 global interrupt.
    DMA2_Stream5 = 100,
    /// DMA2 Stream 6 global interrupt
    DMA2_Stream6 = 101,
    /// DMA2 Stream 7 global interrupt
    DMA2_Stream7 = 102,
    /// USART6 global interrupt.
    USART6 = 103,
    /// I2C3 event interrupt.
    I2C3_EV = 104,
    /// I2C3 error interrupt.
    I2C3_ER = 105,
    /// USB OHCI global interrupt.
    USBH_OHCI = 106,
    /// USB EHCI global interrupt.
    USBH_EHCI = 107,
    /// EXTI Line 76 interrupt.
    EXTI12 = 108,
    /// EXTI Line 77 interrupt.
    EXTI13 = 109,
    /// DCMI global interrupt.
    DCMI = 110,
    /// CRYP crypto global interrupt.
    CRYP1 = 111,
    /// Hash global interrupt.
    HASH1 = 112,
    /// reserved
    RESERVED_113 = 113,
    /// UART7 global interrupt.
    UART7 = 114,
    /// UART8 global interrupt.
    UART8 = 115,
    /// SPI4 global interrupt.
    SPI4 = 116,
    /// SPI5 global interrupt.
    SPI5 = 117,
    /// SPI6 global interrupt.
    SPI6 = 118,
    /// SAI1 global interrupt.
    SAI1 = 119,
    /// LTDC global interrupt.
    LTDC = 120,
    /// LTDC Error global interrupt.
    LTDC_ER = 121,
    /// ADC2 global interrupt.
    ADC2 = 122,
    /// SAI2 global interrupt.
    SAI2 = 123,
    /// Quad SPI global interrupt.
    QUADSPI = 124,
    /// LP TIM1 interrupt.
    LPTIM1 = 125,
    /// HDMI-CEC global interrupt.
    CEC = 126,
    /// I2C4 Event interrupt.
    I2C4_EV = 127,
    /// I2C4 Error interrupt.
    I2C4_ER = 128,
    /// SPDIF-RX global interrupt.
    SPDIF_RX = 129,
    /// USB On The Go global interrupt.
    OTG = 130,
    /// RESERVED interrupt.
    RESERVED_131 = 131,
    /// IPCC RX0 Occupied interrupt (interrupt going to AIEC input as well).
    IPCC_RX0 = 132,
    /// IPCC TX0 Free interrupt (interrupt going to AIEC input as well).
    IPCC_TX0 = 133,
    /// DMAMUX1 Overrun interrupt.
    DMAMUX1_OVR = 134,
    /// IPCC RX1 Occupied interrupt (interrupt going to AIEC input as well).
    IPCC_RX1 = 135,
    /// IPCC TX1 Free interrupt (interrupt going to AIEC input as well).
    IPCC_TX1 = 136,
    /// CRYP2 crypto global interrupt.
    CRYP2 = 137,
    /// Crypto Hash2 interrupt.
    HASH2 = 138,
    /// I2C5 Event interrupt.
    I2C5_EV = 139,
    /// I2C5 Error interrupt.
    I2C5_ER = 140,
    /// GPU global interrupt.
    GPU = 141,
    /// DFSDM Filter1 interrupt.
    DFSDM1_FLT0 = 142,
    /// DFSDM Filter2 interrupt.
    DFSDM1_FLT1 = 143,
    /// DFSDM Filter3 interrupt.
    DFSDM1_FLT2 = 144,
    /// DFSDM Filter4 interrupt.
    DFSDM1_FLT3 = 145,
    /// SAI3 global interrupt.
    SAI3 = 146,
    /// DFSDM Filter5 interrupt.
    DFSDM1_FLT4 = 147,
    /// TIM15 global interrupt.
    TIM15 = 148,
    /// TIM16 global interrupt.
    TIM16 = 149,
    /// TIM17 global interrupt.
    TIM17 = 150,
    /// TIM12 global interrupt.
    TIM12 = 151,
    /// MDIOS global interrupt.
    MDIOS = 152,
    /// EXTI Line 14 interrupt.
    EXTI14 = 153,
    /// MDMA global interrupt.
    MDMA = 154,
    /// DSI global interrupt.
    DSI = 155,
    /// SDMMC2 global interrupt.
    SDMMC2 = 156,
    /// HSEM Semaphore interrupt 1.
    HSEM_IT1 = 157,
    /// DFSDM Filter6 interrupt.
    DFSDM1_FLT5 = 158,
    /// EXTI Line 15 Interrupts.
    EXTI15 = 159,
    /// MDMA global Secure interrupt.
    MDMA_SEC_IT = 160,
    /// MCU local Reset Request.
    SYSRESETQ = 161,
    /// TIM13 global interrupt.
    TIM13 = 162,
    /// TIM14 global interrupt.
    TIM14 = 163,
    /// DAC1 and DAC2 underrun error interrupt.
    DAC = 164,
    /// RNG1 interrupt.
    RNG1 = 165,
    /// RNG2 interrupt.
    RNG2 = 166,
    /// I2C6 Event interrupt.
    I2C6_EV = 167,
    /// I2C6 Error interrupt.
    I2C6_ER = 168,
    /// SDMMC3 global interrupt.
    SDMMC3 = 169,
    /// LP TIM2 global interrupt.
    LPTIM2 = 170,
    /// LP TIM3 global interrupt.
    LPTIM3 = 171,
    /// LP TIM4 global interrupt.
    LPTIM4 = 172,
    /// LP TIM5 global interrupt.
    LPTIM5 = 173,
    /// ETH1_LPI interrupt (LPI: lpi_intr_o).
    ETH1_LPI = 174,
    /// Window Watchdog 1 Reset through AIEC.
    WWDG1_RST = 175,
    /// MCU Send Event interrupt.
    MCU_SEV = 176,
    /// RCC Wake up interrupt.
    RCC_WAKEUP = 177,
    /// SAI4 global interrupt.
    SAI4 = 178,
    /// Temperature sensor Global interrupt.
    DTS = 179,
    /// reserved.
    RESERVED_180 = 180,
    /// Interrupt for all 6 wake-up pins.
    WAKEUP_PIN = 181,
    /// IWDG1 Early interrupt.
    IWDG1 = 182,
    /// IWDG2 Early interrupt.
    IWDG2 = 183,
    /// TAMP Tamper and Security Error Secure interrupts.
    TAMP_SERR_S = 229,
    /// RTC Wakeup Timer and Alarms (A and B) Secure interrupt.
    RTC_WKUP_ALARM_S = 230,
    /// RTC TimeStamp and Security Error Secure interrupt.
    RTC_TS_SERR_S = 231,
}

/// Initializes the interrupt controller.
pub fn init() {
    gic::enable();

    let num_irq = 32 * ((gic::distributor_info() & 0x1) + 1);

    loop {
        let x = gic::acknowledge_pending();

        unsafe {
            asm! {
                "dsb",
                "isb"
            }
        }

        if x < num_irq {
            gic::end_interrupt(x);
            unsafe {
                asm! {
                    "dsb",
                    "isb"
                }
            }
        } else {
            break;
        }
    }

    for i in 32..num_irq {
        let act_pend = gic::get_irq_status(i);
        let active = ((act_pend & 0b10) >> 1) != 0;
        let pending = (act_pend & 0b01) != 0;

        if active {
            gic::clear_active_irq(i);
        }

        if pending {
            gic::clear_pending_irq(i);
        }
    }

    // Reset the active priority register, in case we halted/reset during an ISR.
    // FixMe: This doesn't always work! Sometimes if we halt during an ISR handler,
    // and upload new code, we have to power cycle for it to run properly.
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.apr0().write(|w| w.bits(0));

        asm! {
            "dsb",
            "isb"
        }
    }

    // gic arch v2.0 specification: section 3.3.2: Writing 255 to GICC_PMR always
    // sets it to the largest supported priority field value.
    gic::set_interface_priority_mask(0xFF);

    gic::set_binary_point(4);
}

/// Enables an interrupt.
pub fn enable_irq(irqn: Irqn) {
    gic::enable_irq(irqn as u32);
}

/// Disables an interrupt.
pub fn disable_irq(irqn: Irqn) {
    gic::disable_irq(irqn as u32);
}

/// Sends a software generated interrupt to a specific core.
/// - `0`: MPU0
/// - `1`: MPU1
pub fn send_sgi(irqn: Irqn, core_id: u32) {
    gic::send_sgi(irqn as u32, 1 << core_id, 0);
}

/// Sets the user IRQ handler.
pub fn set_irq_handler(irq_handler: Option<IrqHandler>) {
    critical_section::with(|_| unsafe {
        IRQ_HANDLER = irq_handler;
    });
}

#[no_mangle]
extern "C" fn irq_handler() {
    let irqn = gic::acknowledge_pending();

    unsafe {
        if let Some(irq_handler) = IRQ_HANDLER {
            if let Ok(irqn) = Irqn::try_from(irqn) {
                irq_handler(irqn);
            }
        }
    }

    gic::end_interrupt(irqn);
}

#[no_mangle]
extern "C" fn fiq_handler() {}

//! DMA request multiplexer.

/// DMA request inputs.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum DmaRequestInput {
    /// Memory to memory.
    MemoryToMemory = 0,

    /// Generator 0.
    Generator0 = 1,
    /// Generator 1.
    Generator1 = 2,
    /// Generator 2.
    Generator2 = 3,
    /// Generator 3.
    Generator3 = 4,
    /// Generator 4.
    Generator4 = 5,
    /// Generator 5.
    Generator5 = 6,
    /// Generator 6.
    Generator6 = 7,
    /// Generator 7.
    Generator7 = 8,

    /// ADC1.
    Adc1 = 9,
    /// ADC2.
    Adc2 = 10,

    /// TIM1 channel 1.
    Tim1Ch1 = 11,
    /// TIM1 channel 2.
    Tim1Ch2 = 12,
    /// TIM1 channel 3.
    Tim1Ch3 = 13,
    /// TIM1 channel 4.
    Tim1Ch4 = 14,
    /// TIM1 update.
    Tim1Up = 15,
    /// TIM1 trigger.
    Tim1Trig = 16,
    /// TIM1 COM.
    Tim1Com = 17,

    /// TIM2 channel 1.
    Tim2Ch1 = 18,
    /// TIM2 channel 3.
    Tim2Ch2 = 19,
    /// TIM2 channel 3.
    Tim2Ch3 = 20,
    /// TIM2 channel 4.
    Tim2Ch4 = 21,
    /// TIM2 update.
    Tim2Up = 22,

    /// TIM3 channel 1.
    Tim3Ch1 = 23,
    /// TIM3 channel 2.
    Tim3Ch2 = 24,
    /// TIM3 channel 3.
    Tim3Ch3 = 25,
    /// TIM3 channel 4.
    Tim3Ch4 = 26,
    /// TIM3 update.
    Tim3Up = 27,
    /// TIM3 trigger.
    Tim3Trig = 28,

    /// TIM4 channel 1.
    Tim4Ch1 = 29,
    /// TIM4 channel 2.
    Tim4Ch2 = 30,
    /// TIM4 channel 3.
    Tim4Ch3 = 31,
    /// TIM4 update.
    Tim4Up = 32,

    /// I2C1 receive.
    I2c1Rx = 33,
    /// I2C1 transmit.
    I2c1Tx = 34,
    /// I2C2 receive.
    I2c2Rx = 35,
    /// I2C2 transmit.
    I2c2Tx = 36,

    /// SPI1 receive.
    Spi1Rx = 37,
    /// SPI1 transmit.
    Spi1Tx = 38,
    /// SPI2 receive.
    Spi2Rx = 39,
    /// SPI2 transmit.
    Spi2Tx = 40,

    /// USART2 receive.
    Usart2Rx = 43,
    /// USART2 transmit.
    Usart2Tx = 44,
    /// USART3 receive.
    Usart3Rx = 45,
    /// USART3 transmit.
    Usart3Tx = 46,

    /// TIM8 channel 1.
    Tim8Ch1 = 47,
    /// TIM8 channel 2.
    Tim8Ch2 = 48,
    /// TIM8 channel 3.
    Tim8Ch3 = 49,
    /// TIM8 channel 4.
    Tim8Ch4 = 50,
    /// TIM8 update.
    Tim8Up = 51,
    /// TIM8 trigger.
    Tim8Trig = 52,
    /// TIM8 COM.
    Tim8Com = 53,

    /// TIM5 channel 1.
    Tim5Ch1 = 55,
    /// TIM5 channel 2.
    Tim5Ch2 = 56,
    /// TIM5 channel 3.
    Tim5Ch3 = 57,
    /// TIM5 channel 4.
    Tim5Ch4 = 58,
    /// TIM5 update.
    Tim5Up = 59,
    /// TIM5 trigger.
    Tim5Trig = 60,

    /// SPI3 receive.
    Spi3Rx = 61,
    /// SPI3 transmit.
    Spi3Tx = 62,

    /// UART4 receive.
    Uart4Rx = 63,
    /// UART4 transmit.
    Uart4Tx = 64,
    /// UART5 receive.
    Uart5Rx = 65,
    /// UART5 transmit.
    Uart5Tx = 66,

    /// DAC channel 1.
    DacCh1 = 67,
    /// DAC channel 2.
    DacCh2 = 68,

    /// TIM6 update.
    Tim6Up = 69,
    /// TIM7 update.
    Tim7Up = 70,

    /// USART6 receive.
    Usart6Rx = 71,
    /// USART6 transmit.
    Usart6Tx = 72,

    /// I2C3 receive.
    I2c3Rx = 73,
    /// I2C3 transmit.
    I2c3Tx = 74,

    /// DCMI.
    Dcmi = 75,

    /// CRYP2 input.
    Cryp2In = 76,
    /// CRYP2 output.
    Cryp2Out = 77,

    /// HASH2 input.
    Hash2In = 78,

    /// UART7 receive.
    Uart7Rx = 79,
    /// UART7 transmit.
    Uart7Tx = 80,
    /// UART8 receive.
    Uart8Rx = 81,
    /// UART8 transmit.
    Uart8Tx = 82,

    /// SPI4 receive.
    Spi4Rx = 83,
    /// SPI4 transmit.
    Spi4Tx = 84,
    /// SPI5 receive.
    Spi5Rx = 85,
    /// SPI5 transmit.
    Spi5Tx = 86,

    /// SAI1 A.
    Sai1A = 87,
    /// SAI1 B.
    Sai1B = 88,
    /// SAI2 A.
    Sai2A = 89,
    /// SAI2 B.
    Sai2B = 90,

    /// DSFDM1 filter 4.
    Dfsdm1Flt4 = 91,
    /// DSFDM1 filter 5.
    Dfsdm1Flt5 = 92,

    /// SPDIF receive DT.
    SpdifRxDt = 93,
    /// SPDIF receive CS.
    SpdifRxCs = 94,

    /// SAI4 A.
    Sai4A = 99,
    /// SAI4 B.
    Sai4B = 100,

    /// DSFDM1 filter 0.
    Dfsdm1Flt0 = 101,
    /// DSFDM1 filter 1.
    Dfsdm1Flt1 = 102,
    /// DSFDM1 filter 2.
    Dfsdm1Flt2 = 103,
    /// DSFDM1 filter 3.
    Dfsdm1Flt3 = 104,

    /// TIM15 channel 1.
    Tim15Ch1 = 105,
    /// TIM15 update.
    Tim15Up = 106,
    /// TIM15 trigger.
    Tim15Trig = 107,
    /// TIM15 COM.
    Tim15Com = 108,

    /// TIM16 channel 1.
    Tim16Ch1 = 109,
    /// TIM16 update.
    Tim16Up = 110,

    /// TIM17 channel 1.
    Tim17Ch1 = 111,
    /// TIM17 update.
    Tim17Up = 112,

    /// SAI3 A.
    Sai3A = 113,
    /// SAI3 B.
    Sai3B = 114,

    /// I2C5 receive.
    I2c5Rx = 115,
    /// I2C5 transmit.
    I2c5Tx = 116,
}

impl From<DmaRequestInput> for u8 {
    fn from(value: DmaRequestInput) -> Self {
        value as u8
    }
}

/// DMA sync inputs.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum DmaSyncInput {
    /// DMAMUX1 channel 0 event.
    Event0 = 0,
    /// DMAMUX1 channel 1 event..
    Event1 = 1,
    /// DMAMUX1 channel 2 event..
    Event2 = 2,
    /// LPTIMER1 output.
    LpTimer1Out = 3,
    /// LPTIMER2 output.
    LpTimer2Out = 4,
    /// LPTIMER3 output.
    LpTimer3Out = 5,
    /// EXT IO interrupt.
    ExtIo = 6,
    /// TIM12 trigger output.
    Tim12Trgo = 7,
}

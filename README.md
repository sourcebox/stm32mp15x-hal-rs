# stm32mp15x-hal

Experimental hardware abstraction library (HAL) for STM32MP15x SoCs.

> **WARNING:**
> This project is very incomplete and not recommended for general use. It only contains the parts that are required for specific projects and even these are expected to have breaking changes in the future.
>
> **Please don't open any issues or pull requests! They will not be addressed or accepted!**

## Design Patterns

This HAL tries to be lightweight and make the peripherals accessible in a consistent way from either the Cortex-A7 or the Cortex-M4 cores.

*Important:* Either the `mpu-ca7` or the `mcu-cm4` feature needs to be enabled when importing the crate.

> **WARNING:**
> It's in the responsibility of the user to prevent concurrent access to peripherals from different cores.

Consequently:

- No typestate programming.
- Peripherals are all individual, no overall container struct, no ownership management.
- Minimal state keeping inside the peripheral structs.
- Constructors using `new()` don't do any initialization. They just return instances.

## Implementation Status

### Startup Code & Linker Scripts

Needs rework to be more compliant with other crates like [cortex-m](https://crates.io/crates/cortex-m) in terms of identifier naming and overall handling.

### Cortex-A7-specifc Code

Should go into an independent crate out of the scope of this project, ideally also working for other ARMv7 cores like A5/A9.

### MMU Configuration

Currently only very basic and hardcoded. Needs to be more flexible. No fine-grained cache-control yet.

### Critical Section

Tries to be multicore-safe using the HSEM peripheral of the SoC and re-entrance dectection, but not well tested and probably buggy.

### Interrupt Handling

Only very basic internal handler on the Cortex-A7, efforts will be concentrated on the Cortex-M4 side.

### Peripherals

| Peripheral    | Notes                                     |
|---------------|-------------------------------------------|
| GPIO          | Mostly done.                              |
| DMA           | Should be reimplemented using macros.     |
| ADC           | --- Not implemented yet. ---              |
| I2C           | Basic master, needs more testing.         |
| SPI           | Only master is tested. No I2S support.    |
| USART         | Mostly done.                              |
| SAI           | API not nice for using with DMA.          |
| SDMMC         | Init for SDHC, block read.                |
| USB           | --- Not implemented yet. ---              |
| IWDG          | API may change.                           |
| STGEN         | Working, but counter value is read-only.  |
| RNG           | Clock source is fixed to CSI.             |
| LTDC          | Basic setup, needs more testing.          |
| Timers        | --- Not implemented yet. ---              |

Note that not all peripherals can be accessed from every core, e.g. IWDG is only available for the Cortex-A7 cores (MPU0 and MPU1).

## License

Published under the MIT license.

Author: Oliver Rockstedt <info@sourcebox.de>

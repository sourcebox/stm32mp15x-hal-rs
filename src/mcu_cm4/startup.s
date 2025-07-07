.syntax unified
.thumb
.cpu cortex-m4


.section .reset_handler, "ax"
Reset_Handler:
    // Set stack pointer
    ldr   sp, =_user_stack_end      

    movs  r1, #0
    b  LoopCopyDataInit

CopyDataInit:
    ldr  r3, =_data_init_start
    ldr  r3, [r3, r1]
    str  r3, [r0, r1]
    adds  r1, r1, #4
    
LoopCopyDataInit:
    ldr  r0, =_data_start
    ldr  r3, =_data_end
    adds  r2, r0, r1
    cmp  r2, r3
    bcc  CopyDataInit
    ldr  r2, =_bss_start
    b  LoopFillZerobss

FillZerobss:
    // Zero fill the bss segment.
    movs  r3, #0
    str  r3, [r2], #4

LoopFillZerobss:
    ldr  r3, = _bss_end
    cmp  r2, r3
    bcc  FillZerobss
  
    // Call static constructors for C/C++.
    bl __libc_init_array

    // Call the application's entry point.
    bl main
    b .

__libc_init_array:
        push    {{r4, r5, r6, lr}}
        ldr     r3, =__preinit_array_end
        ldr     r5, =__preinit_array_start
        subs    r3, r3, r5
        asrs    r4, r3, #2
        movs    r6, #0
.L2:
        cmp     r6, r4
        bne     .L3
        ldr     r5, =__init_array_start
        ldr     r3, =__init_array_end
        subs    r3, r3, r5
        asrs    r4, r3, #2
        movs    r6, #0
.L4:
        cmp     r6, r4
        bne     .L5
        pop     {{r4, r5, r6, pc}}
.L3:
        ldr     r3, [r5], #4
        blx     r3
        adds    r6, r6, #1
        b       .L2
.L5:
        ldr     r3, [r5], #4
        blx     r3
        adds    r6, r6, #1
        b       .L4

_init:
        bx      lr

.size Reset_Handler, .-Reset_Handler


.section .default_handler, "ax"
Default_Handler:
    bkpt
    b .
.size Default_Handler, .-Default_Handler


.section .isr_vector, "ax"
vector_table:
    // Minimal vector table for Cortex-M4
    .word  _user_stack_end                  // Top of Stack
    .word  Reset_Handler                    // Reset Handler
    .word  NMI_Handler                      // NMI Handler
    .word  HardFault_Handler                // Hard Fault Handler
    .word  MemManage_Handler                // MPU Fault Handler
    .word  BusFault_Handler                 // Bus Fault Handler
    .word  UsageFault_Handler               // Usage Fault Handler
    .word  0                                // Reserved
    .word  0                                // Reserved
    .word  0                                // Reserved
    .word  0                                // Reserved
    .word  SVC_Handler                      // SVCall Handler
    .word  DebugMon_Handler                 // Debug Monitor Handler
    .word  0                                // Reserved
    .word  PendSV_Handler                   // PendSV Handler
    .word  SysTick_Handler                  // SysTick Handler

    // External Interrupts
    .word  WWDG1_IRQHandler                 // Window WatchDog 1
    .word  PVD_AVD_IRQHandler               // PVD and AVD through EXTI Line detection                        
    .word  TAMP_IRQHandler                  // Tamper and TimeStamps through the EXTI line
    .word  RTC_WKUP_ALARM_IRQHandler        // RTC Wakeup and Alarm through the EXTI line
    .word  RESERVED4_IRQHandler             // Reserved
    .word  RCC_IRQHandler                   // RCC                                             
    .word  EXTI0_IRQHandler                 // EXTI Line0                                             
    .word  EXTI1_IRQHandler                 // EXTI Line1                                             
    .word  EXTI2_IRQHandler                 // EXTI Line2                                             
    .word  EXTI3_IRQHandler                 // EXTI Line3                                             
    .word  EXTI4_IRQHandler                 // EXTI Line4 
    .word  DMA1_Stream0_IRQHandler          // DMA1 Stream 0
    .word  DMA1_Stream1_IRQHandler          // DMA1 Stream 1                                   
    .word  DMA1_Stream2_IRQHandler          // DMA1 Stream 2                                   
    .word  DMA1_Stream3_IRQHandler          // DMA1 Stream 3                                   
    .word  DMA1_Stream4_IRQHandler          // DMA1 Stream 4                                   
    .word  DMA1_Stream5_IRQHandler          // DMA1 Stream 5
    .word  DMA1_Stream6_IRQHandler          // DMA1 Stream 6 
    .word  ADC1_IRQHandler                  // ADC1                             
    .word  FDCAN1_IT0_IRQHandler            // FDCAN1 Interrupt line 0
    .word  FDCAN2_IT0_IRQHandler            // FDCAN2 Interrupt line 0
    .word  FDCAN1_IT1_IRQHandler            // FDCAN1 Interrupt line 1
    .word  FDCAN2_IT1_IRQHandler            // FDCAN2 Interrupt line 1
    .word  EXTI5_IRQHandler                 // External Line5 interrupts through AIEC
    .word  TIM1_BRK_IRQHandler              // TIM1 Break interrupt
    .word  TIM1_UP_IRQHandler               // TIM1 Update Interrupt
    .word  TIM1_TRG_COM_IRQHandler          // TIM1 Trigger and Commutation Interrupt
    .word  TIM1_CC_IRQHandler               // TIM1 Capture Compare                                   
    .word  TIM2_IRQHandler                  // TIM2                                            
    .word  TIM3_IRQHandler                  // TIM3                                            
    .word  TIM4_IRQHandler                  // TIM4                                            
    .word  I2C1_EV_IRQHandler               // I2C1 Event                                             
    .word  I2C1_ER_IRQHandler               // I2C1 Error                                             
    .word  I2C2_EV_IRQHandler               // I2C2 Event                                             
    .word  I2C2_ER_IRQHandler               // I2C2 Error                                               
    .word  SPI1_IRQHandler                  // SPI1                                            
    .word  SPI2_IRQHandler                  // SPI2                                            
    .word  USART1_IRQHandler                // USART1                                          
    .word  USART2_IRQHandler                // USART2                                          
    .word  USART3_IRQHandler                // USART3                                          
    .word  EXTI10_IRQHandler                // External Line10 interrupts through AIEC
    .word  RTC_TIMESTAMP_IRQHandler         // RTC TimeStamp through EXTI Line
    .word  EXTI11_IRQHandler                // External Line11 interrupts through AIEC
    .word  TIM8_BRK_IRQHandler              // TIM8 Break Interrupt
    .word  TIM8_UP_IRQHandler               // TIM8 Update Interrupt
    .word  TIM8_TRG_COM_IRQHandler          // TIM8 Trigger and Commutation Interrupt
    .word  TIM8_CC_IRQHandler               // TIM8 Capture Compare Interrupt
    .word  DMA1_Stream7_IRQHandler          // DMA1 Stream7                                           
    .word  FMC_IRQHandler                   // FMC                             
    .word  SDMMC1_IRQHandler                // SDMMC1
    .word  TIM5_IRQHandler                  // TIM5                            
    .word  SPI3_IRQHandler                  // SPI3                            
    .word  UART4_IRQHandler                 // UART4                           
    .word  UART5_IRQHandler                 // UART5                           
    .word  TIM6_IRQHandler                  // TIM6
    .word  TIM7_IRQHandler                  // TIM7           
    .word  DMA2_Stream0_IRQHandler          // DMA2 Stream 0                   
    .word  DMA2_Stream1_IRQHandler          // DMA2 Stream 1                   
    .word  DMA2_Stream2_IRQHandler          // DMA2 Stream 2                   
    .word  DMA2_Stream3_IRQHandler          // DMA2 Stream 3                   
    .word  DMA2_Stream4_IRQHandler          // DMA2 Stream 4                   
    .word  ETH1_IRQHandler                  // Ethernet                        
    .word  ETH1_WKUP_IRQHandler             // Ethernet Wakeup through EXTI line              
    .word  FDCAN_CAL_IRQHandler             // FDCAN Calibration
    .word  EXTI6_IRQHandler                 // EXTI Line6 interrupts through AIEC
    .word  EXTI7_IRQHandler                 // EXTI Line7 interrupts through AIEC
    .word  EXTI8_IRQHandler                 // EXTI Line8 interrupts through AIEC
    .word  EXTI9_IRQHandler                 // EXTI Line9 interrupts through AIEC
    .word  DMA2_Stream5_IRQHandler          // DMA2 Stream 5                   
    .word  DMA2_Stream6_IRQHandler          // DMA2 Stream 6                   
    .word  DMA2_Stream7_IRQHandler          // DMA2 Stream 7                   
    .word  USART6_IRQHandler                // USART6                           
    .word  I2C3_EV_IRQHandler               // I2C3 event                             
    .word  I2C3_ER_IRQHandler               // I2C3 error                             
    .word  USBH_OHCI_IRQHandler             // USB Host OHCI
    .word  USBH_EHCI_IRQHandler             // USB Host EHCI
    .word  EXTI12_IRQHandler                // EXTI Line12 interrupts through AIEC
    .word  EXTI13_IRQHandler                // EXTI Line13 interrupts through AIEC
    .word  DCMI_IRQHandler                  // DCMI                            
    .word  CRYP1_IRQHandler                 // Crypto1 global interrupt
    .word  HASH1_IRQHandler                 // Crypto Hash1 interrupt
    .word  FPU_IRQHandler                   // FPU
    .word  UART7_IRQHandler                 // UART7
    .word  UART8_IRQHandler                 // UART8
    .word  SPI4_IRQHandler                  // SPI4
    .word  SPI5_IRQHandler                  // SPI5
    .word  SPI6_IRQHandler                  // SPI6
    .word  SAI1_IRQHandler                  // SAI1
    .word  LTDC_IRQHandler                  // LTDC
    .word  LTDC_ER_IRQHandler               // LTDC error
    .word  ADC2_IRQHandler                  // ADC2 
    .word  SAI2_IRQHandler                  // SAI2
    .word  QUADSPI_IRQHandler               // QUADSPI
    .word  LPTIM1_IRQHandler                // LPTIM1 global interrupt
    .word  CEC_IRQHandler                   // HDMI_CEC
    .word  I2C4_EV_IRQHandler               // I2C4 Event                             
    .word  I2C4_ER_IRQHandler               // I2C4 Error 
    .word  SPDIF_RX_IRQHandler              // SPDIF_RX
    .word  OTG_IRQHandler                   // USB On The Go HS global interrupt
    .word  RESERVED99_IRQHandler            // Reserved
    .word  IPCC_RX0_IRQHandler              // Mailbox RX0 Free interrupt
    .word  IPCC_TX0_IRQHandler              // Mailbox TX0 Free interrupt
    .word  DMAMUX1_OVR_IRQHandler           // DMAMUX1 Overrun interrupt
    .word  IPCC_RX1_IRQHandler              // Mailbox RX1 Free interrupt
    .word  IPCC_TX1_IRQHandler              // Mailbox TX1 Free interrupt
    .word  CRYP2_IRQHandler                 // Crypto2 global interrupt
    .word  HASH2_IRQHandler                 // Crypto Hash2 interrupt
    .word  I2C5_EV_IRQHandler               // I2C5 Event Interrupt
    .word  I2C5_ER_IRQHandler               // I2C5 Error Interrupt
    .word  GPU_IRQHandler                   // GPU Global Interrupt
    .word  DFSDM1_FLT0_IRQHandler           // DFSDM Filter0 Interrupt
    .word  DFSDM1_FLT1_IRQHandler           // DFSDM Filter1 Interrupt
    .word  DFSDM1_FLT2_IRQHandler           // DFSDM Filter2 Interrupt
    .word  DFSDM1_FLT3_IRQHandler           // DFSDM Filter3 Interrupt
    .word  SAI3_IRQHandler                  // SAI3 global Interrupt
    .word  DFSDM1_FLT4_IRQHandler           // DFSDM Filter4 Interrupt
    .word  TIM15_IRQHandler                 // TIM15 global Interrupt
    .word  TIM16_IRQHandler                 // TIM16 global Interrupt
    .word  TIM17_IRQHandler                 // TIM17 global Interrupt
    .word  TIM12_IRQHandler                 // TIM12 global Interrupt
    .word  MDIOS_IRQHandler                 // MDIOS global Interrupt
    .word  EXTI14_IRQHandler                // EXTI Line14 interrupts through AIEC
    .word  MDMA_IRQHandler                  // MDMA global Interrupt
    .word  DSI_IRQHandler                   // DSI global Interrupt
    .word  SDMMC2_IRQHandler                // SDMMC2 global Interrupt
    .word  HSEM_IT2_IRQHandler              // HSEM global Interrupt
    .word  DFSDM1_FLT5_IRQHandler           // DFSDM Filter5 Interrupt
    .word  EXTI15_IRQHandler                // EXTI Line15 interrupts through AIEC
    .word  nCTIIRQ1_IRQHandler              // Cortex-M4 CTI interrupt 1
    .word  nCTIIRQ2_IRQHandler              // Cortex-M4 CTI interrupt 2
    .word  TIM13_IRQHandler                 // TIM13 global interrupt
    .word  TIM14_IRQHandler                 // TIM14 global interrupt
    .word  DAC_IRQHandler                   // DAC1 and DAC2 underrun error interrupts
    .word  RNG1_IRQHandler                  // RNG1 interrupt
    .word  RNG2_IRQHandler                  // RNG2 interrupt
    .word  I2C6_EV_IRQHandler               // I2C6 Event Interrupt
    .word  I2C6_ER_IRQHandler               // I2C6 Error Interrupt
    .word  SDMMC3_IRQHandler                // SDMMC3 global Interrupt
    .word  LPTIM2_IRQHandler                // LPTIM2 global interrupt
    .word  LPTIM3_IRQHandler                // LPTIM3 global interrupt
    .word  LPTIM4_IRQHandler                // LPTIM4 global interrupt
    .word  LPTIM5_IRQHandler                // LPTIM5 global interrupt
    .word  ETH1_LPI_IRQHandler              // ETH1_LPI interrupt 
    .word  RESERVED143_IRQHandler           // Reserved
    .word  MPU_SEV_IRQHandler               // MPU Send Event through AIEC
    .word  RCC_WAKEUP_IRQHandler            // RCC Wake up interrupt
    .word  SAI4_IRQHandler                  // SAI4 global interrupt
    .word  DTS_IRQHandler                   // Temperature sensor interrupt
    .word  RESERVED148_IRQHandler           // Reserved
    .word  WAKEUP_PIN_IRQHandler            // Interrupt for all 6 wake-up pins


    // Weak aliases for all interrupts. Can be redefined in application code.
    .weak       NMI_Handler
    .thumb_set  NMI_Handler, Default_Handler

    .weak       HardFault_Handler
    .thumb_set  HardFault_Handler, Default_Handler
    
    .weak       MemManage_Handler
    .thumb_set  MemManage_Handler, Default_Handler

    .weak       BusFault_Handler
    .thumb_set  BusFault_Handler, Default_Handler

    .weak       UsageFault_Handler
    .thumb_set  UsageFault_Handler, Default_Handler

    .weak       SVC_Handler
    .thumb_set  SVC_Handler, Default_Handler

    .weak       DebugMon_Handler
    .thumb_set  DebugMon_Handler, Default_Handler

    .weak       PendSV_Handler
    .thumb_set  PendSV_Handler, Default_Handler

    .weak       SysTick_Handler
    .thumb_set  SysTick_Handler, Default_Handler

    .weak       RESERVED4_IRQHandler
    .weak       RESERVED99_IRQHandler
    .weak       ETH1_LPI_IRQHandler
    .weak       RESERVED143_IRQHandler
    .weak       WWDG1_IRQHandler
    .weak       PVD_AVD_IRQHandler                      
    .weak       TAMP_IRQHandler
    .weak       RTC_WKUP_ALARM_IRQHandler                   
    .weak       RCC_IRQHandler                                   
    .weak       EXTI0_IRQHandler                    
    .weak       EXTI1_IRQHandler                    
    .weak       EXTI2_IRQHandler                    
    .weak       EXTI3_IRQHandler                    
    .weak       EXTI4_IRQHandler                    
    .weak       DMA1_Stream0_IRQHandler
    .weak       DMA1_Stream1_IRQHandler             
    .weak       DMA1_Stream2_IRQHandler             
    .weak       DMA1_Stream3_IRQHandler             
    .weak       DMA1_Stream4_IRQHandler             
    .weak       DMA1_Stream5_IRQHandler             
    .weak       DMA1_Stream6_IRQHandler             
    .weak       ADC1_IRQHandler                      
    .weak       ADC2_IRQHandler                      
    .weak       FDCAN1_IT0_IRQHandler
    .weak       FDCAN2_IT0_IRQHandler
    .weak       FDCAN1_IT1_IRQHandler
    .weak       FDCAN2_IT1_IRQHandler
    .weak       FDCAN_CAL_IRQHandler
    .weak       EXTI5_IRQHandler
    .weak       TIM1_BRK_IRQHandler
    .weak       TIM1_UP_IRQHandler
    .weak       TIM1_TRG_COM_IRQHandler
    .weak       TIM1_CC_IRQHandler                  
    .weak       TIM2_IRQHandler                     
    .weak       TIM3_IRQHandler                     
    .weak       TIM4_IRQHandler                     
    .weak       I2C1_EV_IRQHandler                  
    .weak       I2C1_ER_IRQHandler                  
    .weak       I2C2_EV_IRQHandler                  
    .weak       I2C2_ER_IRQHandler                  
    .weak       SPI1_IRQHandler                     
    .weak       SPI2_IRQHandler                     
    .weak       USART1_IRQHandler                   
    .weak       USART2_IRQHandler                   
    .weak       USART3_IRQHandler                   
    .weak       EXTI10_IRQHandler
    .weak       RTC_TIMESTAMP_IRQHandler
    .weak       EXTI11_IRQHandler
    .weak       TIM8_BRK_IRQHandler
    .weak       TIM8_UP_IRQHandler
    .weak       TIM8_TRG_COM_IRQHandler
    .weak       TIM8_CC_IRQHandler                  
    .weak       DMA1_Stream7_IRQHandler             
    .weak       FMC_IRQHandler                      
    .weak       SDMMC1_IRQHandler
    .weak       TIM5_IRQHandler                     
    .weak       SPI3_IRQHandler                     
    .weak       UART4_IRQHandler                    
    .weak       UART5_IRQHandler                    
    .weak       TIM6_IRQHandler
    .weak       TIM7_IRQHandler                     
    .weak       DMA2_Stream0_IRQHandler             
    .weak       DMA2_Stream1_IRQHandler             
    .weak       DMA2_Stream2_IRQHandler             
    .weak       DMA2_Stream3_IRQHandler             
    .weak       DMA2_Stream4_IRQHandler             
    .weak       ETH1_IRQHandler                      
    .weak       ETH1_WKUP_IRQHandler                 
    .weak       ETH1_LPI_IRQHandler                 
    .weak       EXTI6_IRQHandler
    .weak       EXTI7_IRQHandler
    .weak       EXTI8_IRQHandler
    .weak       EXTI9_IRQHandler
    .weak       DMA2_Stream5_IRQHandler             
    .weak       DMA2_Stream6_IRQHandler             
    .weak       DMA2_Stream7_IRQHandler             
    .weak       USART6_IRQHandler                   
    .weak       I2C3_EV_IRQHandler                  
    .weak       I2C3_ER_IRQHandler                  
    .weak       USBH_OHCI_IRQHandler
    .weak       USBH_EHCI_IRQHandler
    .weak       EXTI12_IRQHandler
    .weak       EXTI13_IRQHandler
    .weak       DCMI_IRQHandler                     
    .weak       CRYP1_IRQHandler
    .weak       HASH1_IRQHandler
    .weak       FPU_IRQHandler                      
    .weak       UART7_IRQHandler                    
    .weak       UART8_IRQHandler                    
    .weak       SPI4_IRQHandler                     
    .weak       SPI5_IRQHandler                     
    .weak       SPI6_IRQHandler                     
    .weak       SAI1_IRQHandler                     
    .weak       LTDC_IRQHandler                     
    .weak       LTDC_ER_IRQHandler                  
    .weak       SAI2_IRQHandler                     
    .weak       QUADSPI_IRQHandler                  
    .weak       LPTIM1_IRQHandler
    .weak       CEC_IRQHandler                      
    .weak       I2C4_EV_IRQHandler                  
    .weak       I2C4_ER_IRQHandler                  
    .weak       SPDIF_RX_IRQHandler                 
    .weak       OTG_IRQHandler
    .weak       IPCC_RX0_IRQHandler
    .weak       IPCC_TX0_IRQHandler
    .weak       DMAMUX1_OVR_IRQHandler
    .weak       IPCC_RX1_IRQHandler
    .weak       IPCC_TX1_IRQHandler
    .weak       CRYP2_IRQHandler
    .weak       HASH2_IRQHandler
    .weak       I2C5_EV_IRQHandler
    .weak       I2C5_ER_IRQHandler
    .weak       GPU_IRQHandler
    .weak       DFSDM1_FLT0_IRQHandler
    .weak       DFSDM1_FLT1_IRQHandler
    .weak       DFSDM1_FLT2_IRQHandler
    .weak       DFSDM1_FLT3_IRQHandler
    .weak       SAI3_IRQHandler                        
    .weak       DFSDM1_FLT4_IRQHandler
    .weak       TIM15_IRQHandler                       
    .weak       TIM16_IRQHandler                       
    .weak       TIM17_IRQHandler                       
    .weak       TIM12_IRQHandler                       
    .weak       MDIOS_IRQHandler                       
    .weak       EXTI14_IRQHandler
    .weak       MDMA_IRQHandler                        
    .weak       DSI_IRQHandler                         
    .weak       SDMMC2_IRQHandler                      
    .weak       HSEM_IT2_IRQHandler
    .weak       DFSDM1_FLT5_IRQHandler
    .weak       EXTI15_IRQHandler
    .weak       nCTIIRQ1_IRQHandler
    .weak       nCTIIRQ2_IRQHandler
    .weak       TIM13_IRQHandler
    .weak       TIM14_IRQHandler
    .weak       DAC_IRQHandler
    .weak       RNG1_IRQHandler
    .weak       RNG2_IRQHandler
    .weak       I2C6_EV_IRQHandler
    .weak       I2C6_ER_IRQHandler
    .weak       SDMMC3_IRQHandler
    .weak       LPTIM2_IRQHandler
    .weak       LPTIM3_IRQHandler
    .weak       LPTIM4_IRQHandler
    .weak       LPTIM5_IRQHandler
    .weak       MPU_SEV_IRQHandler
    .weak       RCC_WAKEUP_IRQHandler
    .weak       SAI4_IRQHandler
    .weak       DTS_IRQHandler
    .weak       RESERVED148_IRQHandler
    .weak       WAKEUP_PIN_IRQHandler

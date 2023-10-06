// Startup code for MPU0.

.syntax unified
.cpu cortex-a7

.equ MODE_FIQ, 0x11
.equ MODE_IRQ, 0x12
.equ MODE_SVC, 0x13
.equ MODE_SYS, 0x1F


.section .reset_handler, "ax"
Reset_Handler:
    cpsid   if

    mrc     p15, 0, r0, c1, c0, 0			// Read System Control register (SCTLR)
    bic     r0, r0, #(0x1 << 12) 			// Clear I bit 12 to disable I Cache
    bic     r0, r0, #(0x1 <<  2) 			// Clear C bit  2 to disable D Cache
    bic     r0, r0, #0x1 					// Clear M bit  0 to disable MMU
    bic     r0, r0, #(0x1 << 11) 			// Clear Z bit 11 to disable branch prediction
    bic     r0, r0, #(0x1 << 13) 			// Clear V bit 13 to disable High Vector Table Base Address
    mcr     p15, 0, r0, c1, c0, 0 			// Write System Control register (SCTLR)
    isb

    // Configure ACTLR
    mrc     p15, 0, r0, c1, c0, 1 			// Read CP15 Auxiliary Control Register
    orr     r0, r0, #(1 <<  1) 				// Enable L2 prefetch hint (UNK/WI since r4p1)
    mcr     p15, 0, r0, c1, c0, 1 		    // Write CP15 Auxiliary Control Register

    // Set Vector Base Address Register (VBAR) to point to this application's vector table.
    ldr    r0, =vector_table
    mcr    p15, 0, r0, c12, c0, 0

    // FIQ stack
    cpsid   if, #MODE_FIQ 
    ldr sp, =_mpu0_fiq_stack_end

    // IRQ stack
    cpsid   if, #MODE_IRQ
    ldr sp, =_mpu0_irq_stack_end

    // Supervisor (SVC) stack
    cpsid   if, #MODE_SVC
    ldr sp, =_mpu0_svc_stack_end

    // USER and SYS mode stack
    cpsid   if, #MODE_SYS
    ldr sp, =_mpu0_user_stack_end

    // Copying initialization values (.data)
    ldr r0, =_text_end
    ldr r1, =_data_start
    ldr r2, =_data_end
data_loop:
    cmp r1, r2
    ldrlt r3, [r0], #4
    strlt r3, [r1], #4
    blt data_loop

    // Initialize .bss
    mov r0, #0
    ldr r1, =_bss_start
    ldr r2, =_bss_end
bss_loop:
    cmp r1, r2
    strlt r0, [r1], #4
    blt bss_loop

    mov r3, #0x1000
    movt r3, #0xA002
    mov r2, #0000
    movt r2, #0002
    str r2, [r3, #0x0f00]

    // Permit access to VFP, registers by modifying CPACR.
    mrc     p15, 0, r1, c1, c0, 2
    orr     r1, r1, #0x00F00000
    mcr     p15, 0, r1, c1, c0, 2

    // Ensure that subsequent instructions occur in the context of VFP access permitted.
    isb

    // Enable the FPU
    vmrs    r1, fpexc
    orr     r1, r1, #0x40000000
    vmsr    fpexc, r1

    // Initialise FPSCR to a known state.
    // Mask off all bits that do not have to be preserved.
    // Non-preserved bits can/should be zero.
    vmrs    r2, fpscr
    movw    r3, #6060
    movt    r3, #8
    and     r2, r2, r3
    vmsr    fpscr, r2

    // Set bits [11:10] of the NSACR for access to CP10 and CP11 from both
    // secure and non-secure states, and clear the NSASEDIS and NSD32DIS bits.
    mrc    p15, 0, r0, c1, c1, 2
    orr    r0, r0, #0x0C00          // Enable NEON.
    bic    r0, r0, #0xC000          // Clear NSASEDIS/NSD32DIS.

    // Call static constructors for C/C++.
    bl __libc_init_array                    

    // Enable irq interrupts
    cpsie  i 								

run_main:
    bl main
    b .

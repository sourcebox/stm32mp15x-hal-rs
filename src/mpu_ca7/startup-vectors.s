// Vector table and exception handlers.

.section .vector_table, "ax"
.global vector_table
vector_table:
    b Reset_Handler
    b Undef_Handler 						// 0x04 Undefined Instruction 
    b SVC_Handler 							// 0x08 Software Interrupt 
    b PAbt_Handler  						// 0x0C Prefetch Abort
    b DAbt_Handler 							// 0x10 Data Abort
    b . 									// 0x14 Reserved 
    b IRQ_Handler 							// 0x18 IRQ 
    b FIQ_Handler 							// 0x1C FIQ


Undef_Handler:
    bkpt
    b .

PAbt_Handler:
    bkpt
    b .

DAbt_Handler:
    bkpt
    b .

SVC_Handler:
    push   {{r0-r3, r12, lr}}
    // bl     svc_handler
    pop    {{r0-r3, r12, lr}}
    subs   pc, lr, #4

IRQ_Handler:
    push   {{r0-r3, r12, lr}}
    bl     irq_handler
    pop    {{r0-r3, r12, lr}}
    subs   pc, lr, #4

FIQ_Handler:
    push   {{r0-r3, r12, lr}}
    bl     fiq_handler
    pop    {{r0-r3, r12, lr}}
    subs   pc, lr, #4

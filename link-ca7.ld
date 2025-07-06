OUTPUT_FORMAT("elf32-littlearm", "elf32-littlearm", "elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(vector_table)

/* Stack sizes */
MPU0_USER_STACK_SIZE = 16M;
MPU0_SVC_STACK_SIZE = 4K;
MPU0_IRQ_STACK_SIZE = 4K;
MPU0_FIQ_STACK_SIZE = 4K;
MPU1_USER_STACK_SIZE = 16M;
MPU1_IRQ_STACK_SIZE = 4K;
MPU1_FIQ_STACK_SIZE = 4K;

/* Heap size */
HEAP_SIZE = 16M;

/* Calculate total stack size */
STACK_SIZE = MPU0_USER_STACK_SIZE;
STACK_SIZE += MPU0_SVC_STACK_SIZE;
STACK_SIZE += MPU0_IRQ_STACK_SIZE;
STACK_SIZE += MPU0_FIQ_STACK_SIZE;
STACK_SIZE += MPU1_USER_STACK_SIZE;
STACK_SIZE += MPU1_IRQ_STACK_SIZE;
STACK_SIZE += MPU1_FIQ_STACK_SIZE;

MEMORY
{
    ROM (rx) : ORIGIN = 0xC2000040, LENGTH = 0x0DFFFFC0			/* ~230MB */
    RAM (rw): ORIGIN = 0xC0200000, LENGTH = 0x01D00000 			/* ~30MB */
	HEAP (rw) : ORIGIN = 0xD0000000, LENGTH = HEAP_SIZE
	STACK (rw) : ORIGIN = 0xD1000000, LENGTH = STACK_SIZE
	SRAM1 (rw) : ORIGIN = 0x10000000, LENGTH = 128K
	SRAM2 (rw) : ORIGIN = 0x10020000, LENGTH = 128K
	SRAM3 (rw) : ORIGIN = 0x10040000, LENGTH = 64K
	SRAM4 (rw) : ORIGIN = 0x10050000, LENGTH = 64K
	SYSRAM (rw) : ORIGIN = 0x2FFC0000, LENGTH = 256K
}

SECTIONS
{
    .text : {
        *(.vector_table)
		*(.reset_handler)
		*(.text)
        *(.text*)

							/* Todo: check if we need the next 3 lines */
		*(.glue_7)         	/* glue arm to thumb code */
		*(.glue_7t)        	/* glue thumb to arm code */
		*(.eh_frame)

		KEEP (*(.vector_table))
		KEEP (*(.reset_handler))
		KEEP (*(.init)) 	/* libc ctors */
		KEEP (*(.fini)) 	/* libc dtors */

        . = ALIGN(8);
     } > ROM

	.rodata :
	{
		. = ALIGN(8);
		*(.rodata)         
		*(.rodata*) 
		. = ALIGN(8);
	} > ROM

	/* used for unwinding (probably not used, but is ignored if your app doens't use exceptions */
	.ARM.extab	 : { *(.ARM.extab* .gnu.linkonce.armextab.*) } >ROM
	.ARM : {
		__exidx_start = .;
		*(.ARM.exidx*)
		__exidx_end = .;
	} > ROM

	.preinit_array :
	{
		. = ALIGN(8);
		PROVIDE_HIDDEN (__preinit_array_start = .);
		KEEP (*(.preinit_array*))
		PROVIDE_HIDDEN (__preinit_array_end = .);
		. = ALIGN(8);
	} > ROM

	.init_array :
	{
		. = ALIGN(8);
		PROVIDE_HIDDEN (__init_array_start = .);
		KEEP (*(SORT(.init_array.*)))
		KEEP (*(.init_array*))
		PROVIDE_HIDDEN (__init_array_end = .);
		. = ALIGN(8);
	} > ROM

	.fini_array :
	{
		. = ALIGN(8);
		PROVIDE_HIDDEN (__fini_array_start = .);
		KEEP (*(SORT(.fini_array.*)))
		KEEP (*(.fini_array*))
		PROVIDE_HIDDEN (__fini_array_end = .);
		. = ALIGN(8);
	} > ROM

    _text_end = .;

    .data :
    {
        . = ALIGN(8);
        _data_start = .;
        *(.data)
        *(.data*)
        . = ALIGN(8);
        _data_end = .;
    } > RAM

    .bss : 
	{
        . = ALIGN(8);
        _bss_start = .;
        *(.bss)
        *(.bss*)   
		*(COMMON)
        . = ALIGN(8);
        _bss_end = .;
    } > RAM

	.heap (NOLOAD):
	{
		. = ALIGN(8);
		_sheap = .;
		. += HEAP_SIZE;
		_eheap = .;
	} > HEAP

	.stack (NOLOAD):
	{
		. = ALIGN(8);
		. += MPU0_USER_STACK_SIZE;
		_mpu0_user_stack_end = .;
		. += MPU0_SVC_STACK_SIZE;
		_mpu0_svc_stack_end = .;
		. += MPU0_IRQ_STACK_SIZE;
		_mpu0_irq_stack_end = .;
		. += MPU0_FIQ_STACK_SIZE;
		_mpu0_fiq_stack_end = .;
		. += MPU1_USER_STACK_SIZE;
		_mpu1_user_stack_end = .;
		. += MPU1_IRQ_STACK_SIZE;
		_mpu1_irq_stack_end = .;
		. += MPU1_FIQ_STACK_SIZE;
		_mpu1_fiq_stack_end = .;
	} > STACK

	.sram1 (NOLOAD):
	{
		. = ALIGN(8);
		*(.sram1*)
	} > SRAM1

	.sram2 (NOLOAD):
	{
		. = ALIGN(8);
		*(.sram2*)
	} > SRAM2

	.sram3 (NOLOAD):
	{
		. = ALIGN(8);
		*(.sram3*)
	} > SRAM3

	.sram4 (NOLOAD):
	{
		. = ALIGN(8);
		*(.sram4*)
	} > SRAM4

	.sysram (NOLOAD):
	{
		. = ALIGN(8);
		*(.dma*) 
		*(.sysram*)
	} > SYSRAM


  /* Remove information from the compiler libraries */
  /DISCARD/ :
  {
    libc.a ( * )
    libm.a ( * )
    libgcc.a ( * )
  }

	.ARM.attributes 0 : { *(.ARM.attributes) }
}
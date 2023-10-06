//! Global interrupt controller.

use crate::pac;

/// Enable the interrupt distributor using the GIC's CTLR register.
pub fn enable_distributor() {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        gicd.gicd_ctlr.modify(|_, w| w.enablegrp0().set_bit());
    }
}

/// Disable the interrupt distributor using the GIC's CTLR register.
pub fn disable_distributor() {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        gicd.gicd_ctlr.modify(|_, w| w.enablegrp0().clear_bit());
    }
}

/// Reads the GIC's TYPER register.
pub fn distributor_info() -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        gicd.gicd_typer.read().bits()
    }
}

/// Reads the GIC's IIDR register.
pub fn distributor_implementer() -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        gicd.gicd_iidr.read().bits()
    }
}

/// Sets the GIC's ITARGETSR register for the given interrupt.
/// - `irqn`: Interrupt to be configured.
/// - `cpu_target`: CPU interfaces to assign this interrupt to.
pub fn set_target(irqn: u32, cpu_target: u32) {
    let mask = itargetsr((irqn / 4) as usize) & !(0xFF << ((irqn % 4) * 8));
    set_itargetsr(
        (irqn / 4) as usize,
        mask | ((cpu_target & 0xFF) << ((irqn % 4) * 8)),
    );
}

/// Reads the GIC's ITARGETSR register.
/// - `irqn`: Interrupt to acquire the configuration for.
pub fn get_target(irqn: u32) -> u32 {
    (itargetsr((irqn / 4) as usize) >> ((irqn % 4) * 8)) & 0xFF
}

/// Enables the CPU's interrupt interface.
pub fn enable_interface() {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.gicc_ctlr.modify(|_, w| w.enablegrp0().set_bit());
    }
}

/// Disables the CPU's interrupt interface.
pub fn disable_interface() {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.gicc_ctlr.modify(|_, w| w.enablegrp0().clear_bit());
    }
}

/// Reads the CPU's IAR register.
pub fn acknowledge_pending() -> u32 {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.gicc_iar.read().bits()
    }
}

/// Writes the given interrupt number to the CPU's EOIR register.
/// - `irqn`: The interrupt to be signaled as finished.
pub fn end_interrupt(irqn: u32) {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.gicc_eoir.write(|w| w.bits(irqn));
    }
}

/// Enables the given interrupt using GIC's ISENABLER register.
/// - `irqn`: The interrupt to be enabled.
pub fn enable_irq(irqn: u32) {
    set_isenabler((irqn / 32) as usize, 1 << (irqn % 32));
}

/// Gets interrupt enable status using GIC's ISENABLER register.
/// - `irqn`: The interrupt to be queried.
/// - Returns:
///   - 0 - interrupt is not enabled
///   - 1 - interrupt is enabled
pub fn get_enable_irq(irqn: u32) -> u32 {
    (isenabler((irqn / 32) as usize) >> (irqn % 32)) & 1
}

/// Disables the given interrupt using GIC's ICENABLER register.
/// - `irqn`: The interrupt to be disabled.
pub fn disable_irq(irqn: u32) {
    set_icenabler((irqn / 32) as usize, 1 << (irqn % 32));
}

/// Gets interrupt pending status from GIC's ISPENDR register.
/// - `irqn`: The interrupt to be queried.
/// - Returns:
///   - 0 - interrupt is not pending
///   - 1 - interrupt is pendig.
pub fn get_pending_irq(irqn: u32) -> u32 {
    let mut pend;

    if irqn >= 16 {
        pend = (ispendr((irqn / 32) as usize) >> (irqn % 32)) & 1;
    } else {
        // INTID 0-15 Software Generated Interrupt
        pend = (spendsgir((irqn / 4) as usize) >> ((irqn % 4) * 8)) & 0xFF;

        // No CPU identification offered
        if pend != 0 {
            pend = 1;
        } else {
            pend = 0;
        }
    }

    pend
}

/// Sets the given interrupt as pending using GIC's ISPENDR register.
/// - `irqn`: The interrupt to be enabled.
pub fn set_pending_irq(irqn: u32) {
    if irqn >= 16 {
        set_ispendr((irqn / 32) as usize, 1 << (irqn % 32));
    } else {
        // INTID 0-15 Software Generated Interrupt
        set_spendsgir((irqn / 4) as usize, 1 << ((irqn % 4) * 8));
    }
}

/// Clears the given interrupt from being pending using GIC's ICPENDR register.
/// - `irqn`: The interrupt to be cleared.
pub fn clear_pending_irq(irqn: u32) {
    if irqn >= 16 {
        set_icpendr((irqn / 32) as usize, 1 << (irqn % 32));
    } else {
        // INTID 0-15 Software Generated Interrupt
        set_cpendsgir((irqn / 4) as usize, 1 << ((irqn % 4) * 8));
    }
}

/// Clears the given interrupt from being active using GIC's ICACTIVER register.
/// - `irqn`: The interrupt to be cleared.
pub fn clear_active_irq(irqn: u32) {
    if irqn >= 16 {
        set_icactiver((irqn / 32) as usize, 1 << (irqn % 32));
    }
}

/// Sets the interrupt configuration using GIC's ICFGR register.
/// - `irqn`: The interrupt to be configured.
/// - `int_config`: Int_config field value.
///   - Bit 0: Reserved (0 - N-N model, 1 - 1-N model for some GIC before v1)
///   - Bit 1: 0 - level sensitive, 1 - edge triggered
pub fn set_configuration(irqn: u32, int_config: u32) {
    let mut icfgr = icfgr((irqn / 16) as usize);
    let shift = (irqn % 16) << 1;

    icfgr &= !(3 << shift);
    icfgr |= int_config << shift;

    set_icfgr((irqn / 16) as usize, icfgr);
}

/// Gets the interrupt configuration from the GIC's ICFGR register.
/// - `irqn`: Interrupt to acquire the configuration for.
/// - Returns Int_config field value.
///   - Bit 0: Reserved (0 - N-N model, 1 - 1-N model for some GIC before v1)
///   - Bit 1: 0 - level sensitive, 1 - edge triggered
pub fn get_configuration(irqn: u32) -> u32 {
    icfgr((irqn / 16) as usize) >> ((irqn % 16) >> 1)
}

/// Sets the priority for the given interrupt in the GIC's IPRIORITYR register.
/// - `irqn`: The interrupt to be configured.
/// - `priority`: The priority for the interrupt, lower values denote higher priorities.
pub fn set_priority(irqn: u32, priority: u32) {
    let mask = ipriorityr((irqn / 4) as usize) & !(0xFF << ((irqn % 4) * 8));
    set_ipriorityr(
        (irqn / 4) as usize,
        mask | ((priority & 0xFF) << ((irqn % 4) * 8)),
    );
}

/// Reads the current interrupt priority from GIC's IPRIORITYR register.
/// - `irqn`: The interrupt to be queried.
pub fn get_priority(irqn: u32) -> u32 {
    (ipriorityr((irqn / 4) as usize) >> ((irqn % 4) * 8)) & 0xFF
}

/// Sets the interrupt priority mask using CPU's PMR register.
/// - `priority`: Priority mask to be set.
pub fn set_interface_priority_mask(priority: u32) {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.gicc_pmr.write(|w| w.bits(priority & 0xFF));
    }
}

/// Reads the current interrupt priority mask from CPU's PMR register.
pub fn get_interface_priority_mask() -> u32 {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.gicc_pmr.read().bits()
    }
}

/// Configures the group priority and subpriority split point using CPU's BPR register.
/// - `binary_point`: Amount of bits used as subpriority.
pub fn set_binary_point(binary_point: u32) {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.gicc_bpr.write(|w| w.bits(binary_point & 7));
    }
}

/// Reads the current group priority and subpriority split point from CPU's BPR register.
pub fn get_binary_point() -> u32 {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.gicc_bpr.read().bits()
    }
}

/// Gets the status for a given interrupt.
/// - `irqn`: The interrupt to get status for.
/// - Returns:
///   - 0 - not pending/active
///   - 1 - pending
///   - 2 - active
///   - 3 - pending and active
pub fn get_irq_status(irqn: u32) -> u32 {
    let active = ((isactiver((irqn / 32) as usize)) >> (irqn % 32)) & 1;
    let pending = ((ispendr((irqn / 32) as usize)) >> (irqn % 32)) & 1;

    (active << 1) | pending
}

/// Generates a software interrupt using GIC's SGIR register.
/// - `irqn` -  Software interrupt to be generated.
/// - `target_list`: List of CPUs the software interrupt should be forwarded to.
/// - `filter_list`: Filter to be applied to determine interrupt receivers.
pub fn send_sgi(irqn: u32, target_list: u32, filter_list: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        gicd.gicd_sgir.write(|w| {
            w.bits(((filter_list & 0x03) << 24) | ((target_list & 0xFF) << 16) | (irqn & 0x0F))
        });
    }
}

/// Gets the interrupt number of the highest interrupt pending from CPU's HPPIR register.
pub fn get_high_pending_irq() -> u32 {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.gicc_hppir.read().bits()
    }
}

/// Provides information about the implementer and revision of the CPU interface.
pub fn get_interface_id() -> u32 {
    unsafe {
        let gicc = &(*pac::GICC::ptr());
        gicc.gicc_iidr.read().bits()
    }
}

/// Sets the interrupt group from the GIC's IGROUPR register.
/// - `irqn`: The interrupt to be queried.
/// - `group`:  Interrupt group number:
///   - 0 - Group 0
///   - 1 - Group 1
pub fn set_group(irqn: u32, group: u32) {
    let mut igroupr = igroupr((irqn / 32) as usize);
    let shift = irqn % 32;

    igroupr &= !(1 << shift);
    igroupr |= (group & 1) << shift;

    set_igroupr((irqn / 32) as usize, igroupr);
}

/// Gets the interrupt group from the GIC's IGROUPR register.
/// - `irqn`:  The interrupt to be queried.
/// - Returns:
///   - 0 - Group 0
///   - 1 - Group 1
pub fn gic_get_group(irqn: u32) -> u32 {
    (igroupr((irqn / 32) as usize) >> (irqn % 32)) & 1
}

/// Initializes the interrupt distributor.
pub fn dist_init() {
    // A reset sets all bits in the IGROUPRs corresponding to the SPIs to 0,
    // configuring all of the interrupts as Secure.

    // Disable interrupt forwarding.
    disable_distributor();

    // Get the maximum number of interrupts that the GIC supports.
    let num_irq = 32 * ((distributor_info() & 0x1F) + 1);

    // Priority level is implementation defined.
    // To determine the number of priority bits implemented write 0xFF to an IPRIORITYR
    // priority field and read back the value stored.
    set_priority(0, 0xFF);
    let priority_field = get_priority(0);

    for i in 32..num_irq {
        // Disable the SPI interrupt.
        disable_irq(i);

        // Set level-sensitive (and N-N model).
        set_configuration(i, 0);

        // Set priority
        set_priority(i, priority_field / 2);

        // Set target list to CPU0
        set_target(i, 1);
    }

    // Enable distributor
    enable_distributor();
}

/// Initializes the CPU's interrupt interface.
pub fn cpu_interface_init() {
    // A reset sets all bits in the IGROUPRs corresponding to the SPIs to 0,
    // configuring all of the interrupts as Secure.

    // Disable interrupt forwarding.
    disable_interface();

    // Priority level is implementation defined.
    // To determine the number of priority bits implemented write 0xFF to an IPRIORITYR
    // priority field and read back the value stored.
    set_priority(0, 0xFF);
    let priority_field = get_priority(0);

    // SGI and PPI.
    for i in 0..32 {
        if i > 15 {
            // Set level-sensitive (and N-N model) for PPI.
            set_configuration(i, 0);
        }

        // Disable SGI and PPI interrupts.
        disable_irq(i);

        // Set priority.
        set_priority(i, priority_field / 2);
    }

    // Enable interface.
    enable_interface();

    // Set binary point to 0.
    set_binary_point(0);

    // Set priority mask.
    set_interface_priority_mask(0xFF);
}

/// Initializes and enable the GIC.
pub fn enable() {
    dist_init();
    cpu_interface_init(); // per CPU
}

/// Reads the ISENABLER register for an index.
fn isenabler(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_isenabler0.read().bits(),
            1 => gicd.gicd_isenabler1.read().bits(),
            2 => gicd.gicd_isenabler2.read().bits(),
            3 => gicd.gicd_isenabler3.read().bits(),
            4 => gicd.gicd_isenabler4.read().bits(),
            5 => gicd.gicd_isenabler5.read().bits(),
            6 => gicd.gicd_isenabler6.read().bits(),
            7 => gicd.gicd_isenabler7.read().bits(),
            8 => gicd.gicd_isenabler8.read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the ISENABLER register for an index.
fn set_isenabler(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_isenabler0.write(|w| w.bits(value)),
            1 => gicd.gicd_isenabler1.write(|w| w.bits(value)),
            2 => gicd.gicd_isenabler2.write(|w| w.bits(value)),
            3 => gicd.gicd_isenabler3.write(|w| w.bits(value)),
            4 => gicd.gicd_isenabler4.write(|w| w.bits(value)),
            5 => gicd.gicd_isenabler5.write(|w| w.bits(value)),
            6 => gicd.gicd_isenabler6.write(|w| w.bits(value)),
            7 => gicd.gicd_isenabler7.write(|w| w.bits(value)),
            8 => gicd.gicd_isenabler8.write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the ICENABLER register for an index.
fn set_icenabler(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_icenabler0.write(|w| w.bits(value)),
            1 => gicd.gicd_icenabler1.write(|w| w.bits(value)),
            2 => gicd.gicd_icenabler2.write(|w| w.bits(value)),
            3 => gicd.gicd_icenabler3.write(|w| w.bits(value)),
            4 => gicd.gicd_icenabler4.write(|w| w.bits(value)),
            5 => gicd.gicd_icenabler5.write(|w| w.bits(value)),
            6 => gicd.gicd_icenabler6.write(|w| w.bits(value)),
            7 => gicd.gicd_icenabler7.write(|w| w.bits(value)),
            8 => gicd.gicd_icenabler8.write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        }
    }
}

/// Reads the ISPENDR register for an index.
fn ispendr(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_ispendr0.read().bits(),
            1 => gicd.gicd_ispendr1.read().bits(),
            2 => gicd.gicd_ispendr2.read().bits(),
            3 => gicd.gicd_ispendr3.read().bits(),
            4 => gicd.gicd_ispendr4.read().bits(),
            5 => gicd.gicd_ispendr5.read().bits(),
            6 => gicd.gicd_ispendr6.read().bits(),
            7 => gicd.gicd_ispendr7.read().bits(),
            8 => gicd.gicd_ispendr8.read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the ISPENDR register for an index.
fn set_ispendr(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_ispendr0.write(|w| w.bits(value)),
            1 => gicd.gicd_ispendr1.write(|w| w.bits(value)),
            2 => gicd.gicd_ispendr2.write(|w| w.bits(value)),
            3 => gicd.gicd_ispendr3.write(|w| w.bits(value)),
            4 => gicd.gicd_ispendr4.write(|w| w.bits(value)),
            5 => gicd.gicd_ispendr5.write(|w| w.bits(value)),
            6 => gicd.gicd_ispendr6.write(|w| w.bits(value)),
            7 => gicd.gicd_ispendr7.write(|w| w.bits(value)),
            8 => gicd.gicd_ispendr8.write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the ICPENDR register for an index.
fn set_icpendr(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_icpendr0.write(|w| w.bits(value)),
            1 => gicd.gicd_icpendr1.write(|w| w.bits(value)),
            2 => gicd.gicd_icpendr2.write(|w| w.bits(value)),
            3 => gicd.gicd_icpendr3.write(|w| w.bits(value)),
            4 => gicd.gicd_icpendr4.write(|w| w.bits(value)),
            5 => gicd.gicd_icpendr5.write(|w| w.bits(value)),
            6 => gicd.gicd_icpendr6.write(|w| w.bits(value)),
            7 => gicd.gicd_icpendr7.write(|w| w.bits(value)),
            8 => gicd.gicd_icpendr8.write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the ICACTIVER register for an index.
fn set_icactiver(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_icactiver0.write(|w| w.bits(value)),
            1 => gicd.gicd_icactiver1.write(|w| w.bits(value)),
            2 => gicd.gicd_icactiver2.write(|w| w.bits(value)),
            3 => gicd.gicd_icactiver3.write(|w| w.bits(value)),
            4 => gicd.gicd_icactiver4.write(|w| w.bits(value)),
            5 => gicd.gicd_icactiver5.write(|w| w.bits(value)),
            6 => gicd.gicd_icactiver6.write(|w| w.bits(value)),
            7 => gicd.gicd_icactiver7.write(|w| w.bits(value)),
            8 => gicd.gicd_icactiver8.write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        }
    }
}

/// Reads the ISACTIVER register for an index.
fn isactiver(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_isactiver0.read().bits(),
            1 => gicd.gicd_isactiver1.read().bits(),
            2 => gicd.gicd_isactiver2.read().bits(),
            3 => gicd.gicd_isactiver3.read().bits(),
            4 => gicd.gicd_isactiver4.read().bits(),
            5 => gicd.gicd_isactiver5.read().bits(),
            6 => gicd.gicd_isactiver6.read().bits(),
            7 => gicd.gicd_isactiver7.read().bits(),
            8 => gicd.gicd_isactiver8.read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Reads the SPENDSGIR register for an index.
fn spendsgir(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_spendsgir0.read().bits(),
            1 => gicd.gicd_spendsgir1.read().bits(),
            2 => gicd.gicd_spendsgir2.read().bits(),
            3 => gicd.gicd_spendsgir3.read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the SPENDSGIR register for an index.
fn set_spendsgir(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_spendsgir0.write(|w| w.bits(value)),
            1 => gicd.gicd_spendsgir1.write(|w| w.bits(value)),
            2 => gicd.gicd_spendsgir2.write(|w| w.bits(value)),
            3 => gicd.gicd_spendsgir3.write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the CPENDSGIR register for an index.
fn set_cpendsgir(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_cpendsgir0.write(|w| w.bits(value)),
            1 => gicd.gicd_cpendsgir1.write(|w| w.bits(value)),
            2 => gicd.gicd_cpendsgir2.write(|w| w.bits(value)),
            3 => gicd.gicd_cpendsgir3.write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        }
    }
}

/// Returns the ICFGR register for an index.
fn icfgr(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_icfgr0.read().bits(),
            1 => gicd.gicd_icfgr1.read().bits(),
            2 => gicd.gicd_icfgr2.read().bits(),
            3 => gicd.gicd_icfgr3.read().bits(),
            4 => gicd.gicd_icfgr4.read().bits(),
            5 => gicd.gicd_icfgr5.read().bits(),
            6 => gicd.gicd_icfgr6.read().bits(),
            7 => gicd.gicd_icfgr7.read().bits(),
            8 => gicd.gicd_icfgr8.read().bits(),
            9 => gicd.gicd_icfgr9.read().bits(),
            10 => gicd.gicd_icfgr10.read().bits(),
            11 => gicd.gicd_icfgr11.read().bits(),
            12 => gicd.gicd_icfgr12.read().bits(),
            13 => gicd.gicd_icfgr13.read().bits(),
            14 => gicd.gicd_icfgr14.read().bits(),
            15 => gicd.gicd_icfgr15.read().bits(),
            16 => gicd.gicd_icfgr16.read().bits(),
            17 => gicd.gicd_icfgr17.read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the ICFGR register for an index.
fn set_icfgr(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_icfgr0.write(|w| w.bits(value)),
            1 => gicd.gicd_icfgr1.write(|w| w.bits(value)),
            2 => gicd.gicd_icfgr2.write(|w| w.bits(value)),
            3 => gicd.gicd_icfgr3.write(|w| w.bits(value)),
            4 => gicd.gicd_icfgr4.write(|w| w.bits(value)),
            5 => gicd.gicd_icfgr5.write(|w| w.bits(value)),
            6 => gicd.gicd_icfgr6.write(|w| w.bits(value)),
            7 => gicd.gicd_icfgr7.write(|w| w.bits(value)),
            8 => gicd.gicd_icfgr8.write(|w| w.bits(value)),
            9 => gicd.gicd_icfgr9.write(|w| w.bits(value)),
            10 => gicd.gicd_icfgr10.write(|w| w.bits(value)),
            11 => gicd.gicd_icfgr11.write(|w| w.bits(value)),
            12 => gicd.gicd_icfgr12.write(|w| w.bits(value)),
            13 => gicd.gicd_icfgr13.write(|w| w.bits(value)),
            14 => gicd.gicd_icfgr13.write(|w| w.bits(value)),
            15 => gicd.gicd_icfgr15.write(|w| w.bits(value)),
            16 => gicd.gicd_icfgr16.write(|w| w.bits(value)),
            17 => gicd.gicd_icfgr17.write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        }
    }
}

/// Reads the ITARGETSR register for an index.
fn itargetsr(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_itargetsr0.read().bits(),
            1 => gicd.gicd_itargetsr1.read().bits(),
            2 => gicd.gicd_itargetsr2.read().bits(),
            3 => gicd.gicd_itargetsr3.read().bits(),
            4 => gicd.gicd_itargetsr4.read().bits(),
            5 => gicd.gicd_itargetsr5.read().bits(),
            6 => gicd.gicd_itargetsr6.read().bits(),
            7 => gicd.gicd_itargetsr7.read().bits(),
            8 => gicd.gicd_itargetsr8.read().bits(),
            9 => gicd.gicd_itargetsr9.read().bits(),
            10 => gicd.gicd_itargetsr10.read().bits(),
            11 => gicd.gicd_itargetsr11.read().bits(),
            12 => gicd.gicd_itargetsr12.read().bits(),
            13 => gicd.gicd_itargetsr13.read().bits(),
            14 => gicd.gicd_itargetsr14.read().bits(),
            15 => gicd.gicd_itargetsr15.read().bits(),
            16 => gicd.gicd_itargetsr16.read().bits(),
            17 => gicd.gicd_itargetsr17.read().bits(),
            18 => gicd.gicd_itargetsr18.read().bits(),
            19 => gicd.gicd_itargetsr19.read().bits(),
            20 => gicd.gicd_itargetsr20.read().bits(),
            21 => gicd.gicd_itargetsr21.read().bits(),
            22 => gicd.gicd_itargetsr22.read().bits(),
            23 => gicd.gicd_itargetsr23.read().bits(),
            24 => gicd.gicd_itargetsr24.read().bits(),
            25 => gicd.gicd_itargetsr25.read().bits(),
            26 => gicd.gicd_itargetsr26.read().bits(),
            27 => gicd.gicd_itargetsr27.read().bits(),
            28 => gicd.gicd_itargetsr28.read().bits(),
            29 => gicd.gicd_itargetsr29.read().bits(),
            30 => gicd.gicd_itargetsr30.read().bits(),
            31 => gicd.gicd_itargetsr31.read().bits(),
            32 => gicd.gicd_itargetsr32.read().bits(),
            33 => gicd.gicd_itargetsr33.read().bits(),
            34 => gicd.gicd_itargetsr34.read().bits(),
            35 => gicd.gicd_itargetsr35.read().bits(),
            36 => gicd.gicd_itargetsr36.read().bits(),
            37 => gicd.gicd_itargetsr37.read().bits(),
            38 => gicd.gicd_itargetsr38.read().bits(),
            39 => gicd.gicd_itargetsr39.read().bits(),
            40 => gicd.gicd_itargetsr40.read().bits(),
            41 => gicd.gicd_itargetsr41.read().bits(),
            42 => gicd.gicd_itargetsr42.read().bits(),
            43 => gicd.gicd_itargetsr43.read().bits(),
            44 => gicd.gicd_itargetsr44.read().bits(),
            45 => gicd.gicd_itargetsr45.read().bits(),
            46 => gicd.gicd_itargetsr46.read().bits(),
            47 => gicd.gicd_itargetsr47.read().bits(),
            48 => gicd.gicd_itargetsr48.read().bits(),
            49 => gicd.gicd_itargetsr49.read().bits(),
            50 => gicd.gicd_itargetsr50.read().bits(),
            51 => gicd.gicd_itargetsr51.read().bits(),
            52 => gicd.gicd_itargetsr52.read().bits(),
            53 => gicd.gicd_itargetsr53.read().bits(),
            54 => gicd.gicd_itargetsr54.read().bits(),
            55 => gicd.gicd_itargetsr55.read().bits(),
            56 => gicd.gicd_itargetsr56.read().bits(),
            57 => gicd.gicd_itargetsr57.read().bits(),
            58 => gicd.gicd_itargetsr58.read().bits(),
            59 => gicd.gicd_itargetsr59.read().bits(),
            60 => gicd.gicd_itargetsr60.read().bits(),
            61 => gicd.gicd_itargetsr61.read().bits(),
            62 => gicd.gicd_itargetsr62.read().bits(),
            63 => gicd.gicd_itargetsr63.read().bits(),
            64 => gicd.gicd_itargetsr64.read().bits(),
            65 => gicd.gicd_itargetsr65.read().bits(),
            66 => gicd.gicd_itargetsr66.read().bits(),
            67 => gicd.gicd_itargetsr67.read().bits(),
            68 => gicd.gicd_itargetsr68.read().bits(),
            69 => gicd.gicd_itargetsr69.read().bits(),
            70 => gicd.gicd_itargetsr70.read().bits(),
            71 => gicd.gicd_itargetsr71.read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the ITARGETSR register for an index.
fn set_itargetsr(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            // Indexes 0..7 are read-only.
            8 => gicd.gicd_itargetsr8.write(|w| w.bits(value)),
            9 => gicd.gicd_itargetsr9.write(|w| w.bits(value)),
            10 => gicd.gicd_itargetsr10.write(|w| w.bits(value)),
            11 => gicd.gicd_itargetsr11.write(|w| w.bits(value)),
            12 => gicd.gicd_itargetsr12.write(|w| w.bits(value)),
            13 => gicd.gicd_itargetsr13.write(|w| w.bits(value)),
            14 => gicd.gicd_itargetsr14.write(|w| w.bits(value)),
            15 => gicd.gicd_itargetsr15.write(|w| w.bits(value)),
            16 => gicd.gicd_itargetsr16.write(|w| w.bits(value)),
            17 => gicd.gicd_itargetsr17.write(|w| w.bits(value)),
            18 => gicd.gicd_itargetsr18.write(|w| w.bits(value)),
            19 => gicd.gicd_itargetsr19.write(|w| w.bits(value)),
            20 => gicd.gicd_itargetsr20.write(|w| w.bits(value)),
            21 => gicd.gicd_itargetsr21.write(|w| w.bits(value)),
            22 => gicd.gicd_itargetsr22.write(|w| w.bits(value)),
            23 => gicd.gicd_itargetsr23.write(|w| w.bits(value)),
            24 => gicd.gicd_itargetsr24.write(|w| w.bits(value)),
            25 => gicd.gicd_itargetsr25.write(|w| w.bits(value)),
            26 => gicd.gicd_itargetsr26.write(|w| w.bits(value)),
            27 => gicd.gicd_itargetsr27.write(|w| w.bits(value)),
            28 => gicd.gicd_itargetsr28.write(|w| w.bits(value)),
            29 => gicd.gicd_itargetsr29.write(|w| w.bits(value)),
            30 => gicd.gicd_itargetsr30.write(|w| w.bits(value)),
            31 => gicd.gicd_itargetsr31.write(|w| w.bits(value)),
            32 => gicd.gicd_itargetsr32.write(|w| w.bits(value)),
            33 => gicd.gicd_itargetsr33.write(|w| w.bits(value)),
            34 => gicd.gicd_itargetsr34.write(|w| w.bits(value)),
            35 => gicd.gicd_itargetsr35.write(|w| w.bits(value)),
            36 => gicd.gicd_itargetsr36.write(|w| w.bits(value)),
            37 => gicd.gicd_itargetsr37.write(|w| w.bits(value)),
            38 => gicd.gicd_itargetsr38.write(|w| w.bits(value)),
            39 => gicd.gicd_itargetsr39.write(|w| w.bits(value)),
            40 => gicd.gicd_itargetsr40.write(|w| w.bits(value)),
            41 => gicd.gicd_itargetsr41.write(|w| w.bits(value)),
            42 => gicd.gicd_itargetsr42.write(|w| w.bits(value)),
            43 => gicd.gicd_itargetsr43.write(|w| w.bits(value)),
            44 => gicd.gicd_itargetsr44.write(|w| w.bits(value)),
            45 => gicd.gicd_itargetsr45.write(|w| w.bits(value)),
            46 => gicd.gicd_itargetsr46.write(|w| w.bits(value)),
            47 => gicd.gicd_itargetsr47.write(|w| w.bits(value)),
            48 => gicd.gicd_itargetsr48.write(|w| w.bits(value)),
            49 => gicd.gicd_itargetsr49.write(|w| w.bits(value)),
            50 => gicd.gicd_itargetsr50.write(|w| w.bits(value)),
            51 => gicd.gicd_itargetsr51.write(|w| w.bits(value)),
            52 => gicd.gicd_itargetsr52.write(|w| w.bits(value)),
            53 => gicd.gicd_itargetsr53.write(|w| w.bits(value)),
            54 => gicd.gicd_itargetsr54.write(|w| w.bits(value)),
            55 => gicd.gicd_itargetsr55.write(|w| w.bits(value)),
            56 => gicd.gicd_itargetsr56.write(|w| w.bits(value)),
            57 => gicd.gicd_itargetsr57.write(|w| w.bits(value)),
            58 => gicd.gicd_itargetsr58.write(|w| w.bits(value)),
            59 => gicd.gicd_itargetsr59.write(|w| w.bits(value)),
            60 => gicd.gicd_itargetsr60.write(|w| w.bits(value)),
            61 => gicd.gicd_itargetsr61.write(|w| w.bits(value)),
            62 => gicd.gicd_itargetsr62.write(|w| w.bits(value)),
            63 => gicd.gicd_itargetsr63.write(|w| w.bits(value)),
            64 => gicd.gicd_itargetsr64.write(|w| w.bits(value)),
            65 => gicd.gicd_itargetsr65.write(|w| w.bits(value)),
            66 => gicd.gicd_itargetsr66.write(|w| w.bits(value)),
            67 => gicd.gicd_itargetsr67.write(|w| w.bits(value)),
            68 => gicd.gicd_itargetsr68.write(|w| w.bits(value)),
            69 => gicd.gicd_itargetsr69.write(|w| w.bits(value)),
            70 => gicd.gicd_itargetsr70.write(|w| w.bits(value)),
            71 => gicd.gicd_itargetsr71.write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        }
    }
}

/// Reads the IPRIORITYR register for an index.
fn ipriorityr(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_ipriorityr0.read().bits(),
            1 => gicd.gicd_ipriorityr1.read().bits(),
            2 => gicd.gicd_ipriorityr2.read().bits(),
            3 => gicd.gicd_ipriorityr3.read().bits(),
            4 => gicd.gicd_ipriorityr4.read().bits(),
            5 => gicd.gicd_ipriorityr5.read().bits(),
            6 => gicd.gicd_ipriorityr6.read().bits(),
            7 => gicd.gicd_ipriorityr7.read().bits(),
            8 => gicd.gicd_ipriorityr8.read().bits(),
            9 => gicd.gicd_ipriorityr9.read().bits(),
            10 => gicd.gicd_ipriorityr10.read().bits(),
            11 => gicd.gicd_ipriorityr11.read().bits(),
            12 => gicd.gicd_ipriorityr12.read().bits(),
            13 => gicd.gicd_ipriorityr13.read().bits(),
            14 => gicd.gicd_ipriorityr14.read().bits(),
            15 => gicd.gicd_ipriorityr15.read().bits(),
            16 => gicd.gicd_ipriorityr16.read().bits(),
            17 => gicd.gicd_ipriorityr17.read().bits(),
            18 => gicd.gicd_ipriorityr18.read().bits(),
            19 => gicd.gicd_ipriorityr19.read().bits(),
            20 => gicd.gicd_ipriorityr20.read().bits(),
            21 => gicd.gicd_ipriorityr21.read().bits(),
            22 => gicd.gicd_ipriorityr22.read().bits(),
            23 => gicd.gicd_ipriorityr23.read().bits(),
            24 => gicd.gicd_ipriorityr24.read().bits(),
            25 => gicd.gicd_ipriorityr25.read().bits(),
            26 => gicd.gicd_ipriorityr26.read().bits(),
            27 => gicd.gicd_ipriorityr27.read().bits(),
            28 => gicd.gicd_ipriorityr28.read().bits(),
            29 => gicd.gicd_ipriorityr29.read().bits(),
            30 => gicd.gicd_ipriorityr30.read().bits(),
            31 => gicd.gicd_ipriorityr31.read().bits(),
            32 => gicd.gicd_ipriorityr32.read().bits(),
            33 => gicd.gicd_ipriorityr33.read().bits(),
            34 => gicd.gicd_ipriorityr34.read().bits(),
            35 => gicd.gicd_ipriorityr35.read().bits(),
            36 => gicd.gicd_ipriorityr36.read().bits(),
            37 => gicd.gicd_ipriorityr37.read().bits(),
            38 => gicd.gicd_ipriorityr38.read().bits(),
            39 => gicd.gicd_ipriorityr39.read().bits(),
            40 => gicd.gicd_ipriorityr40.read().bits(),
            41 => gicd.gicd_ipriorityr41.read().bits(),
            42 => gicd.gicd_ipriorityr42.read().bits(),
            43 => gicd.gicd_ipriorityr43.read().bits(),
            44 => gicd.gicd_ipriorityr44.read().bits(),
            45 => gicd.gicd_ipriorityr45.read().bits(),
            46 => gicd.gicd_ipriorityr46.read().bits(),
            47 => gicd.gicd_ipriorityr47.read().bits(),
            48 => gicd.gicd_ipriorityr48.read().bits(),
            49 => gicd.gicd_ipriorityr49.read().bits(),
            50 => gicd.gicd_ipriorityr50.read().bits(),
            51 => gicd.gicd_ipriorityr51.read().bits(),
            52 => gicd.gicd_ipriorityr52.read().bits(),
            53 => gicd.gicd_ipriorityr53.read().bits(),
            54 => gicd.gicd_ipriorityr54.read().bits(),
            55 => gicd.gicd_ipriorityr55.read().bits(),
            56 => gicd.gicd_ipriorityr56.read().bits(),
            57 => gicd.gicd_ipriorityr57.read().bits(),
            58 => gicd.gicd_ipriorityr58.read().bits(),
            59 => gicd.gicd_ipriorityr59.read().bits(),
            60 => gicd.gicd_ipriorityr60.read().bits(),
            61 => gicd.gicd_ipriorityr61.read().bits(),
            62 => gicd.gicd_ipriorityr62.read().bits(),
            63 => gicd.gicd_ipriorityr63.read().bits(),
            64 => gicd.gicd_ipriorityr64.read().bits(),
            65 => gicd.gicd_ipriorityr65.read().bits(),
            66 => gicd.gicd_ipriorityr66.read().bits(),
            67 => gicd.gicd_ipriorityr67.read().bits(),
            68 => gicd.gicd_ipriorityr68.read().bits(),
            69 => gicd.gicd_ipriorityr69.read().bits(),
            70 => gicd.gicd_ipriorityr70.read().bits(),
            71 => gicd.gicd_ipriorityr71.read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the IPRIORITYR register for an index.
fn set_ipriorityr(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_ipriorityr0.write(|w| w.bits(value)),
            1 => gicd.gicd_ipriorityr1.write(|w| w.bits(value)),
            2 => gicd.gicd_ipriorityr2.write(|w| w.bits(value)),
            3 => gicd.gicd_ipriorityr3.write(|w| w.bits(value)),
            4 => gicd.gicd_ipriorityr4.write(|w| w.bits(value)),
            5 => gicd.gicd_ipriorityr5.write(|w| w.bits(value)),
            6 => gicd.gicd_ipriorityr6.write(|w| w.bits(value)),
            7 => gicd.gicd_ipriorityr7.write(|w| w.bits(value)),
            8 => gicd.gicd_ipriorityr8.write(|w| w.bits(value)),
            9 => gicd.gicd_ipriorityr9.write(|w| w.bits(value)),
            10 => gicd.gicd_ipriorityr10.write(|w| w.bits(value)),
            11 => gicd.gicd_ipriorityr11.write(|w| w.bits(value)),
            12 => gicd.gicd_ipriorityr12.write(|w| w.bits(value)),
            13 => gicd.gicd_ipriorityr13.write(|w| w.bits(value)),
            14 => gicd.gicd_ipriorityr14.write(|w| w.bits(value)),
            15 => gicd.gicd_ipriorityr15.write(|w| w.bits(value)),
            16 => gicd.gicd_ipriorityr16.write(|w| w.bits(value)),
            17 => gicd.gicd_ipriorityr17.write(|w| w.bits(value)),
            18 => gicd.gicd_ipriorityr18.write(|w| w.bits(value)),
            19 => gicd.gicd_ipriorityr19.write(|w| w.bits(value)),
            20 => gicd.gicd_ipriorityr20.write(|w| w.bits(value)),
            21 => gicd.gicd_ipriorityr21.write(|w| w.bits(value)),
            22 => gicd.gicd_ipriorityr22.write(|w| w.bits(value)),
            23 => gicd.gicd_ipriorityr23.write(|w| w.bits(value)),
            24 => gicd.gicd_ipriorityr24.write(|w| w.bits(value)),
            25 => gicd.gicd_ipriorityr25.write(|w| w.bits(value)),
            26 => gicd.gicd_ipriorityr26.write(|w| w.bits(value)),
            27 => gicd.gicd_ipriorityr27.write(|w| w.bits(value)),
            28 => gicd.gicd_ipriorityr28.write(|w| w.bits(value)),
            29 => gicd.gicd_ipriorityr29.write(|w| w.bits(value)),
            30 => gicd.gicd_ipriorityr30.write(|w| w.bits(value)),
            31 => gicd.gicd_ipriorityr31.write(|w| w.bits(value)),
            32 => gicd.gicd_ipriorityr32.write(|w| w.bits(value)),
            33 => gicd.gicd_ipriorityr33.write(|w| w.bits(value)),
            34 => gicd.gicd_ipriorityr34.write(|w| w.bits(value)),
            35 => gicd.gicd_ipriorityr35.write(|w| w.bits(value)),
            36 => gicd.gicd_ipriorityr36.write(|w| w.bits(value)),
            37 => gicd.gicd_ipriorityr37.write(|w| w.bits(value)),
            38 => gicd.gicd_ipriorityr38.write(|w| w.bits(value)),
            39 => gicd.gicd_ipriorityr39.write(|w| w.bits(value)),
            40 => gicd.gicd_ipriorityr40.write(|w| w.bits(value)),
            41 => gicd.gicd_ipriorityr41.write(|w| w.bits(value)),
            42 => gicd.gicd_ipriorityr42.write(|w| w.bits(value)),
            43 => gicd.gicd_ipriorityr43.write(|w| w.bits(value)),
            44 => gicd.gicd_ipriorityr44.write(|w| w.bits(value)),
            45 => gicd.gicd_ipriorityr45.write(|w| w.bits(value)),
            46 => gicd.gicd_ipriorityr46.write(|w| w.bits(value)),
            47 => gicd.gicd_ipriorityr47.write(|w| w.bits(value)),
            48 => gicd.gicd_ipriorityr48.write(|w| w.bits(value)),
            49 => gicd.gicd_ipriorityr49.write(|w| w.bits(value)),
            50 => gicd.gicd_ipriorityr50.write(|w| w.bits(value)),
            51 => gicd.gicd_ipriorityr51.write(|w| w.bits(value)),
            52 => gicd.gicd_ipriorityr52.write(|w| w.bits(value)),
            53 => gicd.gicd_ipriorityr53.write(|w| w.bits(value)),
            54 => gicd.gicd_ipriorityr54.write(|w| w.bits(value)),
            55 => gicd.gicd_ipriorityr55.write(|w| w.bits(value)),
            56 => gicd.gicd_ipriorityr56.write(|w| w.bits(value)),
            57 => gicd.gicd_ipriorityr57.write(|w| w.bits(value)),
            58 => gicd.gicd_ipriorityr58.write(|w| w.bits(value)),
            59 => gicd.gicd_ipriorityr59.write(|w| w.bits(value)),
            60 => gicd.gicd_ipriorityr60.write(|w| w.bits(value)),
            61 => gicd.gicd_ipriorityr61.write(|w| w.bits(value)),
            62 => gicd.gicd_ipriorityr62.write(|w| w.bits(value)),
            63 => gicd.gicd_ipriorityr63.write(|w| w.bits(value)),
            64 => gicd.gicd_ipriorityr64.write(|w| w.bits(value)),
            65 => gicd.gicd_ipriorityr65.write(|w| w.bits(value)),
            66 => gicd.gicd_ipriorityr66.write(|w| w.bits(value)),
            67 => gicd.gicd_ipriorityr67.write(|w| w.bits(value)),
            68 => gicd.gicd_ipriorityr68.write(|w| w.bits(value)),
            69 => gicd.gicd_ipriorityr69.write(|w| w.bits(value)),
            70 => gicd.gicd_ipriorityr70.write(|w| w.bits(value)),
            71 => gicd.gicd_ipriorityr71.write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        }
    }
}

/// Returns the IGROUPR register for an index.
fn igroupr(index: usize) -> u32 {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_igroupr0.read().bits(),
            1 => gicd.gicd_igroupr1.read().bits(),
            2 => gicd.gicd_igroupr2.read().bits(),
            3 => gicd.gicd_igroupr3.read().bits(),
            4 => gicd.gicd_igroupr4.read().bits(),
            5 => gicd.gicd_igroupr5.read().bits(),
            6 => gicd.gicd_igroupr6.read().bits(),
            7 => gicd.gicd_igroupr7.read().bits(),
            8 => gicd.gicd_igroupr8.read().bits(),
            _ => panic!("Index out of range."),
        }
    }
}

/// Sets the IGROUPR register for an index.
fn set_igroupr(index: usize, value: u32) {
    unsafe {
        let gicd = &(*pac::GICD::ptr());
        match index {
            0 => gicd.gicd_igroupr0.write(|w| w.bits(value)),
            1 => gicd.gicd_igroupr1.write(|w| w.bits(value)),
            2 => gicd.gicd_igroupr2.write(|w| w.bits(value)),
            3 => gicd.gicd_igroupr3.write(|w| w.bits(value)),
            4 => gicd.gicd_igroupr4.write(|w| w.bits(value)),
            5 => gicd.gicd_igroupr5.write(|w| w.bits(value)),
            6 => gicd.gicd_igroupr6.write(|w| w.bits(value)),
            7 => gicd.gicd_igroupr7.write(|w| w.bits(value)),
            8 => gicd.gicd_igroupr8.write(|w| w.bits(value)),
            _ => panic!("Index out of range."),
        }
    }
}

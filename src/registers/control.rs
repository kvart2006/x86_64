//! Functions to read and write control registers.

pub use super::model_specific::{Efer, EferFlags};

use bitflags::bitflags;

/// CR0
/// Contains system control flags that control operating mode and states of the CPU.
#[derive(Debug)]
pub struct Cr0;

bitflags! {
    /// Configuration flags of the Cr0 register.
    pub struct Cr0Flags: u64 {
        /// Enables protected mode.
        const PROTECTED_MODE_ENABLE = 1 << 0;
        /// Enables monitoring of the coprocessor, typical for x87 instructions.
        ///
        /// Controls together with the `TASK_SWITCHED` flag whether a `wait` or `fwait`
        /// instruction should cause a device-not-available exception.
        const MONITOR_COPROCESSOR = 1 << 1;
        /// Force all x87 and MMX instructions to cause an exception.
        const EMULATE_COPROCESSOR = 1 << 2;
        /// Automatically set to 1 on _hardware_ task switch.
        ///
        /// This flags allows lazily saving x87/MMX/SSE instructions on hardware context switches.
        const TASK_SWITCHED = 1 << 3;
        /// Enables the native error reporting mechanism for x87 FPU errors.
        const NUMERIC_ERROR = 1 << 5;
        /// Controls whether supervisor-level writes to read-only pages are inhibited.
        ///
        /// When set, it is not possible to write to read-only pages from ring 0.
        const WRITE_PROTECT = 1 << 16;
        /// Enables automatic alignment checking.
        const ALIGNMENT_MASK = 1 << 18;
        /// Ignored. Used to control write-back/write-through cache strategy on older CPUs.
        const NOT_WRITE_THROUGH = 1 << 29;
        /// Disables internal caches (only for some cases).
        const CACHE_DISABLE = 1 << 30;
        /// Enables page translation.
        const PAGING = 1 << 31;
    }
}

/// CR2
/// Contains the page-fault linear address (PFLA) (the linear address that caused a page fault).
/// When page fault occurs, the CPU sets this register to the accessed address.
#[derive(Debug)]
pub struct Cr2;

/// CR3
/// Contains the physical address of the level 4 page table.
#[derive(Debug)]
pub struct Cr3;

bitflags! {
    /// Controls cache settings for the level 4 page table.
    pub struct Cr3Flags: u64 {
        /// Use a writethrough cache policy for the P4 table (else a writeback policy is used).
        const PAGE_LEVEL_WRITETHROUGH = 1 << 3;
        /// Disable caching for the P4 table.
        const PAGE_LEVEL_CACHE_DISABLE = 1 << 4;
    }
}
/// CR4
/// Contains a group of flags that enable several architectural extensions, 
/// and indicate operating system or executive support for specific processor capabilities.
#[derive(Debug)]
pub struct Cr4;

bitflags! {
    /// Configuration flags of the Cr0 register.
    pub struct Cr4Flags: u64 {
        /// VME
        /// If set, enables support for the virtual interrupt flag (VIF) in virtual-8086 mode.
        const VIRTUAL_8086_MODE_EXTENSIONS = 1 << 0;
        /// PVI 
        /// If set, enables support for the virtual interrupt flag (VIF) in protected mode.
        const PROTECTED_MODE_VIRTUAL_INTERRUPTS = 1 << 1;
        /// TSD
        /// If set, RDTSC instruction can only be executed when in ring 0, 
        /// otherwise RDTSC can be used at any privilege level.
        const TIME_STAMP_DISABLE = 1 << 2;
        /// DE
        /// If set, enables debug register based breaks on I/O space access.
        const DEBUGGING_EXTENSIONS = 1 << 3;
        /// PSE
        /// If unset, page size is 4 KiB, else page size is increased to 4 MiB
        /// If PAE is enabled or the processor is in x86-64 long mode this bit is ignored.
        const PAGE_SIZE_EXTENSIONS = 1 << 4;
        /// PAE
        /// If set, changes page table layout to translate 32-bit virtual addresses into 
        /// extended 36-bit physical addresses.
        const PHYSICAL_ADDRESS_EXTENSIONS = 1 << 5;
        /// MCE
        /// If set, enables machine check interrupts to occur.
        const MACHINE_CHECK_EXTENSIONS = 1 << 6;
        /// PGE
        /// If set, address translations (PDE or PTE records) may be shared between address spaces.
        const PAGE_GLOBAL_ENABLED = 1 << 7;
        /// PCE
        /// If set, RDPMC can be executed at any privilege level, else RDPMC can only be used in ring 0.
        const PERFORMANCE_MONITORING_COUNTER_ENABLE = 1 << 8;
        /// OSFXSR
        /// If set, enables Streaming SIMD Extensions (SSE) instructions and fast FPU save & restore.
        const OS_SUPPORT_FOR_FXSAVE_AND_FXRSTOR_INSTRUCTIONS = 1 << 9;
        /// OSXMMEXCPT
        /// If set, enables unmasked SSE exceptions.
        const OS_SUPPORT_FOR_UNMASKED_SIMD_FLOATING_POINT_EXCEPTIONS = 1 << 10;
        /// UMIP
        /// If set, the SGDT, SIDT, SLDT, SMSW and STR instructions cannot be executed if CPL > 0.
        const USER_MODE_INSTRUCTION_PREVENTION = 1 << 11;
        /// LA57
        /// If set, enables 5-Level Paging.
        const LA57 = 1 << 12;
        /// VMXE
        ///
        const VIRTUAL_MACHINE_EXTENSIONS_ENABLE = 1 << 13;
        /// SMXE
        ///
        const SAFER_MODE_EXTENSIONS_ENABLED = 1 << 14;
        /// FSGSBASE
        ///
        const FSGSBASE = 1 << 16;
        /// PCIDE
        /// If set, enables process-context identifiers (PCIDs).
        const PCID_ENABLE = 1 << 17;
        /// OSXSAVE
        ///
        const XSAVE_AND_PROCESSOR_EXTENDED_STATES_ENABLE = 1 << 18;
        /// SMEP
        /// If set, execution of code in a higher ring generates a fault.
        const SUPERVISOR_MODE_EXECUTION_PROTECTION_ENABLE = 1 << 20;
        /// SMAP
        /// If set, access of data in a higher ring generates a fault.
        const SUPERVISOR_MODE_ACCESS_PREVENTION_ENABLE = 1 << 21;
        /// PKE
        /// 
        const PROTECTION_KEY_ENABLE = 1 << 22;
    }
}

#[cfg(target_arch = "x86_64")]
mod x86_64 {
    use super::*;
    use crate::structures::paging::PhysFrame;
    use crate::{PhysAddr, VirtAddr};

    impl Cr0 {
        /// Read the current set of CR0 flags.
        pub fn read() -> Cr0Flags {
            Cr0Flags::from_bits_truncate(Self::read_raw())
        }

        /// Read the current raw CR0 value.
        pub fn read_raw() -> u64 {
            let value: u64;
            unsafe {
                asm!("mov %cr0, $0" : "=r" (value));
            }
            value
        }

        /// Write CR0 flags.
        ///
        /// Preserves the value of reserved fields. Unsafe because it's possible to violate memory
        /// safety by e.g. disabling paging.
        pub unsafe fn write(flags: Cr0Flags) {
            let old_value = Self::read_raw();
            let reserved = old_value & !(Cr0Flags::all().bits());
            let new_value = reserved | flags.bits();

            Self::write_raw(new_value);
        }

        /// Write raw CR0 flags.
        ///
        /// Does _not_ preserve any values, including reserved fields. Unsafe because it's possible to violate memory
        /// safety by e.g. disabling paging.
        pub unsafe fn write_raw(value: u64) {
            asm!("mov $0, %cr0" :: "r" (value) : "memory")
        }

        /// Updates CR0 flags.
        ///
        /// Preserves the value of reserved fields. Unsafe because it's possible to violate memory
        /// safety by e.g. disabling paging.
        pub unsafe fn update<F>(f: F)
        where
            F: FnOnce(&mut Cr0Flags),
        {
            let mut flags = Self::read();
            f(&mut flags);
            Self::write(flags);
        }
    }

    impl Cr2 {
        /// Read the current page fault linear address from the CR3 register.
        pub fn read() -> VirtAddr {
            let value: u64;
            unsafe {
                asm!("mov %cr2, $0" : "=r" (value));
            }
            VirtAddr::new(value)
        }
    }

    impl Cr3 {
        /// Read the current P4 table address from the CR3 register.
        pub fn read() -> (PhysFrame, Cr3Flags) {
            let value: u64;
            unsafe {
                asm!("mov %cr3, $0" : "=r" (value));
            }
            let flags = Cr3Flags::from_bits_truncate(value);
            let addr = PhysAddr::new(value & 0x_000f_ffff_ffff_f000);
            let frame = PhysFrame::containing_address(addr);
            (frame, flags)
        }

        /// Write a new P4 table address into the CR3 register.
        ///
        /// ## Safety
        /// Changing the level 4 page table is unsafe, because it's possible to violate memory safety by
        /// changing the page mapping.
        pub unsafe fn write(frame: PhysFrame, flags: Cr3Flags) {
            let addr = frame.start_address();
            let value = addr.as_u64() | flags.bits();
            asm!("mov $0, %cr3" :: "r" (value) : "memory")
        }
    }

    impl Cr4 {

    }
}

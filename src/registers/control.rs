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
        /// Protection Enable
        ///
        /// Enables protected mode.
        const PE = 1 << 0;
        /// Monitor Coprocessor.
        ///
        /// Controls together with the `TASK_SWITCHED` flag whether a `wait` or `fwait`
        /// instruction should cause a device-not-available exception.
        const MP = 1 << 1;
        /// Emulation
        ///
        /// Indicates that the processor does not have an internal or external x87 FPU when set; 
        /// indicates an x87 FPU is present when clear..
        const EM = 1 << 2;
        /// Task Switched.
        ///
        /// This flags allows lazily saving x87/MMX/SSE instructions on hardware context switches.
        const TS = 1 << 3;
        /// Extension Type
        ///
        /// Reserved in the Pentium 4, Intel Xeon, P6 family, and Pentium processors. 
        const ET = 1 << 4;
        /// Numeric Error
        /// Enables the native error reporting mechanism for x87 FPU errors.
        const NE = 1 << 5;
        /// Controls whether supervisor-level writes to read-only pages are inhibited.
        ///
        /// When set, it is not possible to write to read-only pages from ring 0.
        const WP = 1 << 16;
        /// Alignment Mask
        ///
        /// Enables automatic alignment checking.
        const AM = 1 << 18;
        /// Not Write-through
        /// 
        /// Ignored. Used to control write-back/write-through cache strategy on older CPUs.
        const NW = 1 << 29;
        /// Cache Disable
        /// 
        /// Disables internal caches (only for some cases).
        const CD = 1 << 30;
        /// Paging
        /// 
        /// Enables page translation.
        const PG = 1 << 31;
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
        /// Page-level Cache Disable
        ///
        /// Use a write through cache policy for the P4 table (else a writeback policy is used).
        const PCD = 1 << 3;
        /// Page-level Write-Through
        /// 
        /// Controls the memory type used to access the first paging structure of the current 
        /// paging-structure hierarchy.
        const PWT = 1 << 4;
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
        /// Virtual-8086 Mode Extensions
        ///
        /// If set, enables support for the virtual interrupt flag (VIF) in virtual-8086 mode.
        const VME = 1 << 0;
        /// Protected-Mode Virtual Interrupts
        ///
        /// If set, enables support for the virtual interrupt flag (VIF) in protected mode.
        const PVI = 1 << 1;
        /// Time Stamp Disable
        ///
        /// If set, RDTSC instruction can only be executed when in ring 0, 
        /// otherwise RDTSC can be used at any privilege level.
        const TSD = 1 << 2;
        /// Debugging Extensions
        ///
        /// If set, enables debug register based breaks on I/O space access.
        const DE = 1 << 3;
        /// Page Size Extensions
        ///
        /// If unset, page size is 4 KiB, else page size is increased to 4 MiB
        /// If PAE is enabled or the processor is in x86-64 long mode this bit is ignored.
        const PSE = 1 << 4;
        /// Physical Address Extension
        ///
        /// If set, changes page table layout to translate 32-bit virtual addresses into 
        /// extended 36-bit physical addresses.
        const PAE = 1 << 5;
        /// Machine-Check Enable
        ///
        /// If set, enables machine check interrupts to occur.
        const MCE = 1 << 6;
        /// Page Global Enable
        ///
        /// If set, address translations (PDE or PTE records) may be shared between address spaces.
        const PGE = 1 << 7;
        /// Performance-Monitoring Counter Enable
        ///
        /// If set, RDPMC can be executed at any privilege level, else RDPMC can only be used in ring 0.
        const PCE = 1 << 8;
        /// Operating System Support for FXSAVE and FXRSTOR instructions
        ///
        /// If set, enables Streaming SIMD Extensions (SSE) instructions and fast FPU save & restore.
        const OSFXSR = 1 << 9;
        /// Operating System Support for Unmasked SIMD Floating-Point Exceptions
        ///
        /// If set, enables unmasked SSE exceptions.
        const OSXMMEXCPT = 1 << 10;
        /// User-Mode Instruction Prevention
        ///
        /// If set, the SGDT, SIDT, SLDT, SMSW and STR instructions cannot be executed if CPL > 0.
        const UMIP = 1 << 11;
        /// LA57
        ///
        /// If set, enables 5-Level Paging.
        const LA57 = 1 << 12;
        /// VMX-Enable Bit
        ///
        /// Enables VMX operation when set.
        const VMXE = 1 << 13;
        /// SMX-Enable Bit
        ///
        /// Enables SMX operation when set.
        const SMXE = 1 << 14;
        /// FSGSBASE-Enable Bit
        ///
        /// Enables the instructions RDFSBASE, RDGSBASE, WRFSBASE, and WRGSBASE.
        const FSGSBASE = 1 << 16;
        /// PCID-Enable Bit
        ///
        /// If set, enables process-context identifiers (PCIDs).
        const PCIDE = 1 << 17;
        /// XSAVE and Processor Extended States-Enable Bit
        ///
        const OSXSAVE = 1 << 18;
        /// SMEP-Enable Bit
        ///
        /// If set, execution of code in a higher ring generates a fault.
        const SMEP = 1 << 20;
        /// SMAP-Enable Bit
        ///
        /// If set, access of data in a higher ring generates a fault.
        const SMAP = 1 << 21;
        /// Protection-Key-Enable Bit
        /// 
        /// Enables 4-level paging to associate each linear address with a protection key.
        const PKE = 1 << 22;
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
        /// Read the current set of CR0 flags.
        pub fn read() -> Cr4Flags {
            Cr4Flags::from_bits_truncate(Self::read_raw())
        }

        /// Read the current raw CR0 value.
        pub fn read_raw() -> u64 {
            let value: u64;
            unsafe {
                asm!("mov %cr4, $0" : "=r" (value));
            }
            value
        }

        /// Write CR4 flags.
        ///
        /// Preserves the value of reserved fields. Unsafe because it's possible to violate memory
        /// safety by e.g. disabling paging.
        pub unsafe fn write(flags: Cr4Flags) {
            let old_value = Self::read_raw();
            let reserved = old_value & !(Cr0Flags::all().bits());
            let new_value = reserved | flags.bits();

            Self::write_raw(new_value);
        }

        /// Write raw CR4 flags.
        ///
        /// Does _not_ preserve any values, including reserved fields. Unsafe because it's possible to violate memory
        /// safety by e.g. disabling paging.
        pub unsafe fn write_raw(value: u64) {
            asm!("mov $0, %cr4" :: "r" (value) : "memory")
        }

        /// Updates CR4 flags.
        ///
        /// Preserves the value of reserved fields. Unsafe because it's possible to violate memory
        /// safety by e.g. disabling paging.
        pub unsafe fn update<F>(f: F)
        where
            F: FnOnce(&mut Cr4Flags),
        {
            let mut flags = Self::read();
            f(&mut flags);
            Self::write(flags);
        }
    }
}

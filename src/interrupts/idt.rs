use x86_64::PrivilegeLevel;
use x86_64::VirtAddr;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::instructions::segmentation::{CS, Segment};

use bit_field::BitField;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Entry {
    // The lower bits of the pointer to the handler function.
    pointer_low:  u16,
    //Selector of a code segment in the GDT.
    gdt_selector: SegmentSelector,
    // See below
    options:      EntryOptions,
    // The middle bits of the pointer to the handler function.
    pointer_mid:  u16,
    // The remaining bits of the pointer to the handler function.
    pointer_rem:  u32,
    _reserved:    u32,
}

pub type HandlerFunc = extern "C" fn() -> !;

impl Entry {
    fn new(gdt_selector: SegmentSelector, handler: HandlerFunc) -> Self {
        let pointer = handler as u64;
        Entry {
            gdt_selector: gdt_selector,
            pointer_low:  pointer as u16,
            options:      EntryOptions::new(),
            pointer_mid:  (pointer >> 16) as u16,
            pointer_rem:  (pointer >> 32) as u32,
            _reserved:    0,
        }
    }

    fn missing() -> Self {
        Entry {
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            pointer_low:  0,
            options:      EntryOptions::minimal(),
            pointer_mid:  0,
            pointer_rem:  0,
            _reserved:    0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EntryOptions(u16);

impl EntryOptions {
    fn minimal() -> Self {
        let mut options = 0;
        options.set_bits(9..12, 0b111); // 'must-be-one' bits
        EntryOptions(options)
    }

    fn new() -> Self {
        let mut options = Self::minimal();
        options.set_present(true).disable_interrupts(true);
        options
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }

    pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        self.0.set_bits(13..15, dpl);
        self
    }

    pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0.set_bits(0..3, index);
        self
    }
}

pub struct Idt([Entry; 16]);

impl Idt {
    pub fn new() -> Idt {
        Idt([Entry::missing(); 16])
    }

    /// The method overwrites the specified entry with the given handler function
    pub fn set_handler(&mut self, entry: u8, handler: HandlerFunc) {
        // function of the x86_64 crate to get the current code segment descriptor.
        self.0[entry as usize] = Entry::new(CS::get_reg(), handler);
    }

    pub fn set_handler_with_options(&mut self, entry: u8, handler: HandlerFunc,
        options: EntryOptions) {
        self.set_handler(entry, handler);
        // NOTE(vf): a bit strange api tbh
        self.0[entry as usize].options = options;
    }

    /// Load our table to the CPU
    /// The referenced IDT must be valid as long as our kernel runs.
    pub fn load(&'static self) {
        use x86_64::instructions::tables::{DescriptorTablePointer, lidt};
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            // Virtual start address of the table.
            base: VirtAddr::new(self as *const _ as u64),
            // The maximum addressable byte in the table
            limit: (size_of::<Self>() - 1) as u16,
        };

        // register table using lidt instruction:
        // https://www.felixcloutier.com/x86/lgdt:lidt
        unsafe {lidt(&ptr)};
    }
}

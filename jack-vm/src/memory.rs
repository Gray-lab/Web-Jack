use wasm_bindgen_test::console_log;

use crate::{
    charmap::CharMap,
    parser::{Offset, Segment},
};
use std::ops::{Index, IndexMut};

pub type WordSize = i16;

// Memory sizes
pub(crate) const WORDSIZE: WordSize = 16;
const RAM_SIZE: WordSize = 16384;
const RAM_MAX_INDEX: WordSize = RAM_SIZE - 1;
const DISPLAY_MAX_INDEX: WordSize = 24575;
pub(crate) const DISPLAY_WIDTH: WordSize = 512;
pub(crate) const DISPLAY_HEIGHT: WordSize = 256;
pub(crate) const KEYBOARD_MEM: WordSize = 24576;
// Segment pointer locations
pub(crate) const SP: WordSize = 0;
pub(crate) const LCL: WordSize = 1;
pub(crate) const ARG: WordSize = 2;
pub(crate) const THIS: WordSize = 3;
pub(crate) const THAT: WordSize = 4;
const STATIC: WordSize = 15;
const STATIC_MAX: WordSize = 255;
const TEMP: WordSize = 5;
const TEMP_MAX: WordSize = 12;

struct HeapAllocation {
    pointer: WordSize,
    size: WordSize,
    status: MemoryStatus,
}

#[derive(PartialEq, Eq)]
enum MemoryStatus {
    Used,
    Free,
}

impl HeapAllocation {
    fn new(pointer: WordSize, size: WordSize) -> HeapAllocation {
        HeapAllocation {
            pointer,
            size,
            status: MemoryStatus::Used,
        }
    }

    fn combine(piece1: HeapAllocation, piece2: HeapAllocation) -> HeapAllocation {
        // must be adjacent and free
        // combine sizes and return the lower value pointer
        let new_pointer = i16::min(piece1.pointer, piece2.pointer);
        let new_size = piece1.size + piece2.size;
        HeapAllocation {
            pointer: new_pointer,
            size: new_size,
            status: MemoryStatus::Free,
        }
    }
}

/**
 * Memory array:
 * 0-16383 16 bit main memory (0x0000-0x3fff)
 * 16384-24575 16 bit screen (0x4000-0x5fff) -> pixel (r, c) is mapped onto the c%16 bit of the
 * 16 bit word stored at Screen \[r * 32 + c / 16\]
 * This needs to be exposed to javascript to allow for screen display
 * 24576 is 16 bit value for keyboard press (0x6000)
 * This needs to be updated continuously to allow for user input
 */
pub struct Memory {
    ram: MemoryVec,
    display: MemoryVec,
    pub keyboard: WordSize,
    pub cursor_line: WordSize,
    pub cursor_col: WordSize,
    pub screen_color: WordSize,
    pub char_map: CharMap,
    heap_alloc: Vec<HeapAllocation>,
    pub display_updated: bool,
}

struct MemoryVec(Vec<WordSize>);

impl MemoryVec {
    fn new(vector: Vec<WordSize>) -> MemoryVec {
        MemoryVec(vector)
    }

    fn len(&self) -> WordSize {
        self.0.len() as WordSize
    }

    fn as_ptr(&self) -> *const WordSize {
        self.0.as_ptr()
    }

    fn fill(&mut self, value: WordSize) {
        self.0.iter_mut().for_each(|m| *m = value);
    }
}

impl Index<WordSize> for MemoryVec {
    type Output = WordSize;
    fn index(&self, index: WordSize) -> &Self::Output {
        &(self.0[index as usize]) as &Self::Output
    }
}

impl IndexMut<WordSize> for MemoryVec {
    fn index_mut(&mut self, index: WordSize) -> &mut Self::Output {
        &mut (self.0[index as usize]) as &mut Self::Output
    }
}

impl Memory {
    pub fn new(
        sp: WordSize,
        local: WordSize,
        arg: WordSize,
        this: WordSize,
        that: WordSize,
    ) -> Memory {
        let mut ram = MemoryVec::new(vec![0; Memory::ram_size() as usize]);
        let display = MemoryVec::new(vec![0; Memory::display_size() as usize]);
        ram[SP] = sp;
        ram[LCL] = local;
        ram[ARG] = arg;
        ram[THIS] = this;
        ram[THAT] = that;

        Memory {
            ram,
            display,
            keyboard: 0,
            cursor_line: 0,
            cursor_col: 0,
            screen_color: 0,
            char_map: CharMap::new(),
            heap_alloc: Vec::new(),
            display_updated: false,
        }
    }

    /**
     * Pushes to the global stack the value described by segment and index
     */
    pub fn push(&mut self, segment: Segment, offset: Offset) {
        let value = match segment {
            Segment::Pointer => {
                if offset == 0 {
                    self.get_pointer(THIS)
                } else if offset == 1 {
                    self.get_pointer(THAT)
                } else {
                    panic!("Pointer can only have offset of 0 or 1")
                }
            }
            Segment::Constant => offset.to_owned(),
            Segment::Local => self.get_value_by_pointer(LCL, offset),
            Segment::Argument => self.get_value_by_pointer(ARG, offset),
            Segment::Static => self.get_value_by_pointer(STATIC, offset),
            Segment::This => self.get_value_by_pointer(THIS, offset),
            Segment::That => self.get_value_by_pointer(THAT, offset),
            Segment::Temp => *self.peek(TEMP + offset),
        };
        let stack_pointer = self.get_pointer(SP);
        // Set value to stack and increment SP
        self.ram[stack_pointer] = value;
        self.ram[SP] += 1;
    }

    /**
     * Moves to memory location described by segment and index the item at the top of the global stack
     * Returns the value that was popped
     */
    pub fn pop(&mut self, segment: Segment, offset: Offset) -> WordSize {
        // Decrement SP
        self.ram[SP] -= 1;
        let value = self.get_value_by_pointer(SP, 0);

        let address = match segment {
            Segment::Pointer => {
                if offset == 0 {
                    THIS
                } else if offset == 1 {
                    THAT
                } else {
                    panic!("Pointer can only have offset of 0 or 1")
                }
            }
            Segment::Constant => panic!("Constant can only be pushed"),
            Segment::Local => self.get_pointer(LCL) + offset,
            Segment::Argument => self.get_pointer(ARG) + offset,
            Segment::Static => {
                if STATIC + offset <= STATIC_MAX {
                    STATIC + offset
                } else {
                    panic!("Static memory segment overflow.")
                }
            }
            Segment::This => self.get_pointer(THIS) + offset,
            Segment::That => self.get_pointer(THAT) + offset,
            Segment::Temp => {
                if TEMP + offset <= TEMP_MAX {
                    TEMP + offset
                } else {
                    panic!("Static memory segment overflow.")
                }
            }
        };

        self.ram[address] = value;
        value
    }

    pub fn get_pointer(&self, pointer: WordSize) -> WordSize {
        self.ram[pointer]
    }

    pub fn set_pointer(&mut self, pointer: WordSize, value: WordSize) {
        self.ram[pointer] = value;
    }

    pub fn get_value_by_pointer(&self, pointer: WordSize, offset: WordSize) -> WordSize {
        self.ram[self.ram[pointer] + offset]
    }

    pub fn set_value_by_pointer(&mut self, pointer: WordSize, offset: WordSize, value: WordSize) {
        let address = self.ram[pointer] + offset;
        self.ram[address] = value;
    }

    /**
     * Returns a reference to the value of memory at the index, using the HACK computer memory mapping
     * ram: 0-16383
     * display: 16384-24575
     * keyboard: 24576
     */
    pub fn peek(&self, index: WordSize) -> &WordSize {
        match index {
            0..=RAM_MAX_INDEX => &self.ram[index],
            RAM_SIZE..=DISPLAY_MAX_INDEX => &self.display[index],
            KEYBOARD_MEM => &self.keyboard,
            _ => panic!("Invalid memory index: {}", index),
        }
    }

    /**
     * Changes at the index to the provided value, using the HACK computer memory mapping
     * ram: 0-16383
     * display: 16384-24575
     * keyboard: 24576
     * Returns: Void
     */
    pub fn poke(&mut self, index: WordSize, value: WordSize) {
        match index {
            0..=RAM_MAX_INDEX => self.ram[index] = value,
            RAM_SIZE..=DISPLAY_MAX_INDEX => self.display[index] = value,
            KEYBOARD_MEM => self.keyboard = value,
            _ => panic!("Invalid memory index: {}", index),
        };
    }

    pub fn get_arg(&self, arg_num: WordSize) -> WordSize {
        self.get_value_by_pointer(ARG, arg_num)
    }

    pub fn push_stack_frame(&mut self, num_args: WordSize, line_num: WordSize) {
        // Save return address (not used)
        self.push(Segment::Constant, line_num);
        // Build caller stack
        self.push(Segment::Constant, self.get_pointer(LCL));
        self.push(Segment::Constant, self.get_pointer(ARG));
        self.push(Segment::Constant, self.get_pointer(THIS));
        self.push(Segment::Constant, self.get_pointer(THAT));
        // Set Local Pointer
        self.set_pointer(ARG, self.get_pointer(SP) - num_args - 5);
        self.set_pointer(LCL, self.get_pointer(SP));
    }

    pub fn pop_stack_frame(&mut self) {
        // move return value to where it can be accessed by caller
        self.pop(Segment::Argument, 0);
        // reposition SP
        self.set_pointer(SP, self.get_pointer(ARG) + 1);
        // reset memory pointers based on call stack
        let that = self.get_value_by_pointer(LCL, -1);
        self.set_pointer(THAT, that);
        let this = self.get_value_by_pointer(LCL, -2);
        self.set_pointer(THIS, this);
        let arg = self.get_value_by_pointer(LCL, -3);
        self.set_pointer(ARG, arg);
        let lcl = self.get_value_by_pointer(LCL, -4);
        self.set_pointer(LCL, lcl);
        // Return address isn't used
    }

    pub fn ram(&self) -> *const WordSize {
        self.ram.as_ptr()
    }

    pub fn ram_size() -> WordSize {
        RAM_SIZE
    }

    pub fn display(&self) -> *const WordSize {
        self.display.as_ptr()
    }

    pub fn set_display_xy(&mut self, x: WordSize, y: WordSize) {
        self.display_updated = true;
        let display_word = y * (DISPLAY_WIDTH / 16) + x / 16;
        let bit = x % 16;
        let mask: WordSize = 1 << bit;
        if self.screen_color == 0 {
            // and with inverse of mask
            self.display[display_word] &= !mask;
        } else {
            // or with mask
            self.display[display_word] |= mask;
        }
    }

    pub fn set_display_word(&mut self, index: WordSize, value: WordSize) {
        self.display_updated = true;
        self.display[index] = value;
    }

    pub fn get_display_value(&self, index: WordSize) -> WordSize {
        self.display[index]
    }

    pub fn display_size() -> WordSize {
        (DISPLAY_HEIGHT) * (DISPLAY_WIDTH / 16)
    }

    pub fn set_display(&mut self, value: WordSize, offset: WordSize) {
        self.display_updated = true;
        self.display[offset] = value;
    }

    pub fn clear_display(&mut self) {
        self.display_updated = true;
        self.display.fill(0);
    }

    pub fn fill_display(&mut self) {
        self.display_updated = true;
        self.display.fill(-1);
    }

    pub fn keyboard(&self) -> WordSize {
        self.keyboard
    }

    /**
     * Allocates a block of memory of at least 'size' words
     * Returns the pointer to the block
     */
    pub(crate) fn alloc(&mut self, requested_size: WordSize) -> WordSize {
        // Walk through the HeapAllocations and return the first one that is big enough
        for allocation in self
            .heap_alloc
            .iter_mut()
            .filter(|a| a.status == MemoryStatus::Free && a.size >= requested_size)
        {
            allocation.status = MemoryStatus::Used;
            // console_log!("Alloc returning reused block with address of {}", allocation.pointer);
            return allocation.pointer;
        }

        // Otherwise make a new allocation
        let next_free = self
            .heap_alloc
            .last()
            .map(|a| a.pointer)
            .unwrap_or(RAM_SIZE - 1);
        let new_pointer = next_free - requested_size;
        self.heap_alloc
            .push(HeapAllocation::new(new_pointer, requested_size));
        // console_log!("Alloc returning new block with address of {}",new_pointer);
        new_pointer
    }

    /**
     * Frees block of memory pointed to by 'pointer'
     */
    pub(crate) fn de_alloc(&mut self, pointer: WordSize) {
        // console_log!("de_alloc attempting to de-allocate block with address {}", pointer);
        match self.heap_alloc.iter_mut().find(|a| a.pointer == pointer) {
            Some(a) => match a.status {
                MemoryStatus::Used => a.status = MemoryStatus::Free,
                MemoryStatus::Free => (),
            },
            None => (),
        }
    }
}

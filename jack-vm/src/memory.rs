use crate::parser::{Offset, Segment};
use std::ops::{Index, IndexMut};

pub type WordSize = i16;

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
    keyboard: WordSize, // something to keep track of allocations on heap for when objects are implemented
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
// struct Block {
//     pointer: u16,
//     size: u16,
// }

const RAM_SIZE: WordSize = 16384;
const DISPLAY_SIZE: WordSize = 8192; // 256x512
const KEYBOARD: WordSize = 24576;
const SP: WordSize = 0;
const LCL: WordSize = 1;
const ARG: WordSize = 2;
const THIS: WordSize = 3;
const THAT: WordSize = 4;
const STATIC: WordSize = 15;
const STATIC_MAX: WordSize = 255;
const TEMP: WordSize = 5;
const TEMP_MAX: WordSize = 12;

impl Memory {
    pub fn new(
        sp: WordSize,
        local: WordSize,
        arg: WordSize,
        this: WordSize,
        that: WordSize,
    ) -> Memory {
        let mut ram = MemoryVec::new(vec![0; RAM_SIZE as usize]);
        let display = MemoryVec::new(vec![0; DISPLAY_SIZE as usize]);
        let keyboard = 0;

        ram[SP] = sp;
        ram[LCL] = local;
        ram[ARG] = arg;
        ram[THIS] = this;
        ram[THAT] = that;

        Memory {
            ram,
            display,
            keyboard,
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
            Segment::Local => self.get_value(LCL, offset),
            Segment::Argument => self.get_value(ARG, offset),
            Segment::Static => self.get_value(STATIC, offset),
            Segment::This => self.get_value(THIS, offset),
            Segment::That => self.get_value(THAT, offset),
            Segment::Temp => self.get_value(TEMP, offset),
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
        let value = self.get_value(SP, 0);

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

    fn get_pointer(&self, pointer: WordSize) -> WordSize {
        self.ram[pointer]
    }

    fn set_pointer(&mut self, pointer: WordSize, value: WordSize) {
        self.ram[pointer] = value;
    }

    fn get_value(&self, pointer: WordSize, offset: WordSize) -> WordSize {
        self.ram[self.ram[pointer] + offset]
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
        let that = self.get_value(LCL, -1);
        self.set_pointer(THAT, that);
        let this = self.get_value(LCL, -2);
        self.set_pointer(THIS, this);
        let arg = self.get_value(LCL, -3);
        self.set_pointer(ARG, arg);
        let lcl = self.get_value(LCL, -4);
        self.set_pointer(LCL, lcl);
        // Return address isn't used
    }

    pub fn peek(&self, index: WordSize) -> WordSize {
        if index < RAM_SIZE {
            self.ram[index]
        } else if index < RAM_SIZE + DISPLAY_SIZE {
            self.display[index]
        } else if index == KEYBOARD {
            self.keyboard
        } else {
            panic!(
                "Index out of bounds. Valid indexes range from 0 to {}",
                KEYBOARD
            );
        }
    }

    pub fn poke(&mut self, index: WordSize, value: WordSize) {
        if index < RAM_SIZE {
            self.ram[index] = value;
        } else if index < RAM_SIZE + DISPLAY_SIZE {
            self.display[index] = value;
        } else if index == KEYBOARD {
            self.keyboard = value;
        } else {
            panic!(
                "Index out of bounds. Valid indexes range from 0 to {}",
                KEYBOARD
            );
        };
    }

    pub fn ram(&self) -> *const WordSize {
        self.ram.as_ptr()
    }

    pub fn display(&self) -> *const WordSize {
        self.display.as_ptr()
    }

    pub fn keyboard(&self) -> WordSize {
        self.keyboard
    }

    pub fn set_display(&mut self, value: WordSize, offset: WordSize) {
        self.display[offset] = value;
    }

    pub fn ram_size(&self) -> WordSize {
        self.ram.len()
    }

    pub fn display_size(&self) -> WordSize {
        self.display.len()
    }

    /**
     * Allocates a block of memory of at least 'size' words
     * Returns the pointer to the block
     */
    fn allocate(size: u16) -> u16 {
        todo!()
    }

    /**
     * Frees block of memory pointed to by 'pointer'
     */
    fn deallocate(pointer: u16) {
        todo!()
    }
}

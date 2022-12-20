use core::panic;
use std::cmp::{max, min};
use web_sys::console::{self, assert};

use crate::memory::{Memory, WordSize, DISPLAY_HEIGHT, DISPLAY_WIDTH, WORDSIZE};

pub type NativeFunction = fn(&mut Memory, WordSize) -> WordSize;

const LINES: WordSize = 23;
const COLS: WordSize = 64;
const VOID: WordSize = 0;
const CHAR_HEIGHT: WordSize = 11;
const CHAR_WIDTH: WordSize = 8;

// MATH
pub fn multiply(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 2);
    let a = memory.get_arg(0);
    let b = memory.get_arg(1);
    a * b
}

pub fn divide(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 2);
    let a = memory.get_arg(0);
    let b = memory.get_arg(1);
    a / b
}

pub fn jack_min(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 2);
    let a = memory.get_arg(0);
    let b = memory.get_arg(1);
    i16::min(a, b)
}

pub fn jack_max(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 2);
    let a = memory.get_arg(0);
    let b = memory.get_arg(1);
    i16::max(a, b)
}

pub fn jack_sqrt(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let a = memory.get_arg(0);
    (a as f32).sqrt() as i16
}

pub fn jack_pow(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 2);
    let a = memory.get_arg(0);
    let b = memory.get_arg(1);
    a.pow(b as u32)
}

// STRING
// Strings are heap allocated objects with the following fields:
// length: current length of the char array
// max_length: maximum length of the char array
// string: an array of chars
/**
 * Allocates a new string of length max_length
 * arg0: max_length
 * returns: pointer to string object
 */
pub fn string_new(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let max_length = memory.get_arg(0);
    let req_size = max_length + 2;
    let string_pointer = memory.alloc(req_size);
    // set length to 0
    memory.poke(string_pointer, 0);
    // set max length
    memory.poke(string_pointer + 1, max_length);
    string_pointer
}

/**
 * Disposes of the string
 * returns: VOID
 */
pub fn string_dispose(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 0);
    let string_pointer = memory.get_arg(0);
    memory.de_alloc(string_pointer);
    VOID
}

/**
 * Get number of characters in the string
 * returns: number of characters in string
 */
pub fn string_length(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 0);
    let string_pointer = memory.get_arg(0);
    // length value is located at the 0th position from the string pointer
    *memory.peek(string_pointer)
}

// OUTPUT
// The screen is mapped to 24 rows of 64 characters, with each character
// being 8 pixels wide and 11 pixels high, including margins
fn print_char_helper(memory: &mut Memory, character: &WordSize) {
    let bitmap = memory.char_map.get_bitmap(character);
    // 32 words in a display line
    // each cursor line covers 11 display lines

    for char_row in 0..=11 {
        let address = (DISPLAY_WIDTH / WORDSIZE) * (memory.cursor_line * CHAR_HEIGHT + char_row)
            + memory.cursor_col / 2;
        // even cursor colums change the first half of the word (marked by X) -> 00000000XXXXXXXX
        // odd cursor colums change the second half of the word (marked by X) -> XXXXXXXX00000000
        if memory.cursor_col % 2 == 0 {
            todo!();
        } else {
            todo!();
        }
    }
}

/**
 * Moves cursor to line and col specified
 * arg0: line
 * arg1: col
 * returns: VOID
 */
pub fn move_cursor(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 2);
    let line = memory.get_arg(0);
    let col = memory.get_arg(1);
    assert!(line < LINES && col < COLS);
    memory.cursor_line = line;
    memory.cursor_col = col;
    VOID
}

/**
 * Prints character c
 * arg0: character
 * returns: VOID
 */
pub fn print_char(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let c = &memory.get_arg(0);
    print_char_helper(memory, c);
    // set values at correct half of word to values in bitmap
    VOID
}

pub fn print_string(memory: &mut Memory, args: WordSize) -> WordSize {
    //s is a pointer to a string object in memory
    todo!()
}

pub fn print_int(memory: &mut Memory, args: WordSize) -> WordSize {
    todo!()
}

/**
 * Moves cursor to start of next line
 * returns: VOID
 */
pub fn println(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 0);
    memory.cursor_line = (memory.cursor_line + 1) % (LINES - 1);
    memory.cursor_col = 0;
    VOID
}

/**
 * Deletes previous character
 * returns: VOID
 */
pub fn backspace(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 0);
    memory.cursor_col = i16::min(0, memory.cursor_col - 1);
    todo!();
    // print blank space

    VOID
}

// SCREEN
/**
 * Draws line given coordinates x1, y1, x2, y2
 * Returns: void
 */
fn draw_line_helper(memory: &mut Memory, x1: WordSize, y1: WordSize, x2: WordSize, y2: WordSize) {
    let dx = x2 - x1;
    let dy = y2 - y1;

    let abs_dx = i16::abs(dx);
    let abs_dy = i16::abs(dy);

    let delta_x = dx.signum();
    let delta_y = dy.signum();

    // a and b track how far up and over we went so far
    // when a == dx and b == dy, we are at x2, y2
    let mut a: WordSize = 0;
    let mut b: WordSize = 0;
    let mut diff = 0;

    match (dx, dy) {
        (_, 0) => {
            while i16::abs(a) <= abs_dx {
                memory.set_display_index(x1 + a, y1);
                a += delta_x;
            }
        }
        (0, _) => {
            while i16::abs(b) <= abs_dy {
                memory.set_display_index(x1, y1 + b);
                b += delta_y;
            }
        }
        (_, _) => {
            while i16::abs(a) <= abs_dx && i16::abs(b) <= abs_dy {
                memory.set_display_index(x1 + a, y1 + b);
                if diff < 0 {
                    a += delta_x;
                    diff += abs_dy;
                } else {
                    b += delta_y;
                    diff -= abs_dx;
                }
            }
        }
    }
}

/**
 * Clears screen
 * returns VOID
 */
pub fn clear_screen(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 0);
    memory.clear_display();
    VOID
}

/**
 * Fills screen
 * returns VOID
 */
pub fn fill_screen(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 0);
    memory.fill_display();
    VOID
}

/**
 * Sets color. 0 = off, 1 = on
 * arg0: color
 * returns VOID
 */
pub fn set_color(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let color = memory.get_arg(0);
    memory.screen_color = color;
    VOID
}

/**
 * Draws pixel at point x, y
 * arg0: x
 * arg1: y
 * returns VOID
 */
pub fn draw_pixel(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 2);
    let x = memory.get_arg(0);
    let y = memory.get_arg(1);
    memory.set_display_index(x, y);
    VOID
}

/**
 * Draws line from point x1, y1, to point x2, y2
 * arg0: x1
 * arg1: y1
 * arg2: x2
 * arg3: y2
 * returns VOID
 */
pub fn draw_line(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 4);
    let x1 = memory.get_arg(0);
    let y1 = memory.get_arg(1);
    let x2 = memory.get_arg(2);
    let y2 = memory.get_arg(3);
    draw_line_helper(memory, x1, y1, x2, y2);
    VOID
}

/**
 * Draw unfilled rectangle from point x1, y1, to point x2, y2
 * arg0: x1
 * arg1: y1
 * arg2: x2
 * arg3: y2
 */
pub fn draw_rectangle_outline(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 4);
    let x1 = memory.get_arg(0);
    let y1 = memory.get_arg(1);
    let x2 = memory.get_arg(2);
    let y2 = memory.get_arg(3);
    draw_line_helper(memory, x1, y1, x1, y2);
    draw_line_helper(memory, x2, y1, x2, y2);
    draw_line_helper(memory, x1, y1, x2, y1);
    draw_line_helper(memory, x1, y2, x2, y2);
    VOID
}

/**
 * Draw filled rectangle from point x1, y1, to point x2, y2
 * arg0: x1
 * arg1: y1
 * arg2: x2
 * arg3: y2
 */
pub fn draw_rectangle(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 4);
    let x1 = memory.get_arg(0);
    let y1 = memory.get_arg(1);
    let x2 = memory.get_arg(2);
    let y2 = memory.get_arg(3);
    let dy = y2 - y1;
    let abs_dy = i16::abs(dy);
    let delta_y = dy.signum();
    let mut a = 0;
    while i16::abs(a) <= abs_dy {
        draw_line_helper(memory, x1, y1 + a, x2, y1 + a);
        a += delta_y;
    }
    VOID
}

/**
 * Draw filled circle with center at point x1, y1, and radius r
 * arg0: x1
 * arg1: y1
 * arg2: r
 * returns VOID
 */
pub fn draw_circle(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 3);
    let x1 = memory.get_arg(0);
    let y1 = memory.get_arg(1);
    let r = memory.get_arg(2);

    let bottom = max(y1 + r, 0);
    let top = min(y1 - r, DISPLAY_HEIGHT - 1);
    let left = max(x1 - r, 0);
    let right = min(x1 + r, DISPLAY_WIDTH - 1);
    console::log_1(&top.into());
    console::log_1(&bottom.into());
    for row in top..bottom {
        let dy = row - y1;
        let offset = ((i32::pow(r as i32, 2) - i32::pow(dy as i32, 2)) as f32).sqrt() as WordSize;
        if offset > 0 {
            console::log_1(&row.into());
            console::log_1(&offset.into());
            draw_line_helper(
                memory,
                max(left, x1 - offset),
                row,
                min(right, x1 + offset),
                row,
            );
        }
    }
    VOID
}

//KEYBOARD
pub fn key_pressed(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args ==0);
    memory.keyboard
}

pub fn read_char(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 0);
    panic!("readChar is not implemented")
}

pub fn read_line(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 0);
    panic!("readLine is not implemented")
}

pub fn read_int(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 0);
    panic!("readInt is not implemented")
}


// MEMORY
/**
 * Returns a reference to the value of memory at the index, using the HACK computer memory mapping
 * ram: 0-16383
 * display: 16384-24575
 * keyboard: 24576
 */
pub fn jack_peek(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let index = memory.get_arg(0);
    *memory.peek(index)
}
 /**
 * Changes at the index to the provided value, using the HACK computer memory mapping
 * ram: 0-16383
 * display: 16384-24575
 * keyboard: 24576
 * Returns: Void
 */
pub fn jack_poke(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 2);
    let index = memory.get_arg(0);
    let value = memory.get_arg(1);
    memory.poke(index, value);
    VOID
}

/**
 * Allocates a block of memory of at least 'size' words
 * arg0: size
 * returns VOID
 */
pub fn alloc(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let size = memory.get_arg(0);
    memory.alloc(size)
}
/**
 * Frees block of memory pointed to by 'pointer'
 * arg0: pointer to allocated memory
 * returns: VOID
 */
pub fn de_alloc(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let pointer = memory.get_arg(0);
    memory.de_alloc(pointer);
    VOID
}

// SYS
pub fn wait(memory: &mut Memory, args: WordSize) -> WordSize {
    // Doesn't wait - this VM is pretty slow already. :P
    // rust wasm also doesn't have access to clocks
    // Could either implement a JS binding or set up a system configuration by benchmarking a basic loop
    // and the using that loop to implement wait.
    VOID
}

pub fn halt(memory: &mut Memory, args: WordSize) -> WordSize {
    panic!("halt is not implemented");
    VOID
}

pub fn error(memory: &mut Memory, args: WordSize) -> WordSize {
    panic!("error is not implemented");
    VOID
}
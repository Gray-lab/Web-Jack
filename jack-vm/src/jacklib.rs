use std::cmp::{max, min};
use wasm_bindgen_test::console_log;

use crate::memory::{Memory, WordSize, DISPLAY_HEIGHT, DISPLAY_WIDTH, WORDSIZE};

pub type NativeFunction = fn(&mut Memory, WordSize) -> WordSize;

const LINES: WordSize = 23;
const COLS: WordSize = 64;
const VOID: WordSize = 0;
const CHAR_HEIGHT: WordSize = 11;

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

pub fn jack_abs(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let a = memory.get_arg(0);
    i16::abs(a)
}

pub fn jack_mod(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 2);
    let a = memory.get_arg(0);
    let b = memory.get_arg(1);
    a % b
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
    // console_log!("Requesting allocation for string of size {}", req_size);
    let string_pointer = memory.alloc(req_size);
    // set length to 0
    memory.poke(string_pointer, 0);
    // set max length
    memory.poke(string_pointer + 1, max_length);
    string_pointer
}

/**
 * Disposes of the string
 * arg0: object pointer
 * returns: VOID
 */
pub fn string_dispose(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let string_pointer = memory.get_arg(0);
    // console_log!("Requesting de-allocation for string with pointer {}", string_pointer);
    memory.de_alloc(string_pointer);
    VOID
}

/**
 * Get number of characters in the string
 * arg0: object pointer
 * returns: number of characters in string
 */
pub fn string_length(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let string_pointer = memory.get_arg(0);
    // length value is located at the 0th position from the string pointer
    *memory.peek(string_pointer)
}

/**
 * Return character at index
 * arg0: object pointer
 * arg1: index
 * returns: character value
 */
pub fn char_at(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 2);
    let string_pointer = memory.get_arg(0);
    let index = memory.get_arg(1);
    *memory.peek(string_pointer + index + 2)
}

/**
 * Sets char at index to value
 * arg0: object pointer
 * arg1: index
 * arg2: value
 * returns: VOID
 */
pub fn set_char_at(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 3);
    let string_pointer = memory.get_arg(0);
    let index = memory.get_arg(1);
    let chararcter = memory.get_arg(2);
    memory.poke(string_pointer + index + 2, chararcter);
    VOID
}

/**
 * Appends character to end of string. Will not append if string max length would be exceeded.
 * arg0: string pointer
 * arg1: character
 * returns: string pointer
 */
pub fn append_char(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 2);
    let string_pointer = memory.get_arg(0);
    let character = memory.get_arg(1);
    let length = memory.peek(string_pointer).clone();
    let max_length = memory.peek(string_pointer + 1).clone();
    if length < max_length {
        memory.poke(string_pointer + 2 + length, character);
        memory.poke(string_pointer, length + 1);
        string_pointer
    } else {
        console_log!("Attempted to append to a string at max_length");
        string_pointer
    }
}

/**
 * Erases last character in string
 * arg0: string pointer
 * returns: VOID
 */
pub fn erase_last_char(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let string_pointer = memory.get_arg(0);
    let length = memory.peek(string_pointer).clone();
    memory.poke(string_pointer, length - 1);
    VOID
}

/**
 * Returns the integer value of a string, until the first non-numeric character
 * arg0: string pointer
 * returns: integer as WordSize
 */
pub fn int_value(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    // Get integer value
    let string_pointer = memory.get_arg(0);
    let len = memory.peek(string_pointer);
    let mut string = String::from("");
    for i in (string_pointer + 2)..(string_pointer + 2 + len) {
        string.push(*memory.peek(i) as u8 as char)
    }
    console_log!("{}", string);

    let sign = if string.starts_with('-') { -1 } else { 1 };
    if sign == -1 {
        // Remove the '-'
        string.remove(0);
    }

    //parse the string into an integer
    //https://stackoverflow.com/questions/65601579/parse-an-integer-ignoring-any-non-numeric-suffix
    let number = string
        .chars()
        .map(|c| c.to_digit(10))
        .take_while(|opt| opt.is_some())
        .fold(0, |acc, digit| acc * 10 + digit.unwrap());

    console_log!("{}", number);
    (number as i32 * sign) as WordSize
}

/**
 * Sets value of string to the string representation of an integer
 * arg0: string pointer
 * arg1: value
 * returns: VOID
 */
pub fn set_int(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 2);
    let string_pointer = memory.get_arg(0);
    // Get integer value
    let mut value = memory.get_arg(1);
    // Start at third position in String
    let mut position = 2;
    // Fill with value
    while value != 0 {
        memory.poke(string_pointer + position, value % 10);
        value = value / 10;
        position += 1;
    }

    VOID
}

/**
 * returns backspace character (129)
 */
pub fn string_backspace(_memory: &mut Memory, _args: WordSize) -> WordSize {
    129
}

/**
 * returns double quote character (34)
 */
pub fn double_quote(_memory: &mut Memory, _args: WordSize) -> WordSize {
    34
}

/**
 * returns newline character (128)
 */
pub fn new_line(_memory: &mut Memory, _args: WordSize) -> WordSize {
    128
}

// ARRAY
/**
 * Allocates a new array of size
 * arg0: size
 * returns: pointer to array
 */
pub fn array_new(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let size = memory.get_arg(0);
    // console_log!("Requesting allocation for array of size {}", size);
    memory.alloc(size)
}

/**
 * Deallocates array at pointer
 * arg0: pointer
 * returns: VOID
 */
pub fn array_dispose(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let pointer = memory.get_arg(0);
    // console_log!("Requesting de-allocation for array with pointer {}", pointer);
    memory.de_alloc(pointer);
    VOID
}

// OUTPUT
// The screen is mapped to 24 rows of 64 characters, with each character
// being 8 pixels wide and 11 pixels high, including margins
fn print_char_helper(memory: &mut Memory, character: &WordSize) {
    let bitmap = memory.char_map.get_bitmap(character).clone();
    // 32 words in a display line
    // each cursor line covers 11 display lines

    for char_row in 0..11 {
        let address = (DISPLAY_WIDTH / WORDSIZE) * (memory.cursor_line * CHAR_HEIGHT + char_row)
            + memory.cursor_col / 2;
        // even cursor colums change the first half of the word (marked by X) -> 00000000XXXXXXXX
        // odd cursor colums change the second half of the word (marked by X) -> XXXXXXXX00000000
        let value = memory.get_display_value(address).to_le_bytes();

        if memory.cursor_col % 2 == 0 {
            let new_value = (((value[1] as u16) << 8) | bitmap[char_row as usize] as u16) as i16;
            memory.set_display_word(address, new_value);
        } else {
            let new_value = (((bitmap[char_row as usize] as u16) << 8) | value[0] as u16) as i16;
            memory.set_display_word(address, new_value);
        }
    }
}

/**
 * Steps cursor one location, moving to next line at end of current line
 * returns: Void
 */
fn step_cursor_helper(memory: &mut Memory) {
    if memory.cursor_col == COLS - 1 {
        memory.cursor_col = 0;
        memory.cursor_line = (memory.cursor_line + 1) % (LINES);
    } else {
        memory.cursor_col += 1;
    }
}

/**
 * moves cursor to start of new line
 * returns: void
 */
fn newline_helper(memory: &mut Memory) {
    memory.cursor_line = (memory.cursor_line + 1) % (LINES - 1);
    memory.cursor_col = 0;
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
    console_log!("in print_char({})", c);
    print_char_helper(memory, c);
    step_cursor_helper(memory);
    VOID
}

/**
 * Prints string
 * arg0: string pointer
 * returns: VOID
 */
pub fn print_string(memory: &mut Memory, args: WordSize) -> WordSize {
    //s is a pointer to a string object in memory
    assert!(args == 1);
    let string_pointer = memory.get_arg(0);
    let length = memory.peek(string_pointer).clone();
    for character_pointer in (string_pointer + 2)..(string_pointer + 2 + length) {
        let character = memory.peek(character_pointer).clone();
        print_char_helper(memory, &character);
        step_cursor_helper(memory);
    }
    VOID
}

/**
 * Prints integer value. Handles positive and negative values.
 * arg0: int
 * returns: VOID
 */
pub fn print_int(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    let mut i = memory.get_arg(0);
    let mut digits = vec![];
    let sign: WordSize = if i < 0 { -1 } else { 1 };

    console_log!("In print in with value {}", i);
    i = i * sign;
    while i != 0 {
        digits.push((i % 10) + 48); /* value 48 corresponds to a 0 character in the character bitmap */
        i /= 10;
    }
    if sign == -1 {
        // Print minus sign
        print_char_helper(memory, &(45 as WordSize));
        step_cursor_helper(memory);
    }

    for d in digits.iter().rev() {
        print_char_helper(memory, d);
        step_cursor_helper(memory);
    }
    VOID
}

/**
 * Moves cursor to start of next line
 * returns: VOID
 */
pub fn println(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 0);
    newline_helper(memory);
    VOID
}

/**
 * Deletes previous character
 * returns: VOID
 */
pub fn output_backspace(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 0);
    memory.cursor_col = i16::max(0, memory.cursor_col - 1);
    // print blank space without advancing cursor
    print_char_helper(memory, &32);
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
                memory.set_display_xy(x1 + a, y1);
                a += delta_x;
            }
        }
        (0, _) => {
            while i16::abs(b) <= abs_dy {
                memory.set_display_xy(x1, y1 + b);
                b += delta_y;
            }
        }
        (_, _) => {
            while i16::abs(a) <= abs_dx && i16::abs(b) <= abs_dy {
                memory.set_display_xy(x1 + a, y1 + b);
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
    memory.set_display_xy(x, y);
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

    for row in top..bottom {
        let dy = row - y1;
        let offset = ((i32::pow(r as i32, 2) - i32::pow(dy as i32, 2)) as f32).sqrt() as WordSize;
        if offset > 0 {
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
/**
 * Returns the character of the currently pressed key on the keyboard;
 * if no key is currently pressed, returns 0.
 *
 * Recognizes all ASCII characters, as well as the following keys:
 * new line = 128 = String.newline()
 * backspace = 129 = String.backspace()
 * left arrow = 130
 * up arrow = 131
 * right arrow = 132
 * down arrow = 133
 * home = 134
 * End = 135
 * page up = 136
 * page down = 137
 * insert = 138
 * delete = 139
 * ESC = 140
 * F1 - F12 = 141 - 152
 */
pub fn key_pressed(memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 0);
    memory.keyboard
}

/**
 * Waits until a key is pressed on the keyboard and released,
 * then echoes the key to the screen, and returns the character
 * of the pressed key.
 */
pub fn read_char(_memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 0);
    panic!("readChar is implemented as a jack function, so don't look here for it")
}

/**
 * Displays the message on the screen, reads from the keyboard the entered
 * text until a newline character is detected, echoes the text to the screen,
 * and returns its value. Also handles user backspaces.
 */
pub fn read_line(_memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    panic!("readLine is implemented as a jack function, so don't look here for it")
}

/**
 * Displays the message on the screen, reads from the keyboard the entered
 * text until a newline character is detected, echoes the text to the screen,
 * and returns its integer value (until the first non-digit character in the
 * entered text is detected). Also handles user backspaces.
 */
pub fn read_int(_memory: &mut Memory, args: WordSize) -> WordSize {
    assert!(args == 1);
    panic!("readInt is implemented as a jack function, so don't look here for it")
}

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
pub fn wait(_memory: &mut Memory, _args: WordSize) -> WordSize {
    // Doesn't wait - this VM is pretty slow already. :P
    // rust wasm also doesn't have access to clocks
    // Could either implement a JS binding or set up a system configuration by benchmarking a basic loop
    // and the using that loop to implement wait.
    VOID
}

pub fn halt(_memory: &mut Memory, _args: WordSize) -> WordSize {
    panic!("halt is not implemented");
    VOID
}

pub fn error(_memory: &mut Memory, _args: WordSize) -> WordSize {
    panic!("error is not implemented");
    VOID
}

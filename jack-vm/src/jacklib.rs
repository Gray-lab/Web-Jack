use web_sys::console::assert;

use crate::memory::{Memory, WordSize};

pub type NativeFunction = fn(&mut Memory, WordSize) -> WordSize;

const LINES: WordSize = 23;
const COLS: WordSize = 64;
const VOID: WordSize = 0;

// OUTPUT
pub fn moveCursor(memory: &mut Memory, args: WordSize) -> WordSize {
    // assert!(line < LINES && col < COLS);
    // self.cursor_line = line;
    // self.cursor_col = col;
    todo!()
}

pub fn printChar(memory: &mut Memory, args: WordSize) -> WordSize {
    todo!()
}

pub fn printString(memory: &mut Memory, args: WordSize) -> WordSize {
    //s is a pointer to a string object in memory
    todo!()
}

pub fn printInt(memory: &mut Memory, args: WordSize) -> WordSize {
    todo!()
}

pub fn println(memory: &mut Memory, args: WordSize) -> WordSize {
    // memory.cursor_line = (self.cursor_line + 1) % (LINES - 1);
    todo!()
}

pub fn backSpace(memory: &mut Memory, args: WordSize) -> WordSize {
    // self.cursor_col = (self.cursor_col - 1) % (COLS - 1);
    // print blank space
    todo!()
}

// SCREEN
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
 * Draw circle with center at point x1, y1, and radius r
 * arg0: x1
 * arg1: y1
 * arg2: r
 */
pub fn draw_circle(memory: &mut Memory, args: WordSize) -> WordSize {
    todo!()
}

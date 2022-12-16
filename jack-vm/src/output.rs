struct Output {
    cursor_line: WordSize,
    cursor_col: WordSize,
}

const LINES: WordSize = 23;
const COLS: WordSize = 64;

pub impl Output {
    fn moveCursor(&mut self, line: WordSize, col: WordSize) {
        assert!(line < LINES && col < COLS);
        self.cursor_line = line;
        self.cursor_col = col;
    }

    fn printChar(c: WordSize) {
        todo!()
    }

    fn printString(s: WordSize) {
        //s is a pointer to a string object in memory
        todo!()
    }

    fn printInt(i: WordSize) {
        todo!()
    }

    fn println(&mut self) {
        self.cursor_line = (self.cursor_line + 1) % (LINES - 1);
    }

    fn backSpace(&mut self) {
        self.cursor_col = (self.cursor_col - 1) % (COLS - 1);
        // print blank space
        todo!()
    }
}

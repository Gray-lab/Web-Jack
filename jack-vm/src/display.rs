struct Screen {
    color: bool,
}

pub impl Screen {
    fn clearScreen() {}

    fn setColor(color: bool) {}

    fn drawPixel(x: WordSize, y: WordSize) {}

    fn drawLine(x1: WordSize, y1: WordSize, x2: WordSize, y2: WordSize) {}

    fn drawRectangle(x1: WordSize, y1: WordSize, x2: WordSize, y2: WordSize) {}

    fn drawCircle(x: WordSize, y: WordSize, r: WordSize) {}
}

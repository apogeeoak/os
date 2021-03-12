use screen_char::{Color, ColorCode, ScreenChar};

mod screen_char;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub(super) struct Buffer {
    characters: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    pub fn new() -> &'static mut Buffer {
        unsafe { &mut *(0xb8000 as *mut Buffer) }
    }

    pub fn last_row() -> usize {
        BUFFER_HEIGHT - 1
    }
}

pub(super) struct BufferWriter {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl BufferWriter {
    pub fn new() -> BufferWriter {
        BufferWriter {
            column_position: 0,
            color_code: ColorCode::new(Color::LightGray, Color::Black),
            buffer: Buffer::new(),
        }
    }

    pub fn read_byte(&self, row: usize, column: usize) -> u8 {
        self.buffer.characters[row][column].character()
    }

    pub fn write_byte(&mut self, byte: u8) {
        let byte = match byte {
            // Newline.
            b'\n' => {
                self.new_line();
                return;
            }
            // Printable ASCII byte.
            0x20..=0x7e => byte,
            // Not part of the printable ASCII range.
            _ => 0xfe,
        };

        if self.column_position >= BUFFER_WIDTH {
            self.new_line();
        }

        let row = Buffer::last_row();
        let col = self.column_position;

        self.buffer.characters[row][col] = ScreenChar::new(byte, self.color_code);
        self.column_position += 1;
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::blank(self.color_code);
        for col in 0..BUFFER_WIDTH {
            self.buffer.characters[row][col] = blank;
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.characters[row][col];
                self.buffer.characters[row - 1][col] = character;
            }
        }
        self.clear_row(Buffer::last_row());
        self.column_position = 0;
    }
}

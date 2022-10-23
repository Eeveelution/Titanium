use crate::titanium::tty::TtyOutput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
#[repr(u8)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterColor(u8);

impl CharacterColor {
    fn new(foreground: VgaColor, background: VgaColor) -> CharacterColor {
        return CharacterColor( (background as u8) << 4 | (foreground as u8) );
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct ScreenCharacter {
    ascii_character: u8,
    color_code: CharacterColor,
}

impl Clone for ScreenCharacter {
    fn clone(&self) -> ScreenCharacter {
        return ScreenCharacter { ascii_character: self.ascii_character, color_code: self.color_code }
    }
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct VgaBuffer {
    chars: [[ScreenCharacter; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VgaWriter {
    buffer: &'static mut VgaBuffer,
    current_column: usize,
    current_row: usize,
}

impl TtyOutput for VgaWriter {
    fn print(&mut self, bytes: &[u8]) {
        for (_, &byte) in bytes.iter().enumerate() {
            self.print_char(&byte, CharacterColor::new(VgaColor::White, VgaColor::Black));
        }
    }
}

impl VgaWriter {
    fn new() -> VgaWriter {
        VgaWriter { 
            buffer: unsafe { &mut *(0xb8000 as *mut VgaBuffer) }, 
            current_column: 0, 
            current_row: 0 
        }
    }

    fn print_char(&mut self, data: &u8, color: CharacterColor) {
        match data {
            b'\n' => {
                self.current_row += 1;
                self.current_column = 0;
            },
            byte => {
                if self.current_column >= BUFFER_WIDTH {
                    self.current_row += 1;
                    self.current_column = 0;
                }

                self.buffer.chars[self.current_row][self.current_column] = ScreenCharacter {
                    ascii_character: byte.clone(),
                    color_code: color
                };

                self.current_column += 1;
            }
        }
    }
}

pub fn create_tty_output() -> impl TtyOutput {
    return VgaWriter::new();
}
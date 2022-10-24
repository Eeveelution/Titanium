use lazy_static::lazy_static;
use vga::writers::{Graphics320x240x256, GraphicsWriter};

lazy_static! {
    pub static ref GRAPHICS_MODE_320_240_256: Graphics320x240x256 = Graphics320x240x256::new();
}

pub fn initialize() {
    GRAPHICS_MODE_320_240_256.set_mode();    
}
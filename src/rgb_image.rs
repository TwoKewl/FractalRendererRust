
use crate::rgb::Rgb;

use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct RgbImage {
    pub width: u32,
    pub height: u32,
    bytes: Vec<u8>
}

impl RgbImage {
    pub fn new(width: u32, height: u32) -> Self {
        RgbImage {
            width: width as u32,
            height: height as u32,
            bytes: vec![0; width as usize * height as usize * 3]
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Rgb) {
        let index: usize = (y * self.width as usize + x) * 3;
        self.bytes[index] = pixel.b;
        self.bytes[index + 1] = pixel.g;
        self.bytes[index + 2] = pixel.r;
    }

    pub fn write_to_file(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        
        file.write("BM".as_bytes()).unwrap();
        file.write((54 as u32 + self.width * self.height * 3u32).to_le_bytes().as_ref()).unwrap();
        file.write(0u32.to_le_bytes().as_ref()).unwrap();
        file.write(54u32.to_le_bytes().as_ref()).unwrap();

        file.write(40u32.to_le_bytes().as_ref()).unwrap();
        file.write(self.width.to_le_bytes().as_ref()).unwrap();
        file.write(self.height.to_le_bytes().as_ref()).unwrap();
        file.write(1u16.to_le_bytes().as_ref()).unwrap();
        file.write(24u16.to_le_bytes().as_ref()).unwrap();
        file.write(0u32.to_le_bytes().as_ref()).unwrap();
        file.write((self.width * self.height * 3u32).to_le_bytes().as_ref()).unwrap();
        file.write(0u32.to_le_bytes().as_ref()).unwrap();
        file.write(0u32.to_le_bytes().as_ref()).unwrap();
        file.write(0u32.to_le_bytes().as_ref()).unwrap();
        file.write(0u32.to_le_bytes().as_ref()).unwrap();

        file.write_all(&self.bytes).unwrap();
    }
}
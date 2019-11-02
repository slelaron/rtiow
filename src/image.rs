use rayon::prelude::*;
use std::io::{Result, Write};
use std::marker::{Send, Sync};
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Image {
    pixels: Vec<Color>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn with_background(height: u32, width: u32, color: Color) -> Image {
        let pixels = vec![color; (width * height) as usize];
        Image {
            pixels,
            width,
            height,
        }
    }

    pub fn new(height: u32, width: u32) -> Image {
        Image::with_background(
            height,
            width,
            Color {
                red: 0,
                blue: 0,
                green: 0,
            },
        )
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn write_ppm(&self, ostream: &mut dyn Write) -> Result<()> {
        writeln!(ostream, "P3\n{0} {1}\n255", self.width, self.height)?;
        for i in 0..self.height {
            for j in 0..self.width {
                let Color { red, green, blue } = self[(i, j)];
                write!(ostream, "{0} {1} {2} ", red, green, blue)?;
            }
            writeln!(ostream)?;
        }
        Ok(())
    }

    pub fn process_in_parallel(&mut self, f: impl Fn(u32, u32) -> Color + Sync + Send) {
        let width = self.width();
        self.pixels
            .par_iter_mut()
            .enumerate()
            .for_each(move |(i, item)| *item = f(i as u32 / width, i as u32 % width));
    }
}

impl Index<(u32, u32)> for Image {
    type Output = Color;

    fn index(&self, (i, j): (u32, u32)) -> &Color {
        &self.pixels[(i * self.width + j) as usize]
    }
}

impl IndexMut<(u32, u32)> for Image {
    fn index_mut(&mut self, (i, j): (u32, u32)) -> &mut Color {
        &mut self.pixels[(i * self.width + j) as usize]
    }
}

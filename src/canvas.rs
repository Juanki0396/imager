use std::{path, fs, io};
use std::io::{LineWriter, Write};

use crate::point::Point;
use crate::figure::Figures;
use crate::constants::{WIDTH, HEIGHT, BACKGROUND_COLOR};

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, color: u32) -> Self {
        return Self { 
            width,
            height,
            pixels: vec![color; width * height],
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn dims(&self) -> (usize, usize) {
        (self.height, self.height)
    }

    pub fn fill(&mut self, color: u32) {
        self.pixels.fill(color)
    }

    pub fn save_to_ppm(&self, img_path: &path::Path) -> io::Result<()> {
        let img_file = fs::File::create(img_path)?;
        let mut img_file = LineWriter::new(img_file);
        img_file.write_all(format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())?;
        for c in self.pixels.chunks(70) {
            let line: String = c.iter().map(|p| Canvas::pix_to_ppm(p.clone())).reduce(|a,s| format!("{a}{s}")).unwrap();
            img_file.write_all(format!("{line}\n").as_bytes())?;
        }
        Ok(())
    }

    fn index(&self, p: &Point) -> Option<usize> {
       if p.x() >= self.width() || p.y() >= self.height() {
           return None;
       }
       Some(p.x() + p.y() * self.width)
    }

    pub fn draw(&mut self, fig: Figures, color: u32) {
        match fig {
            Figures::Rectangle(p, d) => {
                for (i, pix) in self.pixels.iter_mut().enumerate() {
                    let row = i / self.width;
                    let column = i % self.width;
                    if (column >= p.x() && column <= p.x() + d.width()) && (row >= p.y() && row <= p.y() + d.height()){
                        *pix = color;
                    }
                }
            },
            Figures::Circle(p, r) => {
                for (i, pix) in self.pixels.iter_mut().enumerate() {
                    let row = i / self.width;
                    let column = i % self.width;
                    if column.abs_diff(p.x()).pow(2) + row.abs_diff(p.y()).pow(2) <= r.pow(2) {
                        *pix = color;
                    }
                }
            },

            Figures::Line(p1, p2) => {
                let (p1, p2) = p1.sort(p2);
                let (x1, y1) = (p1.x() as i32, p1.y() as i32);
                let (x2, y2) = (p2.x() as i32, p2.y() as i32);
                if x1 == x2 {
                    for (i, pix) in self.pixels.iter_mut().enumerate() {
                        let row = i as i32 / self.width as i32;
                        let column = i as i32 % self.width as i32;
                        if ( column == x1) && (row >= y1 && row <= y2) { 
                            *pix = color;
                        }
                    }
                } else {
                    let m: f64 = (y2 - y1) as f64 / (x2 - x1) as f64;
                    for (i, pix) in self.pixels.iter_mut().enumerate() {
                        let row = i as i32 / self.width as i32;
                        let column = i as i32 % self.width as i32;
                        if (column < x1 || column > x2) || (row < y1 || row > y2) { continue } 
                        if (row - y1) == (m * ( column - x1 ) as f64) as i32 {
                            *pix = color;
                        }
                    }
                }
            },

            Figures::Triangle(p1, p2, p3) => {
               let (p1, p2) = p1.sort(p2);
               let (p2, p3) = p2.sort(p3);
               let (p1, p2) = p1.sort(p2);
               let max_y = p1.y().max(p2.y()).max(p3.y()) as i32;
               let min_y = p1.y().min(p2.y()).min(p3.y()) as i32;
               if p1.y() == p2.y() && p2.y() == p3.y() { 
                   self.draw(Figures::Line(p1,p3), color);
                   return;
               }
               if p1.y() == p2.y() {
                    for (i, pix) in self.pixels.iter_mut().enumerate() {
                        let row = i as i32 / self.width as i32;
                        let column = i as i32 % self.width as i32;
                        if  row < min_y && row > max_y {
                            continue;
                        }
                        if p1.lerp_x(&p3, row as usize).unwrap_or(p1.x()) <= column as usize
                            && p2.lerp_x(&p3, row as usize).unwrap_or(p2.x()) >= column as usize {
                            *pix = color;
                        }
                    }
                } else {
                    let p4 = Point::new(p1.lerp_x(&p3, p2.y()).unwrap_or(p1.x()), p2.y());
                    for (i, pix) in self.pixels.iter_mut().enumerate() {
                        let row = i as i32 / self.width as i32;
                        let column = i as i32 % self.width as i32;
                        if  row < min_y && row > max_y {
                            continue;
                        }
                        if p1.lerp_x(&p2, row as usize).unwrap_or(p1.x()) <= column as usize
                            && p1.lerp_x(&p4, row as usize).unwrap_or(p1.x()) >= column as usize {
                            *pix = color;
                        } else if p2.lerp_x(&p3, row as usize).unwrap_or(p2.x()) <= column as usize
                            && p4.lerp_x(&p3, row as usize).unwrap_or(p4.x()) >= column as usize {
                            *pix = color;
                        }
                    }
               }
            }
            _ => unimplemented!(),
        }
    }

    fn pix_to_ppm(pixel: u32) -> String {
        let r: u8 = (pixel >> 8*0) as u8 & 0xFF; 
        let g: u8 = (pixel >> 8*1) as u8 & 0xFF; 
        let b: u8 = (pixel >> 8*2) as u8 & 0xFF; 
        return format!(" {r} {g} {b} ")
    }

}

impl Default for Canvas {
    fn default() -> Self {
        return Self { 
            width: WIDTH,
            height: HEIGHT,
            pixels: vec![BACKGROUND_COLOR; WIDTH * HEIGHT],
        }
    }
}

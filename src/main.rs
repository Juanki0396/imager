use std::{path, fs, io};
use std::io::{LineWriter, Write};

const HEIGHT: usize = 264;
const WIDTH: usize = 264;
const BACKGROUND_COLOR: u32 = 0xFF333333;
const RED: u32 = 0xFF0000FF;
const GREEN: u32 = 0xFF00FF00;
const BLUE: u32 = 0xFFFF0000;

fn main() {
    let mut img = Canvas::default();
    let img_path = path::Path::new("examples/example.ppm");
    if !check_path(&img_path) {
        eprintln!("ERROR: Invalid path {img_path:?}");
        return;
    }
    img.draw(Figures::new_rectangle(50, 90, 50, 200), GREEN);
    img.draw(Figures::new_circle(100, 100, 50), RED);
    img.draw(Figures::new_line(0, 0, 200, 200), BLUE);
    if let Err(_) = img.save_to_ppm(&img_path) {
        eprintln!("ERROR: Cannot save image into path");
    }
    
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Dims {
    width: usize,
    height: usize,
}

#[derive(Debug)]
enum Figures {
    Triangle(Point, Point, Point),
    Rectangle(Point, Dims),
    Circle(Point, usize),
    Line(Point, Point),
}

impl Figures {
   fn new_rectangle(x: usize, y:usize, width: usize, height: usize) -> Self {
       Self::Rectangle(
           Point { x, y }, 
           Dims { width, height },
           )
   }

   fn new_circle(x: usize, y:usize, r: usize) -> Self {
       Self::Circle(
           Point { x , y }, 
           r
           )
   }

   fn new_line(x1: usize, y1:usize, x2: usize, y2: usize) -> Self {
        Self::Line(
            Point { x: x1, y: y1 },
            Point { x: x2, y: y2 },
            )
   }

   fn new_triangle(x1: usize, y1:usize, x2: usize, y2: usize, x3: usize, y3: usize) -> Self {
        Self::Triangle(
            Point { x: x1, y: y1 },
            Point { x: x2, y: y2 },
            Point { x: x3, y: y3 },
            )
   }
}

#[derive(Debug)]
struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl Canvas {
    fn new(width: usize, height: usize, color: u32) -> Self {
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

    fn fill(&mut self, color: u32) {
        self.pixels.fill(color)
    }

    fn save_to_ppm(&self, img_path: &path::Path) -> io::Result<()> {
        let img_file = fs::File::create(img_path)?;
        let mut img_file = LineWriter::new(img_file);
        img_file.write_all(format!("P3\n{} {}\n255\n", self.height, self.width).as_bytes())?;
        for c in self.pixels.chunks(70) {
            let line: String = c.iter().map(|p| pix_to_ppm(p.clone())).reduce(|a,s| format!("{a}{s}")).unwrap();
            img_file.write_all(format!("{line}\n").as_bytes())?;
        }
        Ok(())
    }

    fn index(&self, p: &Point) -> Option<usize> {
       if p.x >= self.width || p.y >= self.height {
           return None;
       }
       Some(p.x + p.y * self.width)
    }

    fn draw(&mut self, fig: Figures, color: u32) {
        match fig {
            Figures::Rectangle(p, d) => {
                for (i, pix) in self.pixels.iter_mut().enumerate() {
                    let row = i / self.width;
                    let column = i % self.width;
                    if (column >= p.x && column <= p.x + d.width) && (row >= p.y && row <= p.y + d.height){
                        *pix = color;
                    }
                }
            },
            Figures::Circle(p, r) => {
                for (i, pix) in self.pixels.iter_mut().enumerate() {
                    let row = i / self.width;
                    let column = i % self.width;
                    if column.abs_diff(p.x).pow(2) + row.abs_diff(p.y).pow(2) <= r {
                        *pix = color;
                    }
                }
            },

            Figures::Line(p1, p2) => {
                let (p1, p2) = if p1.x < p2.x { (p1,p2) } else { (p2, p1) };
                let (x1, y1) = (p1.x as i32, p1.y as i32);
                let (x2, y2) = (p2.x as i32, p2.y as i32);
                let m: f64 = (y2 - y1) as f64 / (x2 - x1) as f64;
                for (i, pix) in self.pixels.iter_mut().enumerate() {
                    let row = i as i32 / self.width as i32;
                    let column = i as i32 % self.width as i32;
                    if (column < x1 || column > x2) || (row < y1 || row > y2) { continue } 
                    if (row - y1) == (m * ( column - x1 ) as f64) as i32 {
                        *pix = color;
                    }
                }
            },
            Figures::Triangle(p1, p2, p3) => todo!(),
            _ => todo!(),
        }
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

fn check_path(img_path: &path::Path) -> bool {
    !img_path.is_dir() && img_path.extension().is_some()
}

fn write_line(pixels: &mut [u32], p1: (usize, usize), p2: (usize, usize)) {
    todo!();
}

fn write_triangle(pixels: &mut [u32], p1: (usize, usize), p2: (usize, usize), p3: (usize, usize)) {
    todo!();
}
fn write_rectangle(pixels: &mut [u32], p1: (usize, usize), len: usize, height: usize) {
    todo!();
}


fn pix_to_ppm(pixel: u32) -> String {
    let r: u8 = (pixel >> 8*0) as u8 & 0xFF; 
    let g: u8 = (pixel >> 8*1) as u8 & 0xFF; 
    let b: u8 = (pixel >> 8*2) as u8 & 0xFF; 
    return format!(" {r} {g} {b} ")
}


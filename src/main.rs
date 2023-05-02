use std::{path, env, fs};
use std::io::{LineWriter, Write};

const ROWS: usize = 264;
const HEIGHT: usize = 264;

fn main() {
    let img: [u32; ROWS * HEIGHT] = [0xFF0000FF; ROWS * HEIGHT];
    let path = path::Path::new("examples/example.ppm");
    if let Err(e) = save_image_ppm(path, &img, ROWS, HEIGHT){
        eprintln!("ERROR: Cannot save image into path");
    }
    
}

#[derive(Debug)]
enum ImgError {
    ImageErr,
    PathErr,
    IOError
}

fn save_image_ppm(img_path: &path::Path, pixels: &[u32], rows: usize, height: usize) -> Result<(), ImgError> {
    let img_file = match fs::File::create(img_path){
        Err(_) => return Err(ImgError::IOError),
        Ok(f) => f
    };
    let mut img_file = LineWriter::new(img_file);
    img_file.write_all(format!("P3\n{rows} {height}\n255\n").as_bytes()).map_err(|_| ImgError::IOError)?;
    for c in pixels.chunks(70) {
        let line: String = c.iter().map(|p| pix_to_ppm(p.clone())).reduce(|a,s| format!("{a}{s}")).ok_or(ImgError::ImageErr)?;
        img_file.write_all(format!("{line}\n").as_bytes()).map_err(|_| ImgError::IOError)?;
    }
    Ok(())
}

fn pix_to_ppm(pixel: u32) -> String {
    let r: u8 = (pixel >> 8*0) as u8 & 0xFF; 
    let g: u8 = (pixel >> 8*1) as u8 & 0xFF; 
    let b: u8 = (pixel >> 8*2) as u8 & 0xFF; 
    let a: u8 = (pixel >> 8*3) as u8 & 0xFF; 
    return format!(" {r} {g} {b} ")
}


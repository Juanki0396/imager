use std::path;
use imager::figure::Figures;
use imager::canvas::Canvas;
use imager::constants::*;

fn main() {
    let mut img = Canvas::default();
    let img_path = path::Path::new("examples/example.ppm");
    if !check_path(&img_path) {
        eprintln!("ERROR: Invalid path {img_path:?}");
        return;
    }
    img.draw(Figures::new_triangle(0, 100, 200, 0, 200, 200), BLUE | GREEN);
    img.draw(Figures::new_rectangle(50, 90, 50, 200), GREEN);
    img.draw(Figures::new_circle(100, 100, 50), RED);
    img.draw(Figures::new_line(0, 0, 200, 200), BLUE);
    img.draw(Figures::new_line(100, 0, 100, 200), BLUE);
    img.draw(Figures::new_line(0, 100, 200, 100), BLUE);
    if let Err(_) = img.save_to_ppm(&img_path) {
        eprintln!("ERROR: Cannot save image into path");
    }
    
}

fn check_path(img_path: &path::Path) -> bool {
    !img_path.is_dir() && img_path.extension().is_some()
}



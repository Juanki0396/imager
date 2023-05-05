use std::path;
use std::process::ExitCode;

use imager::figure::Figures;
use imager::canvas::Canvas;
use imager::constants::*;

fn main() -> ExitCode {
    let mut img = Canvas::default();
    img.fill(WHITE);
    img.draw(Figures::new_circle(450, 300, 150), RED);

    let img_path = path::Path::new("examples/example.ppm");
    if let Err(_) = img.save_to_ppm(&img_path) {
        eprintln!("ERROR: Cannot save image into path");
        return ExitCode::FAILURE;
    }
    return ExitCode::SUCCESS;
}




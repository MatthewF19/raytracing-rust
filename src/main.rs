use std::io::{stdout, stderr, Write};

mod vec3;
mod color;
mod ray;

use color::Color;

fn main() -> Result<(), std::io::Error> {
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;

    let mut img_height = (img_width as f64 / aspect_ratio) as i32;
    img_height = if img_height < 1 { 1 } else { img_height };

    let viewport_height = 2;
    let viewport_width = viewport_height * (img_width as f64 / img_height as f64) as i32;

    print!("P3\n{} {}\n255\n", img_width, img_height);

    let mut err = stderr();
    let mut out = stdout();
    for j in 0..img_height {
        let buff = format!("\rScanlines remaining: {} ", img_height-j);
        err.write(buff.as_bytes())?;
        for i in 0..img_width {
            let pixel_color: Color = Color::new( (i as f64) / (img_width-1) as f64,
                                                 0.0,
                                                 (j as f64) / (img_height-1) as f64);
            pixel_color.write_color(&mut out);
        }
    }
    err.write(b"\rDone.                 \n")?;

    Ok(())
}

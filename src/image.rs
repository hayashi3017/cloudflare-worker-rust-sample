use std::io::Cursor;

use ab_glyph::FontArc;
use image::Rgb;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn generate_img() -> Vec<u8> {
    //! An example of generating julia fractals.
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.1 * x as f32) as u8;
        let b = (0.1 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // famous julia pattern
    let c_values = [
        num_complex::Complex::new(-0.4, 0.6),
        num_complex::Complex::new(0.355, 0.355),
        num_complex::Complex::new(-0.70176, -0.3842),
        num_complex::Complex::new(0.285, 0.01),
        num_complex::Complex::new(-0.8, 0.156),
        num_complex::Complex::new(0.3, -0.5),
    ];
    let mut rng = thread_rng();
    let c = c_values.choose(&mut rng).unwrap();

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    let font = FontArc::try_from_slice(include_bytes!("../assets/Roboto-Thin.ttf"))
        .expect("Error: Font file is not found.");
    let text = if c.im < 0.0 {
        format!("c: {:?} ({} - {}i)", c.norm(), c.re, -c.im)
    } else {
        format!("c: {:?} ({} + {}i)", c.norm(), c.re, c.im)
    };

    // composite text
    let composite_image =
        imageproc::drawing::draw_text(&imgbuf, Rgb([255, 255, 255]), 10, 10, 30.0, &font, &text);

    // Encode the image as PNG
    let mut buf = Cursor::new(vec![]);
    composite_image
        .write_to(&mut buf, image::ImageFormat::Png)
        .unwrap();
    buf.into_inner()

    // For local testing.
    // Save the image as “fractal.png”, the format is deduced from the path
    // composite_image.save("fractal.png").unwrap();
    // vec![]
}

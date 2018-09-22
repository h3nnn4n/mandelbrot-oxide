extern crate num;
extern crate png;
use num::complex::Complex;

mod image;
mod mandelbrot;

fn main() {
    let outfile_name = String::from("out.png");

    let width: u32 = 800;
    let height: u32 = 600;

    let escape_radius = 2.0;
    let iterations = 1024;
    let mut bytes: Vec<u8> = Vec::new();

    for j in 0..height {
        for i in 0..width {
            let c = mandelbrot::get_c(i, j, width, height);
            let v = mandelbrot::mandelbrot(c, escape_radius, iterations);

            for _ in 0..3 {
                bytes.push(v);
            }
        }

        if (j + 1) % 25 == 0 {
            println!("{} of {}", (j + 1), height);
        }
    }

    let normalized_bytes = image::normalize(&mut bytes);

    image::write_bytes(&outfile_name, &normalized_bytes, width, height);
}

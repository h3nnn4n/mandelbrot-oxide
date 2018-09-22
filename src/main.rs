extern crate num;
extern crate png;
use num::complex::Complex;

mod config;
mod image;
mod mandelbrot;

fn main() {
    let outfile_name = String::from("out.png");

    let conf = config::Config::new();

    let mut bytes: Vec<u8> = Vec::new();

    for j in 0..conf.height {
        for i in 0..conf.width {
            let c = mandelbrot::get_c(i, j, conf.width, conf.height);
            let v = mandelbrot::mandelbrot(c, conf.escape_radius, conf.iterations);

            for _ in 0..3 {
                bytes.push(v);
            }
        }

        if (j + 1) % 25 == 0 {
            println!("{} of {}", (j + 1), conf.height);
        }
    }

    let normalized_bytes = image::normalize(&mut bytes);

    image::write_bytes(&outfile_name, &normalized_bytes, conf.width, conf.height);
}

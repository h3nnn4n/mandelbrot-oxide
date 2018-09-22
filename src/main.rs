extern crate num;
extern crate png;

mod color;
mod config;
mod image;
mod mandelbrot;

fn main() {
    let palette_file = String::from("horizon.png");
    let (palette, _) = color::load_palette(&palette_file);

    let outfile_name = String::from("out.png");

    let conf = config::Config::new();

    let mandelbrot_points = mandelbrot::generate_mandelbrot(conf);

    let normalized = image::normalize(mandelbrot_points);

    let color_image = image::apply_palette(normalized, palette);

    image::write_bytes(&outfile_name, &color_image, conf.width, conf.height);
}

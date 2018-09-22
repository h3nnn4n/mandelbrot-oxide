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

    let mut mandelbrot_points: Vec<u32> = Vec::new();

    for j in 0..conf.height {
        for i in 0..conf.width {
            let c = mandelbrot::get_c(i, j, conf);
            let v = mandelbrot::mandelbrot_point(c, conf.escape_radius, conf.iterations);

            mandelbrot_points.push(v);
        }

        if (j + 1) % 25 == 0 {
            println!("{:4} of {}", (j + 1), conf.height);
        }
    }

    let normalized = image::normalize(&mut mandelbrot_points);

    let color_image = image::apply_palette(normalized, palette);

    image::write_bytes(&outfile_name, &color_image, conf.width, conf.height);
}

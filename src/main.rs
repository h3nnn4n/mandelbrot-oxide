extern crate num;
extern crate png;
use num::complex::Complex;

fn main() {
    let outfile_name = String::from("out.png");

    let width: u32 = 800;
    let height: u32 = 600;

    let mut bytes: Vec<u8> = Vec::new();

    for i in 0..width {
        for j in 0..height {
            let c = get_c(i, j, width, height);
            let v = mandelbrot(c);

            for _ in 0..3 {
                bytes.push(v);
            }
            bytes.push(255);
        }
    }

    write_bytes(&outfile_name, &bytes, width, height);
}

fn get_c(x: u32, y: u32, width: u32, height: u32) -> (Complex<f64>) {
    let xcenter = -0.650;
    let ycenter = 0.0;

    let zoom = -1.500;

    let minx = xcenter + zoom;
    let maxx = xcenter - zoom;
    let miny = ycenter + zoom;
    let maxy = ycenter - zoom;

    let real = minx + (x as f64) * (maxx - minx) / (width as f64);
    let imag = miny + (y as f64) * (maxy - miny) / (height as f64);

    let c = Complex { re: real, im: imag };

    c
}

fn mandelbrot(c: Complex<f64>) -> (u8) {
    let r = (c.norm() * 255.0) as u64;

    (r % 255) as u8
}

fn write_bytes(outfile_name: &String, buf: &Vec<u8>, width: u32, height: u32) -> () {
    use png::HasParameters;
    let path = std::path::Path::new(outfile_name);
    let outfile = std::fs::File::create(path).unwrap();
    let ref mut wr = std::io::BufWriter::new(outfile);
    let mut encoder = png::Encoder::new(wr, width, height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&buf).unwrap();
}

extern crate num;
extern crate png;
use num::complex::Complex;

fn main() {
    let outfile_name = String::from("out.png");

    let width: u32 = 800;
    let height: u32 = 600;

    let escape_radius = 2.0;
    let iterations = 1024;

    let mut bytes: Vec<u8> = Vec::new();

    for j in 0..height {
        for i in 0..width {
            let c = get_c(i, j, width, height);
            let v = mandelbrot(c, escape_radius, iterations);

            for _ in 0..3 {
                bytes.push(v);
            }
        }

        if (j + 1) % 25 == 0 {
            println!("{} of {}", (j + 1), height);
        }
    }

    let normalized_bytes = normalize(&mut bytes);

    write_bytes(&outfile_name, &normalized_bytes, width, height);
}

fn normalize(bytes: &mut Vec<u8>) -> (Vec<u8>) {
    fn norm(x: u8, m: u8) -> (u8) {
        let x2 = x as f64;
        let m2 = m as f64;

        let r = (x2 / m2) * 255.0;

        r as u8
    }

    let m = bytes.iter().max().unwrap();

    bytes.iter().map(|x| norm(*x, *m)).collect()
}

fn get_c(x: u32, y: u32, width: u32, height: u32) -> (Complex<f64>) {
    //let xcenter = -0.650;
    //let ycenter = 0.0;

    //let zoom = -1.500;

    let xcenter = -0.74364085;
    let ycenter = 0.13182733;

    let zoom = 0.00012068;

    let minx = xcenter + zoom;
    let maxx = xcenter - zoom;
    let miny = ycenter + zoom;
    let maxy = ycenter - zoom;

    let real = minx + (x as f64) * (maxx - minx) / (width as f64);
    let imag = miny + (y as f64) * (maxy - miny) / (height as f64);

    Complex { re: real, im: imag }
}

fn mandelbrot(c: Complex<f64>, escape_radius: f64, iterations: u32) -> (u8) {
    let mut z = c.clone();

    for i in 0..iterations {
        z = z.powf(2.0) + c;

        if z.norm() > escape_radius {
            return i as u8;
        }
    }

    return 0;
}

fn write_bytes(outfile_name: &String, buf: &Vec<u8>, width: u32, height: u32) -> () {
    use png::HasParameters;
    let path = std::path::Path::new(outfile_name);
    let outfile = std::fs::File::create(path).unwrap();
    let ref mut wr = std::io::BufWriter::new(outfile);
    let mut encoder = png::Encoder::new(wr, width, height);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&buf).unwrap();
}

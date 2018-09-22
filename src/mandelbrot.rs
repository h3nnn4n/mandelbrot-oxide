extern crate num;
use num::complex::Complex;

pub fn get_c(x: u32, y: u32, width: u32, height: u32) -> (Complex<f64>) {
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

pub fn mandelbrot(c: Complex<f64>, escape_radius: f64, iterations: u32) -> (u8) {
    let mut z = c.clone();

    for i in 0..iterations {
        z = z.powf(2.0) + c;

        if z.norm() > escape_radius {
            return i as u8;
        }
    }

    return 0;
}

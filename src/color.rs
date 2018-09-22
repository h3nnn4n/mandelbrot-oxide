extern crate png;
use image;

pub fn load_palette(filename: &String) -> (Vec<(u8, u8, u8)>, u32) {
    let (buf, width, _) = image::read_bytes(filename);

    let mut palette = Vec::new();

    for i in 0..width {
        let index = (i * 3) as usize;
        palette.push((buf[index], buf[index + 1], buf[index + 2]));
    }

    (palette, width)
}

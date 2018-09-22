extern crate png;

pub fn write_bytes(outfile_name: &String, buf: &Vec<u8>, width: u32, height: u32) -> () {
    use png::HasParameters;

    let path = ::std::path::Path::new(outfile_name);
    let outfile = ::std::fs::File::create(path).unwrap();
    let ref mut wr = ::std::io::BufWriter::new(outfile);
    let mut encoder = png::Encoder::new(wr, width, height);

    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&buf).unwrap();
}

pub fn normalize(bytes: &mut Vec<u8>) -> (Vec<u8>) {
    fn norm(x: u8, m: u8) -> (u8) {
        let x2 = x as f64;
        let m2 = m as f64;

        let r = (x2 / m2) * 255.0;

        r as u8
    }

    let m = bytes.iter().max().unwrap();

    bytes.iter().map(|x| norm(*x, *m)).collect()
}

extern crate png;

fn main() {
    let outfile_name = String::from("out.png");

    let width:u32 = 800;
    let height:u32 = 600;

    let mut bytes:Vec<u8> = Vec::new();

    for i in 0..width {
        for j in 0..height {
            for _ in 0..4 {
                bytes.push((i + j) as u8);
            }
        }
    }

    write_bytes(&outfile_name, &bytes, width, height);
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

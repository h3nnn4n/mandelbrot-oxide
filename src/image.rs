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

pub fn read_bytes(infile_name: &String) -> (Vec<u8>, u32, u32) {
    let decoder = png::Decoder::new(::std::fs::File::open(infile_name).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];

    reader.next_frame(&mut buf).unwrap();

    if info.color_type != png::ColorType::RGB || info.bit_depth != png::BitDepth::Eight {
        println!("Not my kind of PNG!");
        ::std::process::exit(1);
    }

    return (buf, info.width, info.height);
}

pub fn normalize(vec: Vec<u32>) -> (Vec<f32>) {
    fn norm(x: u32, m: u32) -> (f32) {
        (x as f32) / (m as f32)
    }

    let m = vec.iter().max().unwrap();

    vec.iter().map(|x| norm(*x, *m)).collect()
}

pub fn apply_palette(vec: Vec<f32>, palette: Vec<(u8, u8, u8)>) -> (Vec<u8>) {
    fn get_index(i: f32) -> (usize) {
        (i * 255.0) as usize
    }

    let mut image = Vec::new();

    for i in 0..vec.len() {
        let index = get_index(vec[i]);

        let color = palette[index];

        image.push(color.0);
        image.push(color.1);
        image.push(color.2);
    }

    image
}

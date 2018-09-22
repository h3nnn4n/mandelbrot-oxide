pub struct Config {
    pub width: u32,
    pub height: u32,

    pub escape_radius: f64,
    pub iterations: u32,
}

impl Config {
    pub fn new() -> Config {
        Config {
            width: 800,
            height: 600,

            escape_radius: 2.0,
            iterations: 512,
        }
    }
}

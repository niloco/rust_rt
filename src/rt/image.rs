pub struct Image {
    image: Vec<Pixel>,
    x: usize,
    y: usize,
}

impl Image {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            image: vec![Pixel::default(); x * y],
            x: x,
            y: y,
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        self.image.get_mut(y * self.x + x)
    }

    pub fn print(&self) {
        for p in self.image.iter() {
            println!("{} {} {}", p.r, p.g, p.b)
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new() -> Self {
        Pixel::default()
    }

    pub fn from_vals(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
        }
    }
    
    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
}
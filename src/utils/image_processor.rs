pub struct ImageProcessor {
    pub outfile: String,
    pub imgobj: image::DynamicImage,
}

impl ImageProcessor {
    pub fn blur(&mut self, sigma: f32) {
        self.imgobj = self.imgobj.blur(sigma);
    }

    pub fn brighten(&mut self, lux: i32) {
        self.imgobj = self.imgobj.brighten(lux);
    }

    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.imgobj = self.imgobj.crop(x, y, width, height);
    }

    pub fn rotate(&mut self, degree: i32) {
        if degree != 90 && degree != 180 && degree != 270 {
            println!("degrees for rotation must be either 90, 180 or 270.");
            std::process::exit(-1);
        }

        self.imgobj = match degree {
            90 => self.imgobj.rotate90(),
            180 => self.imgobj.rotate180(),
            _ => self.imgobj.rotate270(),
        };
    }

    pub fn invert(&mut self) {
        self.imgobj.invert();
    }

    pub fn grayscale(&mut self) {
        self.imgobj = self.imgobj.grayscale();
    }

    pub fn save_file(self) {
        self.imgobj
            .save(self.outfile)
            .expect("Failed writing OUTFILE.");
    }
}
use std::convert::TryInto;
use std::{error::Error, fs::File};
use std::io::prelude::*;
use image::{ImageBuffer, imageops::contrast, Luma};
use serde::Serialize;

extern crate image;
extern crate rand;

mod kmeans;

const ITERATIONS: i32 = 3;

/// x0 x1 x2
/// x7    x3
/// x6 x5 x4

#[derive(Debug, Serialize)]
struct Bounds {
    x0: (usize, usize),
    x1: (usize, usize),
    x2: (usize, usize),
    x3: (usize, usize),
    x4: (usize, usize),
    x5: (usize, usize),
    x6: (usize, usize),
    x7: (usize, usize),
}

impl Bounds {
    fn from_image(img: ImageBuffer<Luma<u8>, Vec<u8>>) -> Bounds {
	let mut x0 = (usize::max_value(), usize::max_value());
	let mut x1 = (usize::max_value(), usize::max_value());
	let mut x2 = (usize::min_value(), usize::max_value());
	let mut x3 = (usize::min_value(), usize::max_value());
	let mut x4 = (usize::min_value(), usize::min_value());
	let mut x5 = (usize::min_value(), usize::min_value());
	let mut x6 = (usize::max_value(), usize::min_value());
	let mut x7 = (usize::max_value(), usize::max_value());
	
	for x in 0..img.width() {
	    for y in 0..img.height() {
		let px = img.get_pixel(x, y);
		if px[0] == 0 { continue; }

		let x: usize = x.try_into().unwrap();
		let y: usize = y.try_into().unwrap();

		if x < x0.0 && y < x0.1 { x0 = (x, y) };
		if y < x1.0 { x1 = (x, y) };
		if x > x2.0 && y < x2.1 { x2 = (x, y) };
		if x > x3.0 { x3 = (x, y) };
		if x > x4.0 && y > x4.1 { x4 = (x, y) };
		if y > x5.1 { x5 = (x, y) };
		if x < x6.0 && y > x6.1 { x6 = (x, y) };
		if x < x7.0 { x7 = (x, y) };
	    }
	}

	Bounds {
	    x0, x1, x2, x3, x4, x5, x6, x7
	}
    }

    fn write_csv(&self) -> Result<(), Box<dyn Error>> {
	let mut file = File::create("bounds.csv")?;
	file.write_all(b"var,x,y\n")?;
	file.write_all(format!("x0,{},{}\n", self.x0.0, self.x0.1).as_bytes())?;
	file.write_all(format!("x1,{},{}\n", self.x1.0, self.x1.1).as_bytes())?;
	file.write_all(format!("x2,{},{}\n", self.x2.0, self.x2.1).as_bytes())?;
	file.write_all(format!("x3,{},{}\n", self.x3.0, self.x3.1).as_bytes())?;
	file.write_all(format!("x4,{},{}\n", self.x4.0, self.x4.1).as_bytes())?;
	file.write_all(format!("x5,{},{}\n", self.x5.0, self.x5.1).as_bytes())?;
	file.write_all(format!("x6,{},{}\n", self.x6.0, self.x6.1).as_bytes())?;
	file.write_all(format!("x7,{},{}\n", self.x7.0, self.x7.1).as_bytes())?;
	Ok(())
    }
}

fn main() {
    let mut img = image::open("slate.jpg")
        .expect("failed to read image")
        .to_luma();

    // Cluster to split into two colors
    let mut clusters = kmeans::generate_clusters(&mut img, 2);
    
    for _ in 0..ITERATIONS {
	kmeans::iteration(&mut img, &mut clusters);
    }
    kmeans::finalize(&mut img, &mut clusters);

    img = contrast(&img, 100.0); // Picture should be pure black and white to allow further processing
    Bounds::from_image(img).write_csv().unwrap();
}

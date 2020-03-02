use image::{Pixel, Luma};
use rand::{thread_rng, Rng};

pub struct ClusterCenter {
    color: u8,
    pixel_colors: Vec<u8>,
    pixel_coords: Vec<(u32, u32)>
}

impl ClusterCenter {
    fn add_pixel(&mut self, pixel_color: u8, pixel_coord: (u32, u32)) {
        self.pixel_colors.push(pixel_color);
	self.pixel_coords.push(pixel_coord)
    }

    fn reset_pixels(&mut self) {
	self.pixel_colors.clear();
	self.pixel_coords.clear();
    }

    fn recalculate(&mut self) {
        self.color =
            (self.pixel_colors.iter().fold(0, |acc, n| acc + *n as usize) / self.pixel_colors.len()) as u8;
    }
}

pub(crate) fn iteration(
    image: &mut image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>,
    clusters: &mut [ClusterCenter],
) {
    for cluster in clusters.iter_mut() { cluster.reset_pixels(); }

    let (x_size, y_size) = image.dimensions();
    
    for x in 0..x_size {
        for y in 0..y_size {
            let mut threshold = std::f64::INFINITY;
            let mut id = clusters.len() + 1;
            let px = image.get_pixel(x, y);

            for (i, cluster) in clusters.iter().enumerate() {
                let vec_length = f64::abs(
                    cluster.color as f64 * cluster.color as f64 - px[0] as f64 * px[0] as f64,
                );

                if vec_length < threshold {
                    threshold = vec_length;
                    id = i;
                }
            }

            clusters[id].add_pixel(px[0], (x, y));
        }
    }

    for center in clusters {
        center.recalculate();
    }
}

pub(crate) fn finalize(
    image: &mut image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>,
    clusters: &mut [ClusterCenter],
) {
    for cluster in clusters.iter() {
	for coord in cluster.pixel_coords.iter() {
	    let px = Luma::from_channels(cluster.color, 0,0,0);
	    image.put_pixel(coord.0, coord.1, px);
	}
    }
}


pub(crate) fn generate_clusters(
    image: &mut image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>,
    n: usize,
) -> Vec<ClusterCenter> {
    let (x_size, y_size) = image.dimensions();

    let mut rng = thread_rng();
    let cluster_centers: Vec<ClusterCenter> = (0..n)
        .map(|_| ClusterCenter {
            color: rng.gen(),
            pixel_colors: Vec::with_capacity((x_size * y_size) as usize / 2),
	    pixel_coords: Vec::with_capacity((x_size * y_size) as usize / 2),
        })
        .collect();

    cluster_centers
}

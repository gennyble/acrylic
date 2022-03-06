use std::f32::consts::PI;

use crate::{Color, Rgba};

pub struct Image<C: Color> {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) data: Vec<C>,
}

impl<C: Color> Image<C> {
    pub fn new(width: usize, height: usize) -> Self {
        Self::with_color(width, height, C::default())
    }

    pub fn with_color(width: usize, height: usize, color: C) -> Self {
        Self {
            width,
            height,
            data: vec![color; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn rotate(mut self, degrees: f32) -> Self {
        // Rotate corners to get size
        let hx = self.width as f32 / 2.0;
        let hy = self.height as f32 / 2.0;

        // Cartesian
        let top_left = (-hx, hy);
        let top_right = (hx, hy);
        let bottom_right = (hx, -hy);
        let bottom_left = (-hx, -hy);

        let tl_rotated = Self::rotate_point(top_left.0, top_left.1, degrees);
        let tr_rotated = Self::rotate_point(top_right.0, top_right.1, degrees);
        let br_rotated = Self::rotate_point(bottom_right.0, bottom_right.1, degrees);
        let bl_rotated = Self::rotate_point(bottom_left.0, bottom_left.1, degrees);
        println!(
            "{:?} / {:?} - {:?} / {:?}",
            tl_rotated, tr_rotated, br_rotated, bl_rotated
        );

        let highest = tl_rotated
            .1
            .max(tr_rotated.1)
            .max(br_rotated.1)
            .max(bl_rotated.1);
        let rightest = tl_rotated
            .0
            .max(tr_rotated.0)
            .max(br_rotated.0)
            .max(bl_rotated.0);

        let width = (rightest * 2.0).ceil() as usize;
        let height = (highest * 2.0).ceil() as usize;
        let mut vec = vec![C::default(); width * height];

        for x in 0..width {
            for y in 0..height {
                let (rx, ry) = Self::rotate_point(
                    x as f32 - width as f32 / 2.0,
                    (height as f32 - y as f32) - height as f32 / 2.0,
                    -degrees,
                );

                let (rx, ry) = (
                    (rx + hx).round() as usize,
                    (self.height as f32 - (ry + hy)).round() as usize,
                );

                if rx > 0 && rx < self.width && ry > 0 && ry < self.height {
                    vec[y * width + x] = self.data[ry * self.width + rx].clone();
                }
            }
        }

        /*for (index, color) in self.data.into_iter().enumerate() {
            let x = index % self.width;
            let y = index / self.width;
            let (rx, ry) =
                Self::rotate_point(x as f32 - hx, (self.height as f32 - y as f32) - hy, degrees);
            print!("{} {} - ", rx, ry);
            let (rx, ry) = (
                (rx + width as f32 / 2.0) as usize,
                (height as f32 - (ry + height as f32 / 2.0)) as usize,
            );
            println!("{} {}", rx, ry);
            vec[ry * width + rx] = color;
        }*/

        Self {
            width,
            height,
            data: vec,
        }
    }

    fn rotate_point(x: f32, y: f32, degrees: f32) -> (f32, f32) {
        let sin = Self::degrees_to_radians(degrees).sin();
        let cos = Self::degrees_to_radians(degrees).cos();
        (cos * x + sin * y, -sin * x + cos * y)
    }

    fn degrees_to_radians(degrees: f32) -> f32 {
        (PI / 180.0) * degrees
    }
}

impl Image<Rgba> {
    pub fn into_bytes(self) -> Vec<u8> {
        unsafe {
            let mut data = std::mem::transmute::<Vec<Rgba>, Vec<u8>>(self.data);
            data.set_len(data.len() * 4);
            data
        }
    }
}

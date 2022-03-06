use std::{fs::File, io::BufWriter};

use crate::{
    primitives::{Line, Primitive, Rect},
    Color, Image, Rgba,
};

pub struct Canvas {
    image: Image<Rgba>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            image: Image::new(width, height),
        }
    }

    pub fn from_image(image: Image<Rgba>) -> Self {
        Self { image }
    }

    pub fn into_image(self) -> Image<Rgba> {
        self.image
    }

    pub fn image(&self) -> &Image<Rgba> {
        &self.image
    }

    pub fn fill<C: Color>(&mut self, color: C) {
        self.image = Image::with_color(self.image.width(), self.image.height(), color.into());
    }

    pub fn draw<P: Into<Primitive>, C: Color>(&mut self, primitive: P, color: C) {
        match primitive.into() {
            Primitive::Line(line) => self.draw_line(line, color.into()),
            Primitive::Rect(rect) => self.draw_rect(rect, color.into()),
        }
    }

    pub fn save(self) {
        let file = File::create("out.png").unwrap();
        let bufw = BufWriter::new(file);

        let mut encoder =
            png::Encoder::new(bufw, self.image.width() as u32, self.image.height() as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.image.into_bytes()).unwrap();
    }

    fn draw_line(&mut self, line: Line, color: Rgba) {
        let thick = 4;

        println!("Called!");
        let mut x = line.from.x;
        let mut y = line.from.y;
        let delta_x = (line.to.x as isize - line.from.x as isize).abs() + 1;
        let delta_y = -(line.to.y as isize - line.from.y as isize).abs() + 1;
        let step_x: isize = if line.from.x < line.to.x { 1 } else { -1 };
        let step_y: isize = if line.from.y < line.to.y { 1 } else { -1 };
        let mut err = delta_x + delta_y;

        let thick = (thick + 1) / 2;
        loop {
            let mut loop_x = x;
            let mut loop_y = y;
            let mut loop_err = err;

            println!("{} {}", x, y);
            let index = y * self.image.width + x;
            self.image.data[index] = color;

            if err * 2 <= delta_x {
                let mut e2 = delta_x - err;
                loop {
                    if !(e2 < e2 * thick && (line.to.x != loop_x || delta_x < delta_y)) {
                        break;
                    }
                    loop_y = (loop_y as isize + step_y) as usize;

                    self.image.data[loop_y * self.image.width + x] = color;

                    e2 += delta_y;
                }

                if y == line.to.y {
                    break;
                }
                err += delta_x;
                y = (y as isize + step_y) as usize;
            }

            if err * 2 >= delta_y {
                let mut e2 = delta_y - err;

                if x == line.to.x {
                    break;
                }
                err += delta_y;
                x = (x as isize + step_x) as usize;
            }
        }
    }

    fn draw_rect(&mut self, rect: Rect, color: Rgba) {
        let from = rect.top_left();
        let to = rect.bottom_right();

        for x in from.x.max(0)..to.x.min(self.image.width) {
            for y in from.y.max(0)..to.y.min(self.image.height) {
                let index = y * self.image.width + x;
                self.image.data[index] = color;
            }
        }
    }
}

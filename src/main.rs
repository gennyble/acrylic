use std::{fs::File, io::BufWriter, time::Instant};

use acrylic::{
    primitives::{Line, Rect},
    Canvas, Image, Rgba,
};

fn main() {
    let mut can = Canvas::new(256, 256);
    can.fill(Rgba::new(0x33, 0x55, 0x33, 0xFF));
    can.draw(
        Rect::new((64, 64), (128, 128)),
        Rgba::new(0x11, 0x33, 0x55, 0xFF),
    );
    can.draw(
        Line::new((64, 64), (192, 192)),
        Rgba::new(0x55, 0x33, 0x11, 0xFF),
    );

    let img = can.into_image();
    let before = Instant::now();
    //let img = img.rotate(168.0);
    println!(
        "Rotate took {}ms",
        Instant::now().duration_since(before).as_millis()
    );

    write_png(img)
}

fn write_png(image: Image<Rgba>) {
    let file = File::create("out.png").unwrap();
    let bufw = BufWriter::new(file);

    let mut encoder = png::Encoder::new(bufw, image.width() as u32, image.height() as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&image.into_bytes()).unwrap();
}

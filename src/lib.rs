fn draw_text<'a, C>(
    image: &'a mut C,
    color: C::Pixel,
    font: &Font,
    fulltext: &str,
    scale: Scale,
    mid: &(u32, u32),
) where
    C: imageproc::drawing::Canvas,
    <C::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    let (raw_x, raw_y) = mid;
    let text_height = get_height(font, scale);
    let line_count = fulltext.lines().count() as u32;

    println!(
        "raw_x: {}\traw_y: {}\ttext_height: {}\tline_count: {}",
        raw_x, raw_y, text_height, line_count,
    );

    for (index, text) in fulltext.lines().enumerate() {
        let text_width = measure_line_width(font, text, scale);
        let x = *raw_x - (text_width as u32) / 2;
        let y_delta = ((index as f32 - (line_count - 1) as f32 / 2f32) * text_height) as i32;
        let y = *raw_y as i32 + y_delta;
        if y < 0 {
            panic!("Oops");
        }
        let y = y as u32;

        println!("index: {}\ttext: {:?}\tx: {}\ty: {}", index, text, x, y);

        draw_text_mut(image, color, x, y, scale, font, text);
    }
}

use std::io::Cursor;

use conv::ValueInto;
use image::io::Reader;
use image::{DynamicImage, ImageOutputFormat, Pixel, Rgb};
use imageproc::definitions::Clamp;
use imageproc::drawing::draw_text_mut;
use rusttype::{point, Font, Scale};

static FONT_BYTES: &[u8] = include_bytes!("../font.ttf");
static INPUT_FILENAME: &str = "in.png";
static COLOR: [u8; 3] = [0, 0, 0];

fn get_height(font: &Font, scale: Scale) -> f32 {
    let v_metrics = font.v_metrics(scale);
    v_metrics.ascent - v_metrics.descent + v_metrics.line_gap
}

pub fn image_text(texts: Vec<&str>, s: (f32, f32), mids: &[(u32, u32)]) -> DynamicImage {
    let mut base = Reader::new(Cursor::new(
        std::fs::read(INPUT_FILENAME)
            .expect("Failed to read input image")
            .to_vec(),
    ))
    .with_guessed_format()
    .expect("Can't guess format")
    .decode()
    .expect("Unable to decode")
    .to_rgb8();
    let scale = rusttype::Scale { x: s.0, y: s.1 };
    let color = Rgb(COLOR);
    let font = Font::try_from_bytes(FONT_BYTES).expect("Invalid font");
    for (mid, text) in mids.iter().zip(texts.iter()) {
        draw_text(&mut base, color, &font, text, scale, mid);
    }

    DynamicImage::ImageRgb8(base)
}

fn measure_line_width(font: &Font, text: &str, scale: Scale) -> f32 {
    font.layout(text, scale, point(0.0, 0.0))
        .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .last()
        .unwrap_or(0.0)
}

pub fn to_png(image: DynamicImage) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut bytes, ImageOutputFormat::Png)
        .expect("Save failed");
    bytes
}

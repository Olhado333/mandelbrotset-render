use image::Rgb;

pub fn color(iter: Option<u32>) -> image::Rgb<u8> {
    match iter {
        None => Rgb([0, 0, 0]),
        Some(selection ) => match selection % 5 {
            0 => Rgb([255, 0, 0]),
            1 => Rgb([255, 255, 0]),
            2 => Rgb([0, 255, 0]),
            3 => Rgb([0, 255, 255]),
            4 => Rgb([0, 0, 255]),
            _ => Rgb([255, 0, 255]),
        }
    }
}
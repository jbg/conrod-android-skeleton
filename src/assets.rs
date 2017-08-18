use android_glue;
use image::{self, DynamicImage};
use rusttype::{Font, FontCollection};

pub fn load_font(filename: &str) -> Font {
    match android_glue::load_asset(filename) {
        Ok(data) => FontCollection::from_bytes(data).into_font().unwrap(),
        Err(_) => panic!("Can't load font.")
    }
}

pub fn load_image(filename: &str) -> DynamicImage {
    match android_glue::load_asset(filename) {
        Ok(data) => image::load_from_memory(&data).unwrap(),
        Err(_) => panic!("Can't load image.")
    }
}

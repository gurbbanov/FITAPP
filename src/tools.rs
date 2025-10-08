use egui::{self, ColorImage, Context, TextureHandle, TextureOptions};
use time::{Weekday};

// pub fn load_png(ctx: &Context, bytes: &[u8]) -> TextureHandle {
//     let image = image::load_from_memory(bytes).unwrap();
//     let size = [image.width() as usize, image.height() as usize];
//     let image_buffer = image.to_rgba8();
//     let pixels = image_buffer.as_flat_samples();

//     let egui_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
//     ctx.load_texture("background", egui_image, TextureOptions::LINEAR)
// }

pub fn load_png(ctx: &Context, bytes: &[u8]) -> Result<TextureHandle, Box<dyn std::error::Error>> {
    let image = image::load_from_memory(bytes)?;
    let size = [image.width() as usize, image.height() as usize];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    let egui_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
    Ok(ctx.load_texture("ambient_test", egui_image, TextureOptions::LINEAR))
}

pub fn weekday_iso(weekday: Weekday) -> u8 {
    match weekday { 
        time::Weekday::Monday => 1,
        time::Weekday::Tuesday => 2,
        time::Weekday::Wednesday => 3,
        time::Weekday::Thursday => 4,
        time::Weekday::Friday => 5,
        time::Weekday::Saturday => 6,
        time::Weekday::Sunday => 7,
    }
}
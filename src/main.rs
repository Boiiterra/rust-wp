mod perlin_lib;

use image::{ImageBuffer, Rgb};
use perlin_lib::PerlinNoise;

fn lerp(a: i32, b: i32, t: f64) -> i32 {
    (a as f64 + (b - a) as f64 * t) as i32
}

fn colour(channel: &PerlinNoise, posx: u32, posy: u32) -> u8 {
    lerp(0, 255, channel.perlin2d(posx as f64, posy as f64, 0.01, 5)) as u8
}

fn main() {
    let imgx = 1920;
    let imgy = 1080;

    let r_channel = PerlinNoise::new(1, true);
    // let g_channel = &r_channel;
    // let b_channel = &r_channel;
    let g_channel = PerlinNoise::new(2, true);
    let b_channel = PerlinNoise::new(3, true);

    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    println!("STATUS -> Generating image.");
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = colour(&r_channel, x, y);
        let g = colour(&g_channel, x, y);
        let b = colour(&b_channel, x, y);
        *pixel = Rgb([r, g, b]);
    }

    imgbuf.save("wallpaper.png").unwrap();
    println!("STATUS -> wallpaper.png is generated with no issues.");
}

// Generates terrain image 1920x1080

mod perlin_lib;

use image::{ImageBuffer, Rgb};
use perlin_lib::PerlinNoise;

fn clamp01(val: f64) -> f64 {
    if val > 1.0 {
        1.0
    } else if val < 0.0 {
        0.0
    } else {
        val
    }
}

fn lerp(a: i32, b: i32, t: f64) -> i32 {
    (a as f64 + (b - a) as f64 * t) as i32
}

fn invlerp(a: f64, b: f64, t: f64) -> f64 {
    if a != b {
        clamp01((t - a) / (b - a))
    } else {
        0.0
    }
}

fn _colour(channel: &PerlinNoise, posx: u32, posy: u32) -> u8 {
    lerp(0, 255, channel.perlin2d(posx as f64, posy as f64, 0.01, 5)) as u8
}

pub fn terrain() {
    let imgx = 1920;
    let imgy = 1080;

    let c_ch = PerlinNoise::new(63, true);

    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    println!("STATUS -> Generating image.");
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut r: u8 = 0;
        let mut g: u8 = 0;
        let mut b: u8 = 0;

        let t = c_ch.perlin2d(x as f64, y as f64, 0.01, 5);
        if t <= 0.43 {
            let nt: f64 = invlerp(0.0, 0.43, t);
            b = lerp(50, 170, nt) as u8;
        }

        if t > 0.43 && t <= 0.5 {
            let nt: f64 = 1.0 - invlerp(0.43, 0.5, t);
            let val: u8 = lerp(120, 150, nt) as u8;
            g = val;
            r = val;
        }

        if t > 0.5 && t <= 0.723 {
            let nt: f64 = 1.0 - invlerp(0.56, 0.723, t);
            g = lerp(100, 125, nt) as u8;
        }

        if t > 0.723 {
            let nt: f64 = invlerp(0.723, 1.0, t);
            let val: u8 = lerp(100, 255, nt) as u8;
            r = val;
            g = val;
            b = val;
        }

        *pixel = Rgb([r, g, b]);
    }

    imgbuf.save("terrain.png").unwrap();
    println!("STATUS -> terrain.png is generated with no issues.");
}

fn main() {
    terrain();
}

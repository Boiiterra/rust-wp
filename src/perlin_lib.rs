// perlin noise from: https://gist.github.com/nowl/828013

use rand::Rng;

pub struct PerlinNoise {
    seed: i32,
    hash: [i32; 256],
}

impl Default for PerlinNoise {
    fn default() -> PerlinNoise {
        PerlinNoise {
            seed: 1,
            hash: [
                208, 34, 231, 213, 32, 248, 233, 56, 161, 78, 24, 140, 71, 48, 140, 254, 245, 255,
                247, 247, 40, 185, 248, 251, 245, 28, 124, 204, 204, 76, 36, 1, 107, 28, 234, 163,
                202, 224, 245, 128, 167, 204, 9, 92, 217, 54, 239, 174, 173, 102, 193, 189, 190,
                121, 100, 108, 167, 44, 43, 77, 180, 204, 8, 81, 70, 223, 11, 38, 24, 254, 210,
                210, 177, 32, 81, 195, 243, 125, 8, 169, 112, 32, 97, 53, 195, 13, 203, 9, 47, 104,
                125, 117, 114, 124, 165, 203, 181, 235, 193, 206, 70, 180, 174, 0, 167, 181, 41,
                164, 30, 116, 127, 198, 245, 146, 87, 224, 149, 206, 57, 4, 192, 210, 65, 210, 129,
                240, 178, 105, 228, 108, 245, 148, 140, 40, 35, 195, 38, 58, 65, 207, 215, 253, 65,
                85, 208, 76, 62, 3, 237, 55, 89, 232, 50, 217, 64, 244, 157, 199, 121, 252, 90, 17,
                212, 203, 149, 152, 140, 187, 234, 177, 73, 174, 193, 100, 192, 143, 97, 53, 145,
                135, 19, 103, 13, 90, 135, 151, 199, 91, 239, 247, 33, 39, 145, 101, 120, 99, 3,
                186, 86, 99, 41, 237, 203, 111, 79, 220, 135, 158, 42, 30, 154, 120, 67, 87, 167,
                135, 176, 183, 191, 253, 115, 184, 21, 233, 58, 129, 233, 142, 39, 128, 211, 118,
                137, 139, 255, 114, 20, 218, 113, 154, 27, 127, 246, 250, 1, 8, 198, 250, 209, 92,
                222, 173, 21, 88, 102, 219,
            ],
        }
    }
}

impl PerlinNoise {
    pub fn new(seed: i32, rng_hash: bool) -> PerlinNoise {
        if rng_hash {
            let mut new_hash: [i32; 256] = [0; 256];
            let mut rng = rand::thread_rng();
            for i in 0..256 {
                new_hash[i] = rng.gen_range(0..256);
            }
            return PerlinNoise {
                seed,
                hash: new_hash,
            };
        }
        PerlinNoise {
            seed,
            ..PerlinNoise::default()
        }
    }

    fn noise2(&self, x: i32, y: i32) -> i32 {
        let mut yindex: i32 = (y + self.seed) % 256;
        if yindex < 0 {
            yindex += 256;
        }
        let mut xindex = (self.hash[yindex as usize] + x) % 256;
        if xindex < 0 {
            xindex += 256;
        }
        self.hash[xindex as usize]
    }

    fn lin_inter(x: f64, y: f64, s: f64) -> f64 {
        x + s * (y - x)
    }

    fn smooth_inter(x: f64, y: f64, s: f64) -> f64 {
        PerlinNoise::lin_inter(x, y, s * s * (3.0 - 2.0 * s))
    }

    fn noise2d(&self, x: f64, y: f64) -> f64 {
        let x_int: i32 = x as i32;
        let y_int: i32 = y as i32;
        let x_frac: f64 = x - x_int as f64;
        let y_frac: f64 = y - y_int as f64;
        let s: i32 = PerlinNoise::noise2(self, x_int, y_int) as i32;
        let t: i32 = PerlinNoise::noise2(self, x_int + 1, y_int) as i32;
        let u: i32 = PerlinNoise::noise2(self, x_int, y_int + 1) as i32;
        let v: i32 = PerlinNoise::noise2(self, x_int + 1, y_int + 1);
        let low: f64 = PerlinNoise::smooth_inter(s as f64, t as f64, x_frac);
        let high: f64 = PerlinNoise::smooth_inter(u as f64, v as f64, x_frac);
        return PerlinNoise::smooth_inter(low, high, y_frac);
    }

    pub fn perlin2d(&self, x: f64, y: f64, freq: f64, depth: u32) -> f64 {
        let mut xa: f64 = x * freq;
        let mut ya: f64 = y * freq;
        let mut amp: f64 = 1.0;
        let mut fin: f64 = 0.0;
        let mut div: f64 = 0.0;

        for _ in 0..depth {
            div += 256.0 * amp;
            fin += PerlinNoise::noise2d(self, xa, ya) * amp;
            amp /= 2.0;
            xa *= 2.0;
            ya *= 2.0;
        }

        fin / (div + (if depth == 0 { 1.0 } else { 0.0 }))
    }
}

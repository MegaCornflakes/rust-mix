use std::cmp::{min, max};
use wasm_bindgen::prelude::*;
use tables::tables::*;

mod tables;

const K1: f64 = 0.031097043125912154;
const K2: f64 = 0.65;
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

#[wasm_bindgen]
pub struct ImageBuffer {
    pigment: Vec<[[f64; 38]; 2]>,
    rgb: Vec<u32>
}

#[wasm_bindgen]
impl ImageBuffer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ImageBuffer {
        ImageBuffer {
            pigment: vec![WHITE_KS[0]; (WIDTH * HEIGHT) as usize],
            rgb: vec![255; (WIDTH * HEIGHT * 4) as usize]
        }
    }

    pub fn init(&mut self) {
        let rgb = pigment_to_rgb(&WHITE_KS[0]);
        for i in 0..WIDTH * HEIGHT {
            let index: usize = (4 * ((i / HEIGHT) * WIDTH + (i % WIDTH))) as usize;
            self.rgb[index] = min(max(0, rgb[0] as u32), 255);
            self.rgb[index + 1] = min(max(0, rgb[1] as u32), 255);
            self.rgb[index + 2] = min(max(0, rgb[2] as u32), 255);
            self.rgb[index + 3] = 255;
        }
    }

    pub fn update(&mut self, x: i32, y: i32, r: i32, color: usize, o: f64) {
        for dy in -r..r {
            if dy + y >= HEIGHT as i32 || dy + y < 0 { continue }
            for dx in -r..r {
                if dx + x >= WIDTH as i32 || dx + x < 0 { continue }
                let index: usize = (WIDTH as i32 * (y + dy) + (x + dx)) as usize;
                let opacity =
                    f64::max(0.,
                             o * (1. -
                                 f64::sqrt(f64::powf(dx as f64, 2.) +
                                    f64::powf(dy as f64, 2.))
                                 / r as f64));
                if opacity == 0. { continue }
                for n in 0..38 {
                    self.pigment[index][0][n] =
                        opacity * COL_KS[color][0][n] +
                            (1. - opacity) * self.pigment[index][0][n];
                    self.pigment[index][1][n] =
                        opacity * COL_KS[color][1][n] +
                            (1. - opacity) * self.pigment[index][1][n];
                }
                let rgb = pigment_to_rgb(&self.pigment[index]);
                self.rgb[4 * index] = min(max(0, rgb[0] as u32), 255);
                self.rgb[4 * index + 1] = min(max(0, rgb[1] as u32), 255);
                self.rgb[4 * index + 2] = min(max(0, rgb[2] as u32), 255);
                self.rgb[4 * index + 3] = 255;
            }
        }
    }
    #[wasm_bindgen(getter)]
    pub fn rgb(&self) -> js_sys::Uint32Array {
        js_sys::Uint32Array::from(&self.rgb[..])
    }
}

fn wavelength_to_rgb(&weights: &[f64; 38]) -> [f64; 3] {
    let mut average_xyz = [0., 0., 0.];
    for i in 0..WL_TO_XYZ.len() {
        for j in 0..3 {
            average_xyz[j] += WL_TO_XYZ[i][j] * weights[i];
        }
    }
    for i in 0..3 {
        average_xyz[i] *= CHROMA_ADJUST[i] * 28.3333333;
    }
    dot(&[average_xyz], &XYZ_TO_RGB)[0]
}

fn pigment_to_ref(pig: &[[f64; 38]; 2]) -> [f64; 38] {
    let mut reflectance: [f64; 38] = [0.; 38];
    for i in 0..38 {
        let r = 1. + pig[0][i] / pig[1][i] - ((pig[0][i] / pig[1][i]).powf(2.) + (2. * pig[0][i]) / pig[1][i]).sqrt();
        reflectance[i] = ((1. - K1) * (1. - K2) * r) / (1. - K2 * r);
    }
    reflectance
}

fn pigment_to_rgb(pig: &[[f64; 38]; 2]) -> [f64; 3] {
    wavelength_to_rgb(&pigment_to_ref(pig))
}

fn dot(a: &[[f64; 3]; 1], b: &[[f64; 3]; 3]) -> [[f64; 3]; 1] {
    let mut c = [[0., 0., 0.]];
    for i in 0..a.len() {
        for j in 0..b[0].len() {
            for k in 0..b.len() {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dot() {
        let a = [[1., 2., 3.]];
        let b = [[4., 5., 6.], [7., 8., 9.], [10., 11., 12.]];
        let c = dot(&a, &b);
        assert_eq!(c, [[48., 54., 60.]]);
    }
    #[test]
    fn test_wavelength_to_rgb() {
        let test_reflectance = [
            0.024397921682939597, 0.031688527788324386, 0.04072236273852973,
            0.04651743351449017, 0.04601290651923267, 0.0499356927452353,
            0.0667534544126962, 0.07697197304088868, 0.07190212194810658,
            0.0608319050156441, 0.04385939136847545, 0.027181263228923776,
            0.014494429987760382, 0.007899000456711562, 0.004800710374541182,
            0.0039007677509941634, 0.003999913462343323, 0.0038997353132813363,
            0.003800393304437832, 0.00429892056978943, 0.005095387923533669,
            0.005985172383253364, 0.007045256460008371, 0.008321795816795345,
            0.009880196202950369, 0.011043173626857456, 0.012173357637494012,
            0.013190239396028133, 0.013009155760964597, 0.0127797934864742,
            0.012365016876425038, 0.012924477576550394, 0.013909444949959795,
            0.01695382464313897, 0.020986765314007807, 0.024790270153378608,
            0.030147645844814365, 0.040403876231291574,
        ];
        assert_eq!(wavelength_to_rgb(&test_reflectance), [2.0086507993906846, 0.8559391227860728, 19.903537616668157])
    }
    #[test]
    fn test_pigment_to_ref() {
        assert_eq!(pigment_to_ref(&COL_KS[4]), [0.022671551606495397, 0.022794107421682083, 0.02276283240786809, 0.02178700653664823, 0.020378472870001208, 0.01868148686182826, 0.017181577787990687, 0.01666255080396113, 0.016079363504351765, 0.015174579846468379, 0.014448567664293505, 0.01462436544072691, 0.017042491307227358, 0.026108510536529215, 0.06455276849864934, 0.17689941796135034, 0.360844066464885, 0.5322114832203123, 0.6570872320104533, 0.7403039086852484, 0.7820369174695251, 0.7987102525758631, 0.8081044802382311, 0.8125404972577264, 0.8160421884093513, 0.8176818793562469, 0.8200087757197079, 0.8217401020850102, 0.8179121076721455, 0.8137798317770332, 0.809941071962822, 0.8081030383517577, 0.8036656175918611, 0.7977559589699674, 0.789459544144161, 0.7804708764950639, 0.7725836773381203, 0.7632573323470572]);
    }
    #[test]
    fn test_pigment_to_rgb() {
        assert_eq!(pigment_to_rgb(&COL_KS[4]), [337.9749375663863, 119.84693793360111, -15.062677037546692])
    }
    #[test]
    fn test_buffer() {
        let mut buffer: ImageBuffer = ImageBuffer::new();
        buffer.pigment[0][0][0] = 1.;
        assert_eq!(buffer.pigment[1], WHITE_KS[0]);
        assert_ne!(buffer.pigment[0], WHITE_KS[0]);
    }
}

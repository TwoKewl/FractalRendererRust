
use std::f32::consts::PI;

use crate::colour_method::ColourMethod;
use crate::mandelbrot_args::MandelArgs;
use crate::rgb::Rgb;

pub fn get_pixel(dist: f32, args: &MandelArgs, colour: &ColourMethod) -> Rgb {
    if args.smooth_colouring {
        if dist == 0.0 {
            return Rgb { r: 0, g: 0, b: 0 };
        }

        match colour {
            ColourMethod::Rainbow => {
                return Rgb {
                    r: (255.0 * (f32::sin(dist) / 2.0 + 0.5)) as u8,
                    g: (255.0 * (f32::sin(dist + PI / 3.0 * 4.0) / 2.0 + 0.5)) as u8,
                    b: (255.0 * (f32::sin(dist + PI / 3.0 * 2.0) / 2.0 + 0.5)) as u8
                }
            },
            ColourMethod::Greyscale => {
                return Rgb {
                    r: (255.0 * (f32::sin(dist) / 2.0 + 0.5)) as u8,
                    g: (255.0 * (f32::sin(dist) / 2.0 + 0.5)) as u8,
                    b: (255.0 * (f32::sin(dist) / 2.0 + 0.5)) as u8
                }
            }
        }
    }

    return Rgb { r: 0, g: 0, b: 0 };
}
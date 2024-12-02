
use crate::mandelbrot_args::{MandelArgs, MandelbrotCoordinates};

const LN2: f32 = 0.693147180559945;

pub fn get_distance_to_set(x: u32, y: u32, args: &MandelArgs) -> f32 {
    match args.coordinates {
        MandelbrotCoordinates::F128(x_min, x_max, y_min, y_max) => {
            let cx = x_min + (x_max - x_min) * (x as f128 / args.width as f128);
            let cy = y_min + (y_max - y_min) * (y as f128 / args.height as f128);
            let mut zx: f128 = 0.0;
            let mut zy: f128 = 0.0;
            let mut zx2: f128;
            let mut zy2: f128;
            
            for i in 0..args.max_iterations {
                zx2 = zx * zx;
                zy2 = zy * zy;

                if zx2 + zy2 > 1024.0 {
                    let log_zn: f32 = (zx2 as f32 + zy2 as f32).ln() / 2.0;
                    let nu: f32 = (log_zn / LN2).ln() / LN2;

                    return (i + 1) as f32 - nu;
                }

                zy = 2.0 * zx * zy + cy;
                zx = zx2 - zy2 + cx;
            }
        },
        MandelbrotCoordinates::F64(x_min, x_max, y_min, y_max) => {
            let cx = x_min + (x_max - x_min) * (x as f64 / args.width as f64);
            let cy = y_min + (y_max - y_min) * (y as f64 / args.height as f64);
            let mut zx: f64 = 0.0;
            let mut zy: f64 = 0.0;
            let mut zx2: f64;
            let mut zy2: f64;
            
            for i in 0..args.max_iterations {
                zx2 = zx * zx;
                zy2 = zy * zy;

                if zx2 + zy2 > 1024.0 {
                    let log_zn: f32 = (zx2 as f32 + zy2 as f32).ln() / 2.0;
                    let nu: f32 = (log_zn / LN2).ln() / LN2;

                    return (i + 1) as f32 - nu;
                }

                zy = 2.0 * zx * zy + cy;
                zx = zx2 - zy2 + cx;
            }
        }
    }

    return 0.0;
}

// pub fn get_iterations_to_escape() -> u32 {
//     panic!("Not implemented");
// }
#![feature(f128)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod colour_method;
mod distance_estimator;
mod image_creator;
mod mandelbrot_args;
mod pixel_generator;
mod rgb;
mod rgb_image;
mod row_generator;

use crate::image_creator::make_image;
use crate::mandelbrot_args::MandelArgs;
use crate::mandelbrot_args::MandelbrotCoordinates;

use crate::colour_method::ColourMethod;

use std::time::Instant;
use std::fs::File;
use std::io::Write;

const X_POINT: f64 = -0.743643887037151;
const Y_POINT: f64 = 0.131825904205330;
const X_POINT_F128: f128 = -0.743643887037151;
const Y_POINT_F128: f128 = 0.131825904205330;

const OUTPUT_DIR: &str = "C:\\Users\\Toby\\Documents\\Rust\\MandelbrotCLIv2\\output\\";

fn main() {
    let mut x_dif: f64 = 4.5;
    let mut y_dif: f64 = 8.0;
    let mut x_dif_f128: f128;
    let mut y_dif_f128: f128;

    let mut coordinates: MandelbrotCoordinates = MandelbrotCoordinates::F64(
        X_POINT - x_dif / 2.0,
        X_POINT + x_dif / 2.0,
        Y_POINT - y_dif / 2.0,
        Y_POINT + y_dif / 2.0,
    );
    
    let mut coordinates_f128: MandelbrotCoordinates;

    let mut args: MandelArgs = MandelArgs {
        width: 1080,
        height: 1920,
        max_iterations: 10000,
        coordinates,
        smooth_colouring: true,
        colour_method: ColourMethod::Rainbow
    };
    
    let mut file = File::create(format!("{}\\..\\output.bin", OUTPUT_DIR)).unwrap();

    for i in 0..800 {
        match &mut coordinates {
            MandelbrotCoordinates::F64(x_min, x_max, y_min, y_max) => {
                *x_min = X_POINT - x_dif / 2.0;
                *x_max = X_POINT + x_dif / 2.0;
                *y_min = Y_POINT - y_dif / 2.0;
                *y_max = Y_POINT + y_dif / 2.0;
                
                args.coordinates = MandelbrotCoordinates::F64(*x_min, *x_max, *y_min, *y_max);
            },
            _ => {}
        }
        
        let now = Instant::now();
        let image = make_image(args.width, args.height, args);
        println!("Finished frame {} in {:.3?}", i, now.elapsed());
        image.write_to_file(&format!("{}\\{:0>4}.bmp", OUTPUT_DIR, i));
        
        x_dif *= 0.97;
        y_dif *= 0.97;
    }

    x_dif_f128 = x_dif as f128;
    y_dif_f128 = y_dif as f128;

    coordinates_f128 = MandelbrotCoordinates::F128(
        X_POINT_F128 - x_dif_f128 / 2.0,
        X_POINT_F128 + x_dif_f128 / 2.0,
        Y_POINT_F128 - y_dif_f128 / 2.0,
        Y_POINT_F128 + y_dif_f128 / 2.0,
    );

    for i in 800..1000 {
        match &mut coordinates_f128 {
            MandelbrotCoordinates::F128(x_min, x_max, y_min, y_max) => {
                *x_min = X_POINT_F128 - x_dif_f128 / 2.0;
                *x_max = X_POINT_F128 + x_dif_f128 / 2.0;
                *y_min = Y_POINT_F128 - y_dif_f128 / 2.0;
                *y_max = Y_POINT_F128 + y_dif_f128 / 2.0;

                args.coordinates = MandelbrotCoordinates::F128(*x_min, *x_max, *y_min, *y_max);
            },
            _ => {}
        }

        let now = Instant::now();
        let image = make_image(args.width, args.height, args);
        println!("Finished frame {} in {:.3?}", i, now.elapsed());
        image.write_to_file(&format!("{}\\{:0>4}.bmp", OUTPUT_DIR, i));

        x_dif_f128 *= 0.97;
        y_dif_f128 *= 0.97;
    }

    match coordinates {
        MandelbrotCoordinates::F64(x_min, x_max, y_min, y_max) => {
            println!("x_min: {}, x_max: {}, y_min: {}, y_max: {}", x_min, x_max, y_min, y_max);
            
            file.write(x_min.to_le_bytes().as_ref()).unwrap();
            file.write(x_max.to_le_bytes().as_ref()).unwrap();
            file.write(y_min.to_le_bytes().as_ref()).unwrap();
            file.write(y_max.to_le_bytes().as_ref()).unwrap();
        },
        MandelbrotCoordinates::F128(x_min, x_max, y_min, y_max) => {
            println!("x_min: {:?}, x_max: {:?}, y_min: {:?}, y_max: {:?}", x_min, x_max, y_min, y_max);

            file.write(x_min.to_le_bytes().as_ref()).unwrap();
            file.write(x_max.to_le_bytes().as_ref()).unwrap();
            file.write(y_min.to_le_bytes().as_ref()).unwrap();
            file.write(y_max.to_le_bytes().as_ref()).unwrap();
        },
    }

}
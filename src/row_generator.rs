use crate::mandelbrot_args::MandelArgs;
use crate::rgb::Rgb;

use crate::colour_method::ColourMethod;
use crate::distance_estimator::get_distance_to_set;
use crate::pixel_generator::get_pixel;

pub fn generate_row(y: u32, args: MandelArgs, colour: ColourMethod) -> Vec<Rgb> {
    let mut row_data: Vec<Rgb> = Vec::new();

    for x in 0..args.width {
        let distance = get_distance_to_set(x, y, &args).sqrt() * 1.5;
        let pixel = get_pixel(distance, &args, &colour);
        row_data.push(pixel);
    }

    row_data
}
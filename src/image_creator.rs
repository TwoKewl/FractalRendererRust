
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

use crate::mandelbrot_args::MandelArgs;
use crate::rgb::Rgb;
use crate::rgb_image::RgbImage;
use crate::row_generator::generate_row;

pub fn make_image(width: u32, height: u32, args: MandelArgs) -> RgbImage {
    let mut img = RgbImage::new(width, height);
    let pool = ThreadPool::new(32);

    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();

        pool.execute(move || {
            let row_data: Vec<Rgb> = generate_row(y, args.clone(), args.colour_method.clone());
            tx.send((y, row_data)).expect("channel will be there waiting for the pool");
        });
    }

    drop(tx);

    for recieved in rx {
        for x in 0..width {
            img.set_pixel(x as usize, recieved.0 as usize, recieved.1[x as usize]);
        }
    }

    img
}
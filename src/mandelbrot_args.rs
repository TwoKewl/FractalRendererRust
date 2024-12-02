
use crate::colour_method::ColourMethod;

#[derive(Clone, Copy)]
pub enum MandelbrotCoordinates {
    F128(f128, f128, f128, f128),
    F64(f64, f64, f64, f64)
}

#[derive(Clone, Copy)]
pub struct MandelArgs {
    pub width: u32,
    pub height: u32,
    pub max_iterations: u32,
    pub coordinates: MandelbrotCoordinates,
    pub smooth_colouring: bool,
    pub colour_method: ColourMethod
}
#[macro_use]
extern crate failure;


extern crate image;
extern crate rusttype;

use rusttype::{point, Font, Scale};
use image::{DynamicImage, Rgba};
use failure::Fail;

pub mod hex;
pub mod avatar;

pub use avatar::AvatarBuilder;
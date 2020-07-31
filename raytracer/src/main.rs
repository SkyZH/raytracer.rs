#![feature(box_syntax)]
#![allow(dead_code)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

mod renderer;
mod scenes;
mod tests;
mod tracer;

use self::renderer::utils::render_high_quality as render;
use self::scenes::cornell_box::cornell_box as scene;
use std::env;

use raytracer_codegen::make_answer;

make_answer!{}

fn main() -> Result<(), std::io::Error> {
    env::set_var("RUST_LOG", "raytracer=info");
    pretty_env_logger::init_custom_env("RUST_LOG");
    info!("generating scene...");
    let (hitable_list, camera, pdf) = scene();
    render(hitable_list, camera, "cornell.png", false, Some(pdf))?;
    Ok(())
}
#![feature(futures_api)]
extern crate wee_alloc;

mod components;
mod drawing_style;
mod fig;
mod float_utils;
mod geom;
mod serializable_app_state;
mod svg;
mod utf_to_binary;
use crate::components::app::App;


// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    yew::start_app::<App>();
}

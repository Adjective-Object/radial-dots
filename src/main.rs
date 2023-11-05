extern crate alloc;
mod components;
mod drawing_style;
mod fig;
mod float_utils;
mod geom;
mod serializable_app_state;
mod svg;
mod utf_to_binary;
mod log;
use crate::components::app::App;

// Use `lol_alloc` as the global allocator.
#[cfg(target_arch = "wasm32")]
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> = LockedAllocator::new(FreeListAllocator::new());

fn main() {
    yew::Renderer::<App>::new().render();
}

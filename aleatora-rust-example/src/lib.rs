mod utils;

use wasm_bindgen::prelude::*;
use aleatora::{osc, SampleRateDependent};
use std::iter::repeat;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut ITER: Option<Box<dyn Iterator<Item = [f64; 2]>>> = None;

// TODO: Wrap unsafe parts in safe Alternator API.
// Ideally, something will automatically fill in and export the `setup` and `process` functions.
// The user of the library can then just write `main` and call `play` at some point to register the stream.
// (Alternator's `setup` will ultimately call the programmer`s `main`.)

pub fn make_composition() -> impl Iterator<Item = [f64; 2]> {
    osc(repeat(400.hz())).zip(osc(repeat(600.hz()))).map(|t| [t.0, t.1])
}

#[wasm_bindgen]
pub fn setup(_sample_rate: f64) {
    aleatora::set_sample_rate(_sample_rate);
    let comp = make_composition();
    unsafe {
        ITER = Some(Box::new(comp));
    }
}

#[wasm_bindgen]
pub fn process(output: &mut [f32]) -> usize {
    let iter = unsafe { ITER.as_mut().unwrap() };
    for frame in output.chunks_mut(2) {
        for (out, sample) in frame.iter_mut().zip(iter.next().unwrap()) {
            *out = sample as f32;
        }
    }
    return output.len()
}

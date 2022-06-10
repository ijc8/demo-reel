mod utils;

use wasm_bindgen::prelude::*;
use aleatora::{osc, pan, SampleRateDependent, Stream};
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
    let a = pan(osc(repeat(400.hz())).mul(repeat(0.5)), osc(repeat(0.25.hz())).add(repeat(1.0)).mul(repeat(0.5)));
    let b = pan(osc(repeat(800.hz())).mul(repeat(0.25)), osc(repeat(0.5.hz())).add(repeat(-1.0)).mul(repeat(0.5)));
    a.zip(b).map(|(x, y)| [x[0] + y[0], x[1] + y[1]])
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

mod utils;

use wasm_bindgen::prelude::*;
use aleatora::{osc, pan, SampleRateDependent, Stream, wave, flip};
use web_sys::{WorkerGlobalScope, Worker, console};
use std::iter::repeat;

static TEST_STRING: &str = r#"{
    "nodes": ["start.wav", "verse1-1.wav", "verse1-2.wav", "chorus.wav"],
    "edges": {
        "0": { "1": 0.5, "2": 0.5 },
        "1": { "3": 1.0 },
        "2": { "3": 1.0 },
        "3": { "0": 0.7, "4": 0.3 }
    }
}
"#;

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
    use serde_json::{Value,Value::Object};
    let v: Value = serde_json::from_str(TEST_STRING).unwrap();
    console::log_1(&v.to_string().into());
    let map = v.as_object().unwrap();
    // TODO: Load audio using `nodes`, build graph using `edges`.
    for (k, v) in map {
        console::log_2(&"key:".into(), &k.into());
    }
    let self_: JsValue = js_sys::global().into();
    let self_: WorkerGlobalScope = self_.into();
    for filename in map["nodes"].as_array().unwrap() {
        let filename = filename.as_str().unwrap();
        let resolve = Closure::new(|_| { console::log_1(&"resolved".into()) });
        let reject = Closure::new(|_| { console::log_1(&"rejected".into()) });
        self_.fetch_with_str(filename).then2(&resolve, &reject);
        resolve.forget();
        reject.forget();
        console::log_1(&filename.into());
    }
    console::log_1(&"yo".into());
    let hmm = std::fs::read_to_string("../zero.wav");
    let zero = include_bytes!("../zero.wav").as_slice();
    let zero = wave::load_mono(zero).into_iter().map(|x| [x, 0.0]);
    let one = include_bytes!("../one.wav").as_slice();
    let one = wave::load_mono(one).into_iter().map(|x| [0.0, x]);
    flip(zero, one).cycle()
    // let a = pan(osc(repeat(400.hz())).mul(repeat(0.5)), osc(repeat(0.25.hz())).add(repeat(1.0)).mul(repeat(0.5)));
    // let b = pan(osc(repeat(800.hz())).mul(repeat(0.25)), osc(repeat(0.5.hz())).add(repeat(-1.0)).mul(repeat(0.5)));
    // a.zip(b).map(|(x, y)| [x[0] + y[0], x[1] + y[1]])
}

#[wasm_bindgen]
pub fn setup(sample_rate: f64, path: &str) {
    aleatora::set_sample_rate(sample_rate);
    console::log_2(&"path".into(), &path.into());
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

mod utils;

use js_sys::{Uint8Array, Object, Map, Array};
use wasm_bindgen::prelude::*;
use aleatora::{AltIterator, Graph, osc, pan, SampleRateDependent, Stream, wave, flip};
use web_sys::{WorkerGlobalScope, Worker, console};
use std::{iter::repeat, collections::HashMap};

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

pub fn make_composition(fs: HashMap<String, Vec<u8>>) -> impl Iterator<Item = [f64; 2]> {
    use serde_json::{Value,Value::Object};
    let bytes = &fs["/graph.json"];
    let text = std::str::from_utf8(bytes).unwrap();
    console::log_2(&"Contents of graph.json".into(), &text.into());

    let v: Value = serde_json::from_slice(bytes).unwrap();
    console::log_1(&v.to_string().into());
    let map = v.as_object().unwrap();
    // TODO: Load audio using `nodes`, build graph using `edges`.
    for (k, v) in map {
        console::log_2(&"key:".into(), &k.into());
    }
    let self_: JsValue = js_sys::global().into();
    let self_: WorkerGlobalScope = self_.into();

    // let graph = Graph::<Box<dyn AltIterator<Item = f64>>, f64>::new();
    for filename in map["nodes"].as_array().unwrap() {
        let mut path = "/".to_owned();
        path.push_str(filename.as_str().unwrap());
        console::log_2(&"About to load:".into(), &(&path).into());
        let bytes = &fs[&path];
        console::log_3(&"node:".into(), &path.into(), &(bytes.len() as u32).into());
    }
    console::log_1(&"cool".into());
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
pub fn setup(sample_rate: f64, files: &Object) {
    console::log_2(&"type?".into(), &files);
    let mut fs = HashMap::<String, Vec<u8>>::new();
    for entry in Object::entries(files).iter() {
        let test: Array = entry.into();
        let mut pair = test.to_vec().into_iter();
        let filename = pair.next().unwrap();
        let buffer = pair.next().unwrap();
        let v: Vec<u8> = Uint8Array::from(buffer).to_vec();
        console::log_2(&filename, &(v.len() as u32).into());
        fs.insert(filename.as_string().unwrap(), v);
    }

    aleatora::set_sample_rate(sample_rate);
    let comp = make_composition(fs);
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

mod utils;

use js_sys::{Uint8Array, Object, Map, Array};
use wasm_bindgen::prelude::*;
use aleatora::{AltIterator, Graph, osc, pan, SampleRateDependent, Stream, wave, flip, GraphIter};
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
    use serde_json::Value;
    let bytes = &fs["/graph.json"];
    let text = std::str::from_utf8(bytes).unwrap();
    console::log_2(&"Contents of graph.json".into(), &text.into());

    let v: Value = serde_json::from_slice(bytes).unwrap();
    console::log_1(&v.to_string().into());
    let map = v.as_object().unwrap();
    for (k, _) in map {
        console::log_2(&"key:".into(), &k.into());
    }

    // Build graph of clips.
    let mut graph = Graph::<Box<dyn AltIterator<Item = f64>>, f64>::new();
    for filename in map["nodes"].as_array().unwrap() {
        let mut path = "/".to_owned();
        path.push_str(filename.as_str().unwrap());
        console::log_2(&"About to load:".into(), &(&path).into());
        let bytes = &fs[&path];
        console::log_3(&"Loaded:".into(), &path.into(), &(bytes.len() as u32).into());
        // TODO: Load in stereo.
        graph.add_node(Box::new(wave::load_mono(bytes.as_slice()).into_iter()));
    }

    for (src, edges) in map["edges"].as_object().unwrap() {
        let src: u32 = src.parse().unwrap();
        let edges = edges.as_object().unwrap();
        for (dst, weight) in edges {
            let dst: u32 = dst.parse().unwrap();
            let weight = weight.as_f64().unwrap();
            console::log_5(&"src:".into(), &src.into(), &"dst:".into(), &dst.into(), &weight.into());
            graph.add_edge(src.into(), dst.into(), weight);
        }
    }
    GraphIter::new(graph).map(|x| [x, x])
}

#[wasm_bindgen]
pub fn setup(sample_rate: f64, files: &Object) {
    console_error_panic_hook::set_once();

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
    for (i, frame) in output.chunks_mut(2).enumerate() {
        match iter.next() {
            Some(x) => for (out, sample) in frame.iter_mut().zip(x) {
                *out = sample as f32;
            },
            None => return i
        }
    }
    output.len()
}

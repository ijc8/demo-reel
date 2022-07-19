mod utils;

use js_sys::{Uint8Array, Object, Array};
use wasm_bindgen::prelude::*;
use aleatora::{AltIterator, Graph, wave, GraphIter};
use std::collections::HashMap;

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
    let json: serde_json::Value = serde_json::from_slice(&fs["/graph.json"]).unwrap();
    let json = json.as_object().unwrap();

    // Build graph of clips.
    let mut graph = Graph::<Box<dyn AltIterator<Item = [f64; 2]>>, f64>::new();
    for filename in json["nodes"].as_array().unwrap() {
        let mut path = "/".to_owned();
        path.push_str(filename.as_str().unwrap());
        graph.add_node(Box::new(wave::load(fs[&path].as_slice()).into_iter()));
    }

    for (src, edges) in json["edges"].as_object().unwrap() {
        let src: u32 = src.parse().unwrap();
        let edges = edges.as_object().unwrap();
        for (dst, weight) in edges {
            let dst: u32 = dst.parse().unwrap();
            let weight = weight.as_f64().unwrap();
            graph.add_edge(src.into(), dst.into(), weight);
        }
    }
    GraphIter::new(graph)
}

#[wasm_bindgen]
pub fn setup(sample_rate: f64, files: &Object) {
    utils::set_panic_hook();

    let mut fs = HashMap::<String, Vec<u8>>::new();
    for entry in Object::entries(files).iter() {
        let entry: Array = entry.into();
        let mut entry = entry.to_vec().into_iter();
        let filename = entry.next().unwrap();
        let buffer = entry.next().unwrap();
        let bytes = Uint8Array::from(buffer).to_vec();
        fs.insert(filename.as_string().unwrap(), bytes);
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

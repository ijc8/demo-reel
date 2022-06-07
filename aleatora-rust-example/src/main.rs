use aleatora::audio::cpal::setup;
use aleatora_rust_example::make_composition;

pub fn main() {
    println!("Test");
    let audio = setup();
    let comp = make_composition();
    let handle = audio.play(comp);
    while handle.is_running() {}
}

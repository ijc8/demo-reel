async function setup(sampleRate) {
    await wasm_bindgen(self.path + "main_bg.wasm")
    wasm_bindgen.setup(sampleRate, self.path)
}

function process(output) {
    return wasm_bindgen.process(output)
}

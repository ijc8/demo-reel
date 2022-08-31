async function setup(sampleRate) {
    // Fetch everything in parallel.
    const loadWasm = wasm_bindgen(self.path + "main_bg.wasm")
    const loadMetadata = (async () => (await fetch(self.path + "track.json")).json())()
    const loadData = (async () => (await (await fetch(self.path + "bundle.data")).blob()).arrayBuffer())()
    const [_, metadata, data] = await Promise.all([loadWasm, loadMetadata, loadData])

    const preloaded = Object.create(null)
    for (const { filename, start, end } of metadata.files) {
        const slice = new Uint8Array(data, start, end - start)
        preloaded[filename] = slice
    }

    wasm_bindgen.setup(sampleRate, preloaded)
}

function process(output) {
    return wasm_bindgen.process(output)
}

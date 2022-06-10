let wasm_bindgen;
(function() {
    const __exports = {};
    let wasm;

    /**
    * @param {number} _sample_rate
    */
    __exports.setup = function(_sample_rate) {
        wasm.setup(_sample_rate);
    };

    let cachegetFloat32Memory0 = null;
    function getFloat32Memory0() {
        if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== wasm.memory.buffer) {
            cachegetFloat32Memory0 = new Float32Array(wasm.memory.buffer);
        }
        return cachegetFloat32Memory0;
    }

    let WASM_VECTOR_LEN = 0;

    function passArrayF32ToWasm0(arg, malloc) {
        const ptr = malloc(arg.length * 4);
        getFloat32Memory0().set(arg, ptr / 4);
        WASM_VECTOR_LEN = arg.length;
        return ptr;
    }
    /**
    * @param {Float32Array} output
    * @returns {number}
    */
    __exports.process = function(output) {
        try {
            var ptr0 = passArrayF32ToWasm0(output, wasm.__wbindgen_malloc);
            var len0 = WASM_VECTOR_LEN;
            const ret = wasm.process(ptr0, len0);
            return ret >>> 0;
        } finally {
            output.set(getFloat32Memory0().subarray(ptr0 / 4, ptr0 / 4 + len0));
            wasm.__wbindgen_free(ptr0, len0 * 4);
        }
    };

    async function load(module, imports) {
        if (typeof Response === 'function' && module instanceof Response) {
            if (typeof WebAssembly.instantiateStreaming === 'function') {
                try {
                    return await WebAssembly.instantiateStreaming(module, imports);

                } catch (e) {
                    if (module.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                    } else {
                        throw e;
                    }
                }
            }

            const bytes = await module.arrayBuffer();
            return await WebAssembly.instantiate(bytes, imports);

        } else {
            const instance = await WebAssembly.instantiate(module, imports);

            if (instance instanceof WebAssembly.Instance) {
                return { instance, module };

            } else {
                return instance;
            }
        }
    }

    async function init(input) {
        if (typeof input === 'undefined') {
            let src;
            if (typeof document === 'undefined') {
                src = location.href;
            } else {
                src = document.currentScript.src;
            }
            input = src.replace(/\.js$/, '_bg.wasm');
        }
        const imports = {};


        if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
            input = fetch(input);
        }



        const { instance, module } = await load(await input, imports);

        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    }

    wasm_bindgen = Object.assign(init, __exports);

})();
async function setup(sampleRate) {
    await wasm_bindgen(self.path + "main_bg.wasm")
    wasm_bindgen.setup(sampleRate)
}

function process(output) {
    return wasm_bindgen.process(output)
}

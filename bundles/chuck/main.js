
var ChucK = (function() {
  var _scriptDir = typeof document !== 'undefined' && document.currentScript ? document.currentScript.src : undefined;
  if (typeof __filename !== 'undefined') _scriptDir = _scriptDir || __filename;
  return (
function(ChucK) {
  ChucK = ChucK || {};



  return ChucK.ready
}
);
})();
if (typeof exports === 'object' && typeof module === 'object')
  module.exports = ChucK;
else if (typeof define === 'function' && define['amd'])
  define([], function() { return ChucK; });
else if (typeof exports === 'object')
  exports["ChucK"] = ChucK;


// ------------------------------------------------------------------------ //

let setDataDir, initChuckInstance, runChuckFile, isShredActive

function initGlobals(Module) {
    setDataDir = Module.cwrap('setDataDir', 'number', ['string'])
    initChuckInstance = Module.cwrap('initChuckInstance', 'number', ['number', 'number', 'number', 'number'])
    runChuckFile = Module.cwrap('runChuckFile', 'number', ['number', 'string'])
    isShredActive = Module.cwrap('isShredActive', 'number', ['number', 'number'])
    // set data dir to "/" for embedded files
    setDataDir("/")
}

const chuckID = 1
const numChannels = 2
let shredID
let Module

const code = `
global int mouseX;
global Event mouseClicked;
SinOsc foo => dac;
300 => float currentBaseFrequency;

// The sine wave's frequency is mapped to 
// the x position of the mouse...
fun void RespondToMouseMovement()
{
    while( true )
    {
        currentBaseFrequency + 0.3 * mouseX => foo.freq;
        10::ms => now;
    }
}
spork ~ RespondToMouseMovement();

//... plus some randomness!
fun void UpdateBaseFrequency()
{
    while( true )
    {
        Math.random2f( 300, 600 ) => currentBaseFrequency;
        200::ms => now;
    }
}
spork ~ UpdateBaseFrequency();

// It will stop playing when you click the mouse
// (at least 1 minute after starting)
// (you can also stop it with the "remove" button below)
5::second => now;
// 1::minute => now;
// mouseClicked => now;
`

const asyncLoadFile = async (url) => {
    return new Uint8Array(await (await fetch(url)).arrayBuffer())
}

let buffer, bufferArray

async function setup(sampleRate) {
    const wasm = await asyncLoadFile(self.path + "/main.wasm")
    const PreModule = {
        wasmBinary: wasm,
        print: console.log,
        printErr: console.err,
        noAudioDecoding: true,
        noImageDecoding: true
    }

    PreModule["preRun"] = () => PreModule["FS_createPreloadedFile"]("/", "main.ck", new TextEncoder().encode(code), true, true)

    Module = await ChucK(PreModule)
    initGlobals(Module)

    initChuckInstance(chuckID, sampleRate, 0, numChannels)
    shredID = runChuckFile(chuckID, "main.ck")
    console.log("Running.")

    const bufferSize = 1024 * numChannels
    buffer = Module._malloc(bufferSize * Float32Array.BYTES_PER_ELEMENT)
    bufferArray = new Float32Array(Module.HEAP32.buffer, buffer, bufferSize)
}

function process(output) {
    if (!isShredActive(chuckID, shredID)) {
        console.log("Done!")
        return 0
    }

    const numFrames = output.length / numChannels
    Module.HEAPF32.set(output, buffer / output.BYTES_PER_ELEMENT)
    // for multichannel, WebAudio uses planar buffers.
    // this version of ChucK has been specially compiled to do the same
    Module._chuckManualAudioCallback(chuckID, 0, buffer, numFrames, 0, numChannels)
    // ...which is awkward, because Alternator already compensates for this. So we first undo it.
    // TODO: Create ChucK WASM build without `__CHUCK_USE_PLANAR_BUFFERS__`.
    for (let f = 0; f < numFrames; f++) {
        for (let c = 0; c < numChannels; c++) {
            output[f * numChannels + c] = bufferArray[c * numFrames + f]
        }
    }

    return output.length
}
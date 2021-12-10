console.log("Loaded main.js.")

const duration = 4*60 + 33
let t = 0
let sampleRate

async function setup(_sampleRate) {
    console.log("Called setup with", _sampleRate)
    sampleRate = _sampleRate
}

function process(output) {
    // Implement first, second, and third movements.
    const samples = Math.min(output.length, (duration - t) * sampleRate)
    t += samples / sampleRate
    return samples
}

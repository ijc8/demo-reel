let freq, dt, phase

async function setup(sampleRate) {
    dt = 1 / sampleRate
    freq = Math.random() * 800 + 200
    phase = 0
}

function process(output) {
    for (let i = 0; i < output.length; i++) {
        output[i] = Math.sin(phase)
        phase +=  2*Math.PI * freq * dt
    }
    return output.length
}

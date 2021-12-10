let sampleRate
let gen

async function setup(_sampleRate) {
    sampleRate = _sampleRate
    gen = generator()
}

function process(output) {
    let i = 0;
    for (; i < output.length; i++) {
        const { value, done } = gen.next()
        if (done) {
            break
        }
        output[i] = value
    }
    return i
}

function shuffle(array) {
    let currentIndex = array.length
    let randomIndex
    while (currentIndex > 0) {
        randomIndex = Math.floor(Math.random() * currentIndex)
        currentIndex--
        [array[currentIndex], array[randomIndex]] = [array[randomIndex], array[currentIndex]]
    }
    return array
}

function* tune() {
    let root = 60
    const seq = [0, 2, 4, 5, 7]
    shuffle(seq)
    seq.push(...seq.slice(0, -1).reverse())
    while (true) {
        for (const pitch of seq) {
            yield root + pitch
        }
        yield* [0, 0, 0]
        root += 1
    }
}

function* generator() {
    let phase = 0
    let dur = 0.5 * sampleRate
    for (const pitch of tune()) {
        const freq = 2**((pitch - 69)/12) * 440
        for (let t = 0; t < dur; t++) {
            const amp = Math.sin(t / dur * Math.PI)
            yield (Math.abs(phase - 0.5)*4 - 1) * amp
            phase = (phase + freq/sampleRate) % 1
        }
        if (pitch === 0) {
            dur *= 0.95
        }
        if (dur < 0.0001) {
            return
        }
    }
}

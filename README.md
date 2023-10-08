### If you're not on <https://gitlab.com/krenyy/rsynth>, then you're on a mirror.

# rsynth

> A simple wave synthesizer that works with JACK and MIDI.

![](./static/demonstration.mp4)

## Usage

`rsynth <path-to-instrument-yml>`

For example:

- `rsynth example.yml`

## Goals

- [x] Synthesize simple waves
- [x] Implement generic oscillator API
- [x] Implement adding multiple simple waves together to make more complex sounds
- [x] Implement envelope
- [x] Create a suitable instrument format
- [ ] Implement single-key polyphony
- [ ] Improve the representation of pressed keys
- [ ] Implement capability to use different instruments for different keys
- [ ] Parse and play .sfz files? (just kidding)

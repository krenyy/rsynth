mod hz;
mod midi;
mod osc;
mod ui;

use crate::hz::Hz;
use crate::osc::Oscillator;
use rayon::prelude::*;

fn main() {
    let (client, _status) =
        jack::Client::new("rsynth", jack::ClientOptions::NO_START_SERVER).unwrap();

    let sample_rate = client.sample_rate();
    let frame_t = 1. / sample_rate as f64;
    let mut time = 0.;

    let midi_in = client.register_port("midi_in", jack::MidiIn).unwrap();
    let mut audio_out = client.register_port("audio_out", jack::AudioOut).unwrap();

    let a4_freq = 440.;
    let step_base = 2f64.powf(1. / 12.);

    let mut keys = [false; 256];
    let oscillator: osc::Amplitude<[Box<dyn Oscillator>; 3]> = osc::Amplitude {
        amplitude: 1.0,
        oscillator: [
            Box::new(osc::Amplitude {
                amplitude: 0.8,
                oscillator: osc::SawtoothFast,
            }),
            Box::new(osc::Amplitude {
                amplitude: 0.7,
                oscillator: osc::Sine,
            }),
            Box::new(osc::Amplitude {
                amplitude: 0.1,
                oscillator: osc::Square,
            }),
        ],
    };
    let oscillator: osc::Amplitude<[Box<dyn Oscillator>; 3]> = osc::Amplitude {
        amplitude: 0.1,
        oscillator: [
            Box::new(osc::Amplitude {
                amplitude: 0.1,
                oscillator: osc::Sawtooth { num_sinewaves: 10 },
            }),
            Box::new(osc::Amplitude {
                amplitude: 1.,
                oscillator: osc::Sine,
            }),
            Box::new(osc::Amplitude {
                amplitude: 0.1,
                oscillator: osc::Square,
            }),
        ],
    };
    let oscillator = osc::Square;

    let handler = jack::ClosureProcessHandler::new(
        move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            let reader = midi_in.iter(ps);
            for v in reader {
                let midi = midi::Midi::try_from(v.bytes).unwrap();

                match midi.message {
                    midi::MidiMessage::NoteOff {
                        key_number,
                        velocity,
                    } => {
                        keys[key_number as usize] = false;
                    }
                    midi::MidiMessage::NoteOn {
                        key_number,
                        velocity,
                    } => {
                        keys[key_number as usize] = true;
                    }
                    msg => unimplemented!("{msg:?}"),
                }
            }

            let audio_slice = audio_out.as_mut_slice(ps);

            audio_slice.par_iter_mut().enumerate().for_each(|(iv, v)| {
                *v = 0.;
                for (i, pressed) in keys.iter().enumerate() {
                    if !pressed {
                        continue;
                    }
                    let frequency = (a4_freq * step_base.powi(i as i32 - 57)).hz();

                    *v += oscillator.value(frequency, time + iv as f64 * frame_t);
                }
            });

            time += frame_t * audio_slice.len() as f64;
            jack::Control::Continue
        },
    );

    let active_client = client.activate_async((), handler).unwrap();

    ui::run();

    active_client.deactivate().unwrap();
}

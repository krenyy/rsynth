mod envelope;
mod hz;
mod instrument;
mod midi;
mod osc;
mod ui;

use crate::{hz::Hz, instrument::Instrument};
use notify::Watcher;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub struct Data {
    pub should_redraw: bool,
    pub instrument: Instrument,
}

fn main() {
    let (client, _status) = jack::Client::new("rsynth", jack::ClientOptions::NO_START_SERVER)
        .expect("failed to create jack client!");

    let sample_rate = client.sample_rate();
    let frame_t = 1. / sample_rate as f64;
    let mut time = 0.;

    let midi_in = client
        .register_port("midi_in", jack::MidiIn)
        .expect("failed to register midi_in port!");
    let mut audio_out = client
        .register_port("audio_out", jack::AudioOut)
        .expect("failed to register audio_out port!");

    let a4_freq = 440.;
    let step_base = 2f64.powf(1. / 12.);

    let data = Arc::new(Mutex::new(Data {
        should_redraw: true,
        instrument: Instrument::read("./example.yml"),
    }));
    let data0 = Arc::clone(&data);
    let data1 = Arc::clone(&data);

    let mut watcher =
        notify::recommended_watcher(move |x: Result<notify::Event, notify::Error>| {
            match x {
                Ok(ev) => match ev.kind {
                    notify::EventKind::Any => (),
                    notify::EventKind::Access(_) => (),
                    notify::EventKind::Create(_) => (),
                    notify::EventKind::Modify(_) => {
                        let _ = std::mem::replace(
                            &mut data0.lock().expect("failed to acquire lock!").instrument,
                            Instrument::read("./example.yml"),
                        );
                        data0.lock().expect("failed to acquire lock!").should_redraw = true;
                    }
                    notify::EventKind::Remove(_) => (),
                    notify::EventKind::Other => (),
                },
                Err(_) => (),
            };
        })
        .expect("failed to create watcher!");
    watcher
        .watch(
            std::path::Path::new("./example.yml"),
            notify::RecursiveMode::NonRecursive,
        )
        .expect("watcher failed! (?)");

    let mut keys = [false; 256];

    let handler = jack::ClosureProcessHandler::new(
        move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            let reader = midi_in.iter(ps);
            for v in reader {
                let midi = midi::Midi::try_from(v.bytes).expect("failed to parse midi event!");

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

            let instrument = &data1.lock().expect("failed to acquire lock!").instrument;

            audio_slice.par_iter_mut().enumerate().for_each(|(iv, v)| {
                *v = 0.;
                for (i, pressed) in keys.iter().enumerate() {
                    if !pressed {
                        continue;
                    }
                    let frequency = (a4_freq * step_base.powi(i as i32 - 57)).hz();

                    *v += instrument
                        .oscillator
                        .value(frequency, time + iv as f64 * frame_t);
                }
            });

            time += frame_t * audio_slice.len() as f64;
            jack::Control::Continue
        },
    );

    let active_client = client
        .activate_async((), handler)
        .expect("failed to activate jack client!");

    ui::run(data);

    active_client
        .deactivate()
        .expect("failed to deactivate jack client!");
}

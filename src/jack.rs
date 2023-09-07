use crate::{
    hz::Hz,
    midi::{ChannelMessageKind, Message, Midi, SystemMessageKind},
    Data,
};
use jack::{AudioOut, Client, ClientOptions, ClosureProcessHandler, Control, MidiIn, ProcessScope};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub fn init(
    data: Arc<Mutex<Data>>,
) -> jack::AsyncClient<(), ClosureProcessHandler<impl FnMut(&Client, &ProcessScope) -> Control>> {
    let (client, _status) = Client::new("rsynth", ClientOptions::NO_START_SERVER)
        .expect("failed to create jack client!");

    let sample_rate = client.sample_rate();
    let frame_t = 1. / sample_rate as f64;
    let mut time = 0.;

    let midi_in = client
        .register_port("midi_in", MidiIn)
        .expect("failed to register midi_in port!");
    let mut audio_out = client
        .register_port("audio_out", AudioOut)
        .expect("failed to register audio_out port!");

    let a4_freq = 440.;
    let step_base = 2f64.powf(1. / 12.);

    let mut keys = [false; 256];

    let handler = ClosureProcessHandler::new(move |_: &Client, ps: &ProcessScope| -> Control {
        let reader = midi_in.iter(ps);
        for v in reader {
            let midi = Midi::try_from(v.bytes).expect("failed to parse midi event!");

            match midi.message {
                Message::ChannelMessage { kind, .. } => match kind {
                    ChannelMessageKind::NoteOff { key_number, .. } => {
                        keys[key_number as usize] = false
                    }
                    ChannelMessageKind::NoteOn { key_number, .. } => {
                        keys[key_number as usize] = true
                    }
                    kind => tracing::warn!("unimplemented ChannelMessageKind: {kind:?}"),
                },
                Message::SystemMessage { kind } => match kind {
                    SystemMessageKind::ActiveSensing => (),
                },
            }
        }

        let audio_slice = audio_out.as_mut_slice(ps);

        let instrument = &data.lock().expect("failed to acquire lock!").instrument;

        audio_slice.par_iter_mut().enumerate().for_each(|(iv, v)| {
            *v = 0.;
            for (i, pressed) in keys.iter().enumerate() {
                if !pressed {
                    continue;
                }
                let frequency = (a4_freq * step_base.powi(i as i32 - 57)).hz();

                *v += instrument.value(frequency, time + iv as f64 * frame_t);
            }
        });

        time += frame_t * audio_slice.len() as f64;

        if time >= f64::MAX {
            time = 0.;
            keys.iter_mut().for_each(|x| *x = false);
        }

        Control::Continue
    });

    let active_client = client
        .activate_async((), handler)
        .expect("failed to activate jack client!");

    active_client
}

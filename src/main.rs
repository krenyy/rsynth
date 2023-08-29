mod hz;
mod midi;
mod oscillators;

use std::io;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

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

    let handler = jack::ClosureProcessHandler::new(
        move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            let reader = midi_in.iter(ps);
            for v in reader {
                let midi = midi::Midi::try_from(v.bytes).unwrap();
                tracing::debug!("{midi:?}");

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
                    msg => tracing::warn!("unimplemented midi message: {msg:?}"),
                }
            }

            for v in audio_out.as_mut_slice(ps).iter_mut() {
                *v = 0.;
                for (i, pressed) in keys.iter().enumerate() {
                    if !pressed {
                        continue;
                    }
                    let frequency = a4_freq * step_base.powi(i as i32 - 57);

                    *v += 0.1
                        * <oscillators::Square as oscillators::Oscillator>::value(frequency, time)
                            as f32;
                }
                time += frame_t;
            }

            jack::Control::Continue
        },
    );

    let active_client = client.activate_async((), handler).unwrap();

    tracing::info!("press any key to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    active_client.deactivate().unwrap();
}

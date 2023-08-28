mod midi;

use std::io;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let (client, _status) =
        jack::Client::new("synth-rs", jack::ClientOptions::NO_START_SERVER).unwrap();

    let sample_rate = client.sample_rate();
    let frame_t = 1. / sample_rate as f64;
    let mut time = 0.;

    let midi_in = client.register_port("midi_in", jack::MidiIn).unwrap();
    let mut audio_out = client.register_port("audio_out", jack::AudioOut).unwrap();

    let frequency_base = 16.35;
    let step_base = 2f64.powf(1. / 12.);

    let mut keys = [false; 256];
    let active_client = client
        .activate_async(
            (),
            jack::ClosureProcessHandler::new(
                move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
                    let reader = midi_in.iter(ps);
                    for v in reader {
                        tracing::debug!("midi event");
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
                            let freq = frequency_base * step_base.powi(i as i32);
                            let x = freq * time * 2. * std::f64::consts::PI;

                            let sine = x.sin();
                            let square = if sine > 0.5 { 1. } else { 0. };
                            let sawtooth = (sine + square) / 2.;

                            *v += 0.1 * sine as f32;
                        }
                        time += frame_t;
                    }

                    jack::Control::Continue
                },
            ),
        )
        .unwrap();

    // std::thread::sleep_ms(3000);

    tracing::info!("press any key to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    active_client.deactivate().unwrap();
}

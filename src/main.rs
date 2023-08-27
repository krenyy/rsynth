mod midi;

use std::io;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let (client, status) =
        jack::Client::new("synth-rs", jack::ClientOptions::NO_START_SERVER).unwrap();

    let sample_rate = client.sample_rate();
    let frame_t = 1. / sample_rate as f64;
    let mut time = 0.;

    let midi_in = client.register_port("midi_in", jack::MidiIn).unwrap();
    let mut audio_out = client.register_port("audio_out", jack::AudioOut).unwrap();

    let mut pressed_keys = [false; 256];
    let active_client = client
        .activate_async(
            (),
            jack::ClosureProcessHandler::new(
                move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
                    let reader = midi_in.iter(ps);
                    for v in reader {
                        let midi = midi::Midi::try_from(v.bytes).unwrap();
                        match midi.message {
                            midi::MidiMessage::NoteOff {
                                key_number,
                                velocity,
                            } => pressed_keys[key_number as usize] = false,
                            midi::MidiMessage::NoteOn {
                                key_number,
                                velocity,
                            } => pressed_keys[key_number as usize] = true,
                            msg => tracing::warn!("unimplemented midi message: {msg:?}"),
                        }
                    }

                    for v in audio_out.as_mut_slice(ps).iter_mut() {
                        *v = 0.;
                        for (i, pressed) in pressed_keys.into_iter().enumerate() {
                            if !pressed {
                                continue;
                            }
                            tracing::trace!(i, pressed);
                            let x = (17. + i as f64) * time * 2. * std::f64::consts::PI;
                            let y = x.sin();
                            *v += y as f32;
                        }
                        time += frame_t;
                    }

                    jack::Control::Continue
                },
            ),
        )
        .unwrap();

    tracing::info!("press any key to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    active_client.deactivate().unwrap();
}

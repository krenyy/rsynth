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

    let mut pressed = false;
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
                            } => pressed = false,
                            midi::MidiMessage::NoteOn {
                                key_number,
                                velocity,
                            } => pressed = true,
                            msg => tracing::warn!("unimplemented midi message: {msg:?}"),
                        }
                    }

                    if pressed {
                        for v in audio_out.as_mut_slice(ps).iter_mut() {
                            let x1 = 220. * time * 2. * std::f64::consts::PI;
                            let y1 = x1.sin();
                            let x2 = 261.63 * time * 2. * std::f64::consts::PI;
                            let y2 = x2.sin();

                            let mixed = 0.5 * y1 + 0.7 * y2;
                            let clipped = mixed.max(-1.).min(1.);

                            *v = clipped as f32;

                            time += frame_t;
                        }
                    }

                    jack::Control::Continue
                },
            ),
        )
        .unwrap();

    // std::thread::sleep_ms(3000);

    tracing::debug!(
        "{:#?}",
        active_client.as_client().ports(
            None,
            Some(jack::PortSpec::jack_port_type(&jack::AudioIn)),
            jack::PortFlags::IS_INPUT,
        )
    );

    for dst_audio in active_client.as_client().ports(
        None,
        Some(jack::jack_sys::FLOAT_MONO_AUDIO),
        jack::PortFlags::IS_INPUT,
    ) {
        active_client
            .as_client()
            .connect_ports_by_name("synth-rs:audio_out", &dst_audio)
            .unwrap();
    }

    tracing::info!("press any key to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    active_client.deactivate().unwrap();
}

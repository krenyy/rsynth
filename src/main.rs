mod midi;

use std::io;

fn main() {
    let (client, status) =
        jack::Client::new("synth-rs", jack::ClientOptions::NO_START_SERVER).unwrap();

    let midi_in = client.register_port("midi_in", jack::MidiIn).unwrap();
    let audio_out = client.register_port("audio_out", jack::AudioOut).unwrap();

    let active_client = client
        .activate_async(
            (),
            jack::ClosureProcessHandler::new(
                move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
                    let reader = midi_in.iter(ps);
                    for x in reader {
                        println!("{}", midi::Midi::try_from(x.bytes).unwrap().message);
                    }
                    jack::Control::Continue
                },
            ),
        )
        .unwrap();

    println!("press any key to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    active_client.deactivate().unwrap();
}

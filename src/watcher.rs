use crate::{envelope::Envelope, instrument::Instrument, osc, Data};
use notify::{
    recommended_watcher, Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

pub fn init(data: Arc<Mutex<Data>>) -> RecommendedWatcher {
    let mut watcher = recommended_watcher(move |x: Result<Event, Error>| {
        match x {
            Ok(ev) => match ev.kind {
                EventKind::Modify(_) => {
                    let _ = std::mem::replace(
                        &mut data.lock().expect("failed to acquire lock!").instrument,
                        Instrument::read("./example.yml").unwrap_or(Instrument {
                            envelope: Envelope::ADSR {
                                attack_time: 0.,
                                decay_time: 0.,
                                sustain_amplitude: 0.,
                                release_time: 0.,
                            },
                            oscillator: Box::new(osc::Zero),
                        }),
                    );
                    data.lock().expect("failed to acquire lock!").should_redraw = true;
                }
                _ => (),
            },
            Err(_) => (),
        };
    })
    .expect("failed to create watcher!");

    watcher
        .watch(Path::new("./example.yml"), RecursiveMode::NonRecursive)
        .expect("watcher failed! (?)");

    watcher
}

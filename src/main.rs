mod envelope;
mod hz;
mod instrument;
mod jack;
mod midi;
mod osc;
mod ui;
mod watcher;

use notify::Watcher;

use crate::instrument::Instrument;
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

pub struct Data {
    pub should_redraw: bool,
    pub instrument: Instrument,
}

fn main() {
    let data = Arc::new(Mutex::new(Data {
        should_redraw: true,
        instrument: Instrument::read("./example.yml").expect("instrument path does not exist!"),
    }));

    let mut watcher = watcher::init(Arc::clone(&data));
    let active_client = jack::init(Arc::clone(&data));

    ui::run(Arc::clone(&data));

    active_client
        .deactivate()
        .expect("failed to deactivate jack client!");

    watcher
        .unwatch(&Path::new("./example.yml"))
        .expect("failed to unwatch file!");
}

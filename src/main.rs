mod envelope;
mod hz;
mod instrument;
mod jack;
mod midi;
mod osc;
mod ui;
mod watcher;

use crate::instrument::Instrument;
use clap::Parser;
use instrument::InstrumentReadError;
use notify::Watcher;
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

pub struct Data {
    pub should_redraw: bool,
    pub instrument: Instrument,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    instrument_path: String,
}

fn main() {
    let args = Args::parse();

    let data = Arc::new(Mutex::new(Data {
        should_redraw: true,
        instrument: Instrument::read(&args.instrument_path).unwrap_or_else(|err| match err {
            InstrumentReadError::IoError(err) => panic!("failed to read instrument!\n{err:?}"),
            InstrumentReadError::Deserialize(err) => {
                panic!("failed to deserialize instrument!\n{err:?}")
            }
        }),
    }));

    let mut watcher = watcher::init(Arc::clone(&data), args.instrument_path);
    let active_client = jack::init(Arc::clone(&data));

    ui::run(Arc::clone(&data));

    active_client
        .deactivate()
        .expect("failed to deactivate jack client!");

    watcher
        .unwatch(&Path::new("./example.yml"))
        .expect("failed to unwatch file!");
}

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Axis, Chart, Dataset, GraphType},
};
use std::io;

use crate::{
    hz::Hz,
    osc::{self, Oscillator},
};

fn ui<B: Backend>(e: Event) -> impl Fn(&mut Frame<'_, B>) {
    let oscillator: osc::Amplitude<[Box<dyn Oscillator>; 3]> = osc::Amplitude {
        amplitude: 1.0,
        oscillator: [
            Box::new(osc::Amplitude {
                amplitude: 0.8,
                oscillator: osc::SawtoothFast,
            }),
            Box::new(osc::Amplitude {
                amplitude: 0.7,
                oscillator: osc::Sine,
            }),
            Box::new(osc::Amplitude {
                amplitude: 0.1,
                oscillator: osc::Square,
            }),
        ],
    };
    let period = 1. / ::std::f64::consts::PI;
    move |f: &mut Frame<B>| {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(f.size());
        let ratio = 1. / layout[0].width as f64;
        let values = (0..layout[0].width)
            .map(|x| {
                let x = x as f64 * ratio * period;
                (
                    x,
                    oscillator.value((2. * ::std::f64::consts::PI).hz(), x) as f64,
                )
            })
            .collect::<Vec<_>>();
        let chart = Chart::new(vec![Dataset::default()
            .name("Wave")
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .data(&values)
            .cyan()])
        .x_axis(
            Axis::default()
                .title("Time")
                .bounds([0., period])
                .labels(["0", "π", "2π"].into_iter().map(Span::from).collect()),
        )
        .y_axis(
            Axis::default()
                .title("Amplitude")
                .bounds([-2., 2.])
                .labels(["-2", "2"].into_iter().map(Span::from).collect()),
        );
        f.render_widget(chart, layout[0]);
    }
}

pub fn run() {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        match event::read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => break,
            e => {
                terminal.draw(ui(e)).unwrap();
            }
        }
    }

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
}

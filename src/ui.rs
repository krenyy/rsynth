use crate::{hz::Hz, Data};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Axis, Chart, Dataset, GraphType},
};
use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
};

pub fn run(data: Arc<Mutex<Data>>) {
    enable_raw_mode().expect("failed to enable raw mode!");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .expect("failed to execute commands!");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("failed to create a terminal!");

    let one_period = 1. / (2. * ::std::f64::consts::PI);
    let num_periods = 2.;
    let period = num_periods * one_period;

    let mut first_draw = true;

    loop {
        if !first_draw
            && event::poll(Duration::from_millis(500)).expect("io error during event poll!")
        {
            match event::read().expect("failed to read an event!") {
                Event::Resize(..) => {
                    data.lock().expect("failed to acquire lock!").should_redraw = true
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => break,
                _ => (),
            }
        }

        if !first_draw && !data.lock().expect("failed to acquire lock!").should_redraw {
            continue;
        }

        terminal
            .draw(|f| {
                let layout = Layout::new()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(100)])
                    .split(f.size());
                let ratio = 1. / layout[0].width as f64;
                let values = (0..layout[0].width)
                    .map(|x| {
                        let x = x as f64 * ratio * period;
                        (
                            x,
                            data.lock()
                                .expect("failed to acquire lock!")
                                .instrument
                                .oscillator
                                .value((2. * ::std::f64::consts::PI).hz(), x)
                                as f64,
                        )
                    })
                    .collect::<Vec<_>>();
                let values_zero = (0..layout[0].width)
                    .map(|x| (x as f64 * ratio * period, 0.))
                    .collect::<Vec<_>>();
                let chart = Chart::new(vec![
                    Dataset::default()
                        .marker(Marker::Braille)
                        .graph_type(GraphType::Line)
                        .data(&values_zero)
                        .dark_gray(),
                    Dataset::default()
                        .marker(Marker::Braille)
                        .graph_type(GraphType::Line)
                        .data(&values)
                        .green(),
                ])
                .x_axis(
                    Axis::default()
                        .title("Time")
                        .bounds([0., period])
                        .labels(["0", "2π", "4π"].into_iter().map(Span::from).collect()),
                )
                .y_axis(
                    Axis::default()
                        .title("Amplitude")
                        .bounds([-1., 1.])
                        .labels(["-1", "0", "1"].into_iter().map(Span::from).collect()),
                );
                f.render_widget(chart, layout[0]);
            })
            .expect("io error during terminal draw!");

        first_draw = false;
        data.lock().expect("failed to acquire lock!").should_redraw = false;
    }

    disable_raw_mode().expect("failed to disable raw mode!");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .expect("failed to execute commands!");
    terminal
        .show_cursor()
        .expect("io error during show_cursor!");
}

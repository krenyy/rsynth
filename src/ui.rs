use crate::{
    hz::Hz,
    osc::{self, Oscillator},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem, ListState},
};
use std::{
    io,
    sync::{Arc, Mutex},
};

pub fn run(oscillator: Arc<Mutex<Box<dyn Oscillator>>>) {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let period = 1. / ::std::f64::consts::PI;
    let mut list_state = ListState::default();

    loop {
        terminal
            .draw(|f| {
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                    .split(f.size());
                let ratio = 1. / layout[0].width as f64;
                let values = (0..layout[0].width)
                    .map(|x| {
                        let x = x as f64 * ratio * period;
                        (
                            x,
                            oscillator
                                .lock()
                                .unwrap()
                                .value((2. * ::std::f64::consts::PI).hz(), x)
                                as f64,
                        )
                    })
                    .collect::<Vec<_>>();
                let values_zero = (0..layout[0].width)
                    .map(|x| {
                        let x = x as f64 * ratio * period;
                        (
                            x,
                            osc::Zero.value((2. * ::std::f64::consts::PI).hz(), x) as f64,
                        )
                    })
                    .collect::<Vec<_>>();
                let chart = Chart::new(vec![
                    Dataset::default()
                        .name("Zero")
                        .marker(Marker::Braille)
                        .graph_type(GraphType::Line)
                        .data(&values_zero)
                        .dark_gray(),
                    Dataset::default()
                        .name("Wave")
                        .marker(Marker::Braille)
                        .graph_type(GraphType::Line)
                        .data(&values)
                        .green(),
                ])
                .x_axis(
                    Axis::default()
                        .title("Time")
                        .bounds([0., period])
                        .labels(["0", "π", "2π"].into_iter().map(Span::from).collect()),
                )
                .y_axis(
                    Axis::default().title("Amplitude").bounds([-2., 2.]).labels(
                        ["-2", "-1", "0", "1", "2"]
                            .into_iter()
                            .map(Span::from)
                            .collect(),
                    ),
                )
                .block(Block::new().title("Chart").borders(Borders::ALL));
                f.render_widget(chart, layout[0]);
                let list = List::new([
                    ListItem::new("LOL"),
                    ListItem::new("DEEZ NUTS"),
                    ListItem::new("LMAO"),
                ])
                .block(Block::new().title("HEY").borders(Borders::ALL))
                .highlight_style(Style::new().black().on_white());
                f.render_stateful_widget(list, layout[1], &mut list_state);
            })
            .unwrap();

        match event::read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Char('h'),
                ..
            }) => {
                let mut osc_lock = oscillator.lock().unwrap();
                let mut osc_fields = osc_lock.get_fields_mut().unwrap();
                let amplitude = osc_fields
                    .get_mut("amplitude")
                    .unwrap()
                    .downcast_mut::<f32>()
                    .unwrap();
                *amplitude -= 0.1
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('l'),
                ..
            }) => {
                let mut osc_lock = oscillator.lock().unwrap();
                let mut osc_fields = osc_lock.get_fields_mut().unwrap();
                let amplitude = osc_fields
                    .get_mut("amplitude")
                    .unwrap()
                    .downcast_mut::<f32>()
                    .unwrap();
                *amplitude += 0.1
            }
            _ => (),
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

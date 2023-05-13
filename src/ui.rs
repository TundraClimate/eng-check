use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use crate::app::App;

pub fn loading<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let text = Paragraph::new("now loading").alignment(Alignment::Center);
    let chunks = Layout::default().direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 2),
            Constraint::Length(1),
            Constraint::Ratio(1, 2),
        ].as_ref())
        .split(area);
    let area = chunks[1];
    let area = Rect { x: area.x, y: area.y - 1, height: area.height, width: area.width };

    f.render_widget(text, area);
}

pub fn main_ui<B: Backend>(app: &App, f: &mut Frame<B>, area: Rect) {
    let outline = Block::default().borders(Borders::ALL);
    let layout = Layout::default().direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(1),
        ].as_ref()).margin(1).split(area);

    f.render_widget(outline, area);
    input_ui(app, f, layout[0]);
    response_ui(app, f, layout[2]);
}

fn input_ui<B: Backend>(app: &App, f: &mut Frame<B>, area: Rect) {
    let outline = Block::default().title("text").borders(Borders::ALL);
    let text = Paragraph::new(Span::raw(app.buffer())).block(outline);

    f.render_widget(text, area);
}

fn response_ui<B: Backend>(app: &App, f: &mut Frame<B>, area: Rect) {
    let outline = Block::default().title("response").borders(Borders::ALL);
    let spans = Spans(app.response().iter().map(|v| Span::raw(v)).collect::<Vec<Span>>());
    let text = Paragraph::new(spans).block(outline);

    f.render_widget(text, area);
}
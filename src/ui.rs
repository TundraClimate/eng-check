use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::widgets::{Paragraph};

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
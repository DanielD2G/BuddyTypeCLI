use crate::data::themes::parse_hex;
use crate::types::{TestMode, TestPhase, ThemeColors};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;

pub struct StatsBar<'a> {
    pub wpm: f64,
    pub accuracy: f64,
    pub elapsed_seconds: f64,
    pub remaining_seconds: f64,
    pub phase: TestPhase,
    pub mode: TestMode,
    pub theme: &'a ThemeColors,
}

impl Widget for StatsBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 {
            return;
        }

        let accent = parse_hex(&self.theme.accent);
        let stats_color = parse_hex(&self.theme.stats);
        let dim = parse_hex(&self.theme.text_dim);

        if self.phase == TestPhase::Idle {
            let line = Line::from(Span::styled(
                "start typing...",
                Style::default().fg(dim),
            ));
            let x = area.x + area.width.saturating_sub(15) / 2;
            buf.set_line(x, area.y, &line, area.width);
            return;
        }

        let time_display = match self.mode {
            TestMode::Time => format!("{}s", self.remaining_seconds.ceil() as u32),
            TestMode::Words => format!("{}s", self.elapsed_seconds.round() as u32),
        };

        let spans = vec![
            Span::styled(
                format!("{}", self.wpm.round() as u32),
                Style::default().fg(accent),
            ),
            Span::styled(" wpm   ", Style::default().fg(stats_color)),
            Span::styled(
                format!("{:.1}%", self.accuracy),
                Style::default().fg(accent),
            ),
            Span::styled(" acc   ", Style::default().fg(stats_color)),
            Span::styled(time_display, Style::default().fg(accent)),
        ];

        let line = Line::from(spans);
        let width: usize = line.width();
        let x = area.x + area.width.saturating_sub(width as u16) / 2;
        buf.set_line(x, area.y, &line, area.width);
    }
}

use crate::data::themes::parse_hex;
use crate::types::ThemeColors;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::Widget;

const LOGO: &[&str] = &[
    r"  ___           _     _      _____                 ",
    r" | _ ) _  _  __| | __| |_  _|_   _|  _ _ __  ___  ",
    r" | _ \| || |/ _` |/ _` | || | | | | | | | '_ \/ -_)",
    r" |___/ \_,_|\__,_|\__,_|\_, | |_|  \_, | .__/\___|",
    r"                       |__/   |__/|_|           ",
];

pub struct Header<'a> {
    pub theme: &'a ThemeColors,
}

impl Widget for Header<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let accent = parse_hex(&self.theme.accent);
        let style = Style::default().fg(accent);

        for (i, line_str) in LOGO.iter().enumerate() {
            if i as u16 >= area.height {
                break;
            }
            let line = Line::from(*line_str).style(style);
            let x = area.x + area.width.saturating_sub(line_str.len() as u16) / 2;
            buf.set_line(x, area.y + i as u16, &line, area.width);
        }
    }
}

pub const HEADER_HEIGHT: u16 = LOGO.len() as u16 + 1;

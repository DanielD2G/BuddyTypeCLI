use crate::data::themes::parse_hex;
use crate::types::ThemeColors;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;

pub struct ResultsChart<'a> {
    pub wpm_history: &'a [f64],
    pub theme: &'a ThemeColors,
    pub terminal_width: u16,
    pub height: u16,
}

impl Default for ResultsChart<'_> {
    fn default() -> Self {
        Self {
            wpm_history: &[],
            theme: &crate::data::themes::get_theme("dark"),
            terminal_width: 50,
            height: 8,
        }
    }
}

impl Widget for ResultsChart<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.wpm_history.len() < 2 || area.height < 3 {
            return;
        }

        let accent = parse_hex(&self.theme.accent);
        let dim = parse_hex(&self.theme.text_dim);
        let accent_style = Style::default().fg(accent);
        let dim_style = Style::default().fg(dim);

        let width = (self.terminal_width.saturating_sub(10) as usize).min(70);
        let height = self.height as usize;

        let max_wpm = self
            .wpm_history
            .iter()
            .cloned()
            .fold(1.0_f64, f64::max);
        let min_wpm = self
            .wpm_history
            .iter()
            .cloned()
            .fold(f64::MAX, f64::min);
        let avg_wpm: f64 = self.wpm_history.iter().sum::<f64>() / self.wpm_history.len() as f64;

        // Resample to fit width
        let samples: Vec<f64> = (0..width)
            .map(|i| {
                let idx = (i as f64 / width as f64 * self.wpm_history.len() as f64).floor() as usize;
                self.wpm_history[idx.min(self.wpm_history.len() - 1)]
            })
            .collect();

        let blocks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

        let mut y = area.y;

        // Title
        let title = Line::from(Span::styled("wpm over time", dim_style));
        let x_center = area.x + area.width.saturating_sub(width as u16) / 2;
        buf.set_line(x_center, y, &title, area.width);
        y += 1;

        // Chart rows (top to bottom)
        for row in (0..height).rev() {
            if y >= area.y + area.height {
                break;
            }
            let mut line_str = String::with_capacity(width);
            for col in 0..samples.len() {
                let normalized = samples[col] / max_wpm;
                let bar_height = normalized * height as f64;

                if bar_height >= (row + 1) as f64 {
                    line_str.push('█');
                } else if bar_height > row as f64 {
                    let frac = bar_height - row as f64;
                    let block_idx =
                        (frac * blocks.len() as f64).floor() as usize;
                    let block_idx = block_idx.min(blocks.len() - 1);
                    line_str.push(blocks[block_idx]);
                } else {
                    line_str.push(' ');
                }
            }
            let line = Line::from(Span::styled(line_str, accent_style));
            buf.set_line(x_center, y, &line, area.width);
            y += 1;
        }

        // Separator
        if y < area.y + area.height {
            let sep = "─".repeat(width);
            let line = Line::from(Span::styled(sep, dim_style));
            buf.set_line(x_center, y, &line, area.width);
            y += 1;
        }

        // Stats line
        if y < area.y + area.height {
            let stats = format!(
                "max: {}  avg: {}  min: {}",
                max_wpm.round() as u32,
                avg_wpm.round() as u32,
                min_wpm.round() as u32
            );
            let line = Line::from(Span::styled(stats, dim_style));
            buf.set_line(x_center, y, &line, area.width);
        }
    }
}

pub const CHART_HEIGHT: u16 = 12; // title + 8 rows + separator + stats + margin

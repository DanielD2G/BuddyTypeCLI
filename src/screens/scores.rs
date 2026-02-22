use crate::app::App;
use crate::config::store::get_scores;
use crate::data::themes::parse_hex;
use crate::types::{TestMode, ThemeColors};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::Frame;

pub fn render(frame: &mut Frame, area: Rect, app: &App, theme: &ThemeColors) {
    let scores = get_scores();
    let accent = parse_hex(&theme.accent);
    let text_color = parse_hex(&theme.text);
    let dim = parse_hex(&theme.text_dim);

    let chunks = Layout::vertical([
        Constraint::Length(2), // title
        Constraint::Min(5),   // table
        Constraint::Length(2), // footer
    ])
    .split(area);

    // Title
    let title = Line::from(Span::styled(
        "scores",
        Style::default()
            .fg(accent)
            .add_modifier(Modifier::BOLD),
    ));
    let tw = title.width() as u16;
    let tx = chunks[0].x + chunks[0].width.saturating_sub(tw) / 2;
    frame.render_widget(
        title,
        Rect::new(tx, chunks[0].y, chunks[0].width, 1),
    );

    if scores.is_empty() {
        let msg = Line::from(Span::styled(
            "no scores yet — complete a test first",
            Style::default().fg(dim),
        ));
        let mw = msg.width() as u16;
        let mx = chunks[1].x + chunks[1].width.saturating_sub(mw) / 2;
        frame.render_widget(
            msg,
            Rect::new(mx, chunks[1].y + 2, chunks[1].width, 1),
        );
    } else {
        let best_wpm = scores.iter().map(|s| s.wpm).max().unwrap_or(0);
        let visible_rows = chunks[1].height.saturating_sub(2) as usize; // header + separator
        let scroll = app.scores_scroll;

        let table_area = chunks[1];
        let mut y = table_area.y;

        // Header
        let header = Line::from(vec![
            Span::styled(pad("#", 4), Style::default().fg(dim)),
            Span::styled(" ", Style::default()),
            Span::styled(pad("WPM", 6), Style::default().fg(dim)),
            Span::styled(" ", Style::default()),
            Span::styled(pad("Raw", 6), Style::default().fg(dim)),
            Span::styled(" ", Style::default()),
            Span::styled(pad("Acc", 7), Style::default().fg(dim)),
            Span::styled(" ", Style::default()),
            Span::styled(pad("Lang", 12), Style::default().fg(dim)),
            Span::styled(" ", Style::default()),
            Span::styled(pad("Mode", 10), Style::default().fg(dim)),
            Span::styled(" ", Style::default()),
            Span::styled("Date", Style::default().fg(dim)),
        ]);
        let hx = table_area.x + table_area.width.saturating_sub(65) / 2;
        frame.render_widget(
            header,
            Rect::new(hx, y, table_area.width, 1),
        );
        y += 1;

        // Separator
        let sep = Line::from(Span::styled(
            "─".repeat(65),
            Style::default().fg(dim),
        ));
        frame.render_widget(sep, Rect::new(hx, y, table_area.width, 1));
        y += 1;

        // Rows
        let visible = &scores[scroll..(scroll + visible_rows).min(scores.len())];
        for (i, score) in visible.iter().enumerate() {
            if y >= table_area.y + table_area.height {
                break;
            }
            let idx = scroll + i + 1;
            let is_best = score.wpm == best_wpm;
            let wpm_color = if is_best { accent } else { text_color };

            let mode_str = match score.mode {
                TestMode::Time => format!("time {}s", score.duration),
                TestMode::Words => format!("words {}w", score.duration),
            };

            let date_str = format_date(&score.date);

            let row = Line::from(vec![
                Span::styled(pad(&idx.to_string(), 4), Style::default().fg(dim)),
                Span::styled(" ", Style::default()),
                Span::styled(
                    pad(&score.wpm.to_string(), 6),
                    Style::default().fg(wpm_color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" ", Style::default()),
                Span::styled(
                    pad(&score.raw_wpm.to_string(), 6),
                    Style::default().fg(text_color),
                ),
                Span::styled(" ", Style::default()),
                Span::styled(
                    pad(&format!("{:.1}%", score.accuracy), 7),
                    Style::default().fg(text_color),
                ),
                Span::styled(" ", Style::default()),
                Span::styled(
                    pad(&score.language.replace('_', " "), 12),
                    Style::default().fg(text_color),
                ),
                Span::styled(" ", Style::default()),
                Span::styled(
                    pad(&mode_str, 10),
                    Style::default().fg(text_color),
                ),
                Span::styled(" ", Style::default()),
                Span::styled(date_str, Style::default().fg(dim)),
            ]);
            frame.render_widget(row, Rect::new(hx, y, table_area.width, 1));
            y += 1;
        }

        // Scroll indicator
        if scores.len() > visible_rows && y < table_area.y + table_area.height {
            let indicator = format!(
                "showing {}-{} of {} — arrows to scroll",
                scroll + 1,
                (scroll + visible_rows).min(scores.len()),
                scores.len()
            );
            let line = Line::from(Span::styled(indicator, Style::default().fg(dim)));
            let iw = line.width() as u16;
            let ix = table_area.x + table_area.width.saturating_sub(iw) / 2;
            frame.render_widget(
                line,
                Rect::new(ix, y, table_area.width, 1),
            );
        }
    }

    // Footer
    let footer = Line::from(Span::styled(
        "esc: back to menu",
        Style::default().fg(dim),
    ));
    let fw = footer.width() as u16;
    let fx = chunks[2].x + chunks[2].width.saturating_sub(fw) / 2;
    frame.render_widget(
        footer,
        Rect::new(fx, chunks[2].y, chunks[2].width, 1),
    );
}

fn pad(s: &str, len: usize) -> String {
    format!("{:<width$}", s, width = len)
}

fn format_date(iso: &str) -> String {
    chrono::DateTime::parse_from_rfc3339(iso)
        .map(|dt| dt.format("%b %d, %I:%M %p").to_string())
        .unwrap_or_else(|_| iso.to_string())
}

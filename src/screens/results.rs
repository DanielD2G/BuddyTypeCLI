use crate::app::App;
use crate::data::themes::parse_hex;
use crate::types::ThemeColors;
use crate::ui::results_chart::{ResultsChart, CHART_HEIGHT};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::Frame;

pub fn render(frame: &mut Frame, area: Rect, app: &App, theme: &ThemeColors) {
    let result = match &app.result {
        Some(r) => r,
        None => return,
    };
    let accent = parse_hex(&theme.accent);
    let text_color = parse_hex(&theme.text);
    let dim = parse_hex(&theme.text_dim);
    let stats_color = parse_hex(&theme.stats);
    let correct_color = parse_hex(&theme.correct);
    let incorrect_color = parse_hex(&theme.incorrect);
    let extra_color = parse_hex(&theme.extra);

    let chunks = Layout::vertical([
        Constraint::Length(2), // main WPM
        Constraint::Length(3), // stats grid
        Constraint::Length(2), // char breakdown
        Constraint::Length(2), // words count
        Constraint::Length(CHART_HEIGHT), // chart
        Constraint::Length(2), // footer
    ])
    .split(area);

    // Main WPM
    {
        let line = Line::from(vec![
            Span::styled(
                format!("{}", result.wpm),
                Style::default()
                    .fg(accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" wpm", Style::default().fg(stats_color)),
        ]);
        let w = line.width() as u16;
        let x = chunks[0].x + chunks[0].width.saturating_sub(w) / 2;
        frame.render_widget(line, Rect::new(x, chunks[0].y, chunks[0].width, 1));
    }

    // Stats grid: raw, accuracy, consistency, time
    {
        let spans = vec![
            Span::styled(
                format!("{}", result.raw_wpm),
                Style::default().fg(text_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  raw    ", Style::default().fg(dim)),
            Span::styled(
                format!("{:.1}%", result.accuracy),
                Style::default().fg(text_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  accuracy    ", Style::default().fg(dim)),
            Span::styled(
                format!("{:.1}%", result.consistency),
                Style::default().fg(text_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  consistency    ", Style::default().fg(dim)),
            Span::styled(
                format!("{:.1}s", result.elapsed_seconds),
                Style::default().fg(text_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  time", Style::default().fg(dim)),
        ];
        let line = Line::from(spans);
        let w = line.width() as u16;
        let x = chunks[1].x + chunks[1].width.saturating_sub(w) / 2;
        frame.render_widget(
            line,
            Rect::new(x, chunks[1].y + 1, chunks[1].width, 1),
        );
    }

    // Character breakdown
    {
        let spans = vec![
            Span::styled(
                format!("{}", result.correct_chars),
                Style::default().fg(correct_color),
            ),
            Span::styled(" correct   ", Style::default().fg(dim)),
            Span::styled(
                format!("{}", result.incorrect_chars),
                Style::default().fg(incorrect_color),
            ),
            Span::styled(" incorrect   ", Style::default().fg(dim)),
            Span::styled(
                format!("{}", result.extra_chars),
                Style::default().fg(extra_color),
            ),
            Span::styled(" extra   ", Style::default().fg(dim)),
            Span::styled(
                format!("{}", result.missed_chars),
                Style::default().fg(stats_color),
            ),
            Span::styled(" missed", Style::default().fg(dim)),
        ];
        let line = Line::from(spans);
        let w = line.width() as u16;
        let x = chunks[2].x + chunks[2].width.saturating_sub(w) / 2;
        frame.render_widget(line, Rect::new(x, chunks[2].y, chunks[2].width, 1));
    }

    // Words count
    {
        let line = Line::from(vec![
            Span::styled(
                format!("{}/{}", result.correct_words, result.total_words),
                Style::default().fg(text_color),
            ),
            Span::styled(" words correct", Style::default().fg(dim)),
        ]);
        let w = line.width() as u16;
        let x = chunks[3].x + chunks[3].width.saturating_sub(w) / 2;
        frame.render_widget(line, Rect::new(x, chunks[3].y, chunks[3].width, 1));
    }

    // WPM Chart
    frame.render_widget(
        ResultsChart {
            wpm_history: &app.result_wpm_history,
            theme,
            terminal_width: app.terminal_width,
            height: 8,
        },
        chunks[4],
    );

    // Footer
    {
        let footer = Line::from(Span::styled(
            "tab: restart  esc: menu",
            Style::default().fg(dim),
        ));
        let w = footer.width() as u16;
        let x = chunks[5].x + chunks[5].width.saturating_sub(w) / 2;
        frame.render_widget(
            footer,
            Rect::new(x, chunks[5].y, chunks[5].width, 1),
        );
    }
}

use crate::app::App;
use crate::data::themes::parse_hex;
use crate::engine::timer::{get_elapsed_seconds, get_remaining_seconds};
use crate::types::ThemeColors;
use crate::ui::stats_bar::StatsBar;
use crate::ui::word_display::WordDisplay;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::Frame;

pub fn render(frame: &mut Frame, area: Rect, app: &App, theme: &ThemeColors) {
    let dim = parse_hex(&theme.text_dim);

    let chunks = Layout::vertical([
        Constraint::Length(2), // stats bar
        Constraint::Min(3),   // word display
        Constraint::Length(2), // footer
    ])
    .split(area);

    // Stats bar
    frame.render_widget(
        StatsBar {
            wpm: app.current_stats.wpm,
            accuracy: app.current_stats.accuracy,
            elapsed_seconds: get_elapsed_seconds(&app.timer),
            remaining_seconds: get_remaining_seconds(&app.timer),
            phase: app.phase,
            mode: app.config.mode,
            theme,
        },
        chunks[0],
    );

    // Word display
    if let Some(ref input_state) = app.input_state {
        frame.render_widget(
            WordDisplay {
                words: &input_state.words,
                current_word_index: input_state.current_word_index,
                cursor_position: input_state.cursor_position,
                theme,
                one_line: app.config.one_line,
            },
            chunks[1],
        );
    }

    // Footer
    let footer_text = if app.restart_pending {
        "enter: confirm restart  any key: cancel  esc: menu"
    } else {
        "tab: restart  esc: menu"
    };
    let footer = Line::from(Span::styled(footer_text, Style::default().fg(dim)));
    let fw = footer.width() as u16;
    let fx = chunks[2].x + chunks[2].width.saturating_sub(fw) / 2;
    frame.render_widget(
        footer,
        Rect::new(fx, chunks[2].y, chunks[2].width, 1),
    );
}

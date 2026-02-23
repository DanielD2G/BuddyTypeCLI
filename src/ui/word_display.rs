use crate::data::themes::parse_hex;
use crate::types::{ThemeColors, WordState};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Widget;

pub struct WordDisplay<'a> {
    pub words: &'a [WordState],
    pub current_word_index: usize,
    pub cursor_position: usize,
    pub terminal_width: u16,
    pub theme: &'a ThemeColors,
    pub one_line: bool,
}

struct LineRange {
    start_index: usize,
    end_index: usize,
}

fn compute_lines(words: &[WordState], max_width: usize) -> Vec<LineRange> {
    let mut lines = Vec::new();
    let mut line_start = 0;
    let mut line_width: usize = 0;

    for i in 0..words.len() {
        let word_len = words[i].word.len() + 1; // +1 for space

        if line_width + word_len > max_width && line_width > 0 {
            lines.push(LineRange {
                start_index: line_start,
                end_index: i,
            });
            line_start = i;
            line_width = 0;
        }
        line_width += word_len;
    }

    if line_start < words.len() {
        lines.push(LineRange {
            start_index: line_start,
            end_index: words.len(),
        });
    }

    lines
}

struct TapeRange {
    start_index: usize,
    end_index: usize,
    leading_pad: usize,
}

fn compute_tape_range(
    words: &[WordState],
    current_word_index: usize,
    cursor_position: usize,
    max_width: usize,
) -> TapeRange {
    let anchor = (max_width as f64 * 0.50).floor() as usize;

    let current_word_len = words
        .get(current_word_index)
        .map(|w| w.word.len())
        .unwrap_or(0);
    let typed_len = words
        .get(current_word_index)
        .map(|w| w.typed.len())
        .unwrap_or(0);
    let display_len = current_word_len.max(typed_len);
    let cursor_col = cursor_position.min(display_len);

    // Fill LEFT of anchor with past words
    let mut left_budget = anchor.saturating_sub(cursor_col);
    let mut start_index = current_word_index;

    for i in (0..current_word_index).rev() {
        if left_budget == 0 {
            break;
        }
        let w = &words[i];
        let w_len = w.word.len().max(w.typed.len()) + 1;
        if w_len > left_budget {
            break;
        }
        left_budget -= w_len;
        start_index = i;
    }

    let leading_pad = left_budget;

    // Fill RIGHT of anchor with upcoming words
    let right_anchor = max_width - anchor + cursor_col;
    let mut right_budget = right_anchor.saturating_sub(display_len + 1);

    let mut end_index = current_word_index + 1;
    for i in (current_word_index + 1)..words.len() {
        if right_budget == 0 {
            break;
        }
        let w_len = words[i].word.len() + 1;
        if w_len > right_budget {
            break;
        }
        right_budget -= w_len;
        end_index = i + 1;
    }

    TapeRange {
        start_index,
        end_index,
        leading_pad,
    }
}

impl Widget for WordDisplay<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width < 4 {
            return;
        }

        let max_width = (area.width.saturating_sub(4) as usize).min(120);

        let correct_color = parse_hex(&self.theme.correct);
        let incorrect_color = parse_hex(&self.theme.incorrect);
        let extra_color = parse_hex(&self.theme.extra);
        let dim_color = parse_hex(&self.theme.text_dim);
        let cursor_color = parse_hex(&self.theme.cursor);

        let correct_style = Style::default().fg(correct_color);
        let incorrect_style = Style::default().fg(incorrect_color);
        let extra_style = Style::default().fg(extra_color);
        let dim_style = Style::default().fg(dim_color);
        let cursor_style = Style::default().fg(cursor_color).add_modifier(Modifier::REVERSED);
        let extra_cursor_style = Style::default().fg(extra_color).add_modifier(Modifier::REVERSED);

        let padding_x = 2u16;

        if self.one_line {
            let tape = compute_tape_range(
                self.words,
                self.current_word_index,
                self.cursor_position,
                max_width,
            );
            let y = area.y;
            let mut x = area.x + padding_x + tape.leading_pad as u16;

            for word_idx in tape.start_index..tape.end_index {
                if x >= area.x + area.width {
                    break;
                }
                let is_current = word_idx == self.current_word_index;
                x = render_word(
                    buf,
                    x,
                    y,
                    area.x + area.width,
                    &self.words[word_idx],
                    is_current,
                    self.cursor_position,
                    correct_style,
                    incorrect_style,
                    extra_style,
                    dim_style,
                    cursor_style,
                    extra_cursor_style,
                );
            }
            return;
        }

        let lines = compute_lines(self.words, max_width);

        // Find which line the current word is on
        let mut current_line = 0;
        for (i, line) in lines.iter().enumerate() {
            if self.current_word_index >= line.start_index
                && self.current_word_index < line.end_index
            {
                current_line = i;
                break;
            }
        }

        let visible_line_count = 3usize;
        let start_line = current_line.saturating_sub(1);

        for row in 0..visible_line_count {
            let line_idx = start_line + row;
            let y = area.y + row as u16;
            if y >= area.y + area.height {
                break;
            }

            if line_idx < lines.len() {
                let line = &lines[line_idx];
                let mut x = area.x + padding_x;

                for word_idx in line.start_index..line.end_index {
                    if x >= area.x + area.width {
                        break;
                    }
                    let is_current = word_idx == self.current_word_index;
                    x = render_word(
                        buf,
                        x,
                        y,
                        area.x + area.width,
                        &self.words[word_idx],
                        is_current,
                        self.cursor_position,
                        correct_style,
                        incorrect_style,
                        extra_style,
                        dim_style,
                        cursor_style,
                        extra_cursor_style,
                    );
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render_word(
    buf: &mut Buffer,
    mut x: u16,
    y: u16,
    max_x: u16,
    word_state: &WordState,
    is_current: bool,
    cursor_pos: usize,
    correct_style: Style,
    incorrect_style: Style,
    extra_style: Style,
    dim_style: Style,
    cursor_style: Style,
    extra_cursor_style: Style,
) -> u16 {
    let word_chars: Vec<char> = word_state.word.chars().collect();
    let typed_chars: Vec<char> = word_state.typed.chars().collect();

    // Render target characters
    for i in 0..word_chars.len() {
        if x >= max_x {
            return x;
        }
        let is_cursor = is_current && i == cursor_pos;
        let ch = word_chars[i];

        let style = if is_cursor {
            cursor_style
        } else if i < typed_chars.len() {
            if typed_chars[i] == ch {
                correct_style
            } else {
                incorrect_style
            }
        } else {
            dim_style
        };

        buf.set_string(x, y, ch.to_string(), style);
        x += 1;
    }

    // Render extra characters (typed beyond word length)
    for i in word_chars.len()..typed_chars.len() {
        if x >= max_x {
            return x;
        }
        let is_cursor = is_current && i == cursor_pos;
        let style = if is_cursor {
            extra_cursor_style
        } else {
            extra_style
        };
        buf.set_string(x, y, typed_chars[i].to_string(), style);
        x += 1;
    }

    // Trailing space / cursor-at-end
    if x < max_x {
        let cursor_at_end =
            is_current && cursor_pos >= word_chars.len().max(typed_chars.len());
        let style = if cursor_at_end {
            cursor_style
        } else {
            Style::default()
        };
        buf.set_string(x, y, " ", style);
        x += 1;
    }

    x
}

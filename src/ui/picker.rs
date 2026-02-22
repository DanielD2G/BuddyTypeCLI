use crate::data::themes::parse_hex;
use crate::types::ThemeColors;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Widget};

const OVERHEAD_ROWS: u16 = 6; // search(2) + scroll indicators(2) + blank+footer(2)

pub struct PickerState {
    pub items: Vec<String>,
    pub selected: String,
    pub search: String,
    pub cursor: usize,
    pub filtered: Vec<usize>, // indices into items
}

pub enum PickerResult {
    /// User selected an item
    Selected(String),
    /// User cancelled
    Cancelled,
    /// Highlighted item changed (for theme preview)
    Highlighted(String),
    /// Still active, consumed the key
    Active,
}

impl PickerState {
    pub fn new(items: Vec<String>, selected: String) -> Self {
        let filtered: Vec<usize> = (0..items.len()).collect();
        let cursor = items
            .iter()
            .position(|i| *i == selected)
            .unwrap_or(0);
        Self {
            items,
            selected,
            search: String::new(),
            cursor,
            filtered,
        }
    }

    fn refilter(&mut self) {
        if self.search.is_empty() {
            self.filtered = (0..self.items.len()).collect();
        } else {
            let q = self.search.to_lowercase();
            self.filtered = self
                .items
                .iter()
                .enumerate()
                .filter(|(_, item)| item.to_lowercase().contains(&q))
                .map(|(i, _)| i)
                .collect();
        }
        self.cursor = self.cursor.min(self.filtered.len().saturating_sub(1));
    }

    pub fn current_highlighted(&self) -> Option<&str> {
        self.filtered
            .get(self.cursor)
            .map(|&i| self.items[i].as_str())
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> PickerResult {
        match key.code {
            KeyCode::Esc => return PickerResult::Cancelled,
            KeyCode::Enter => {
                if let Some(&idx) = self.filtered.get(self.cursor) {
                    return PickerResult::Selected(self.items[idx].clone());
                }
                return PickerResult::Cancelled;
            }
            KeyCode::Up => {
                self.cursor = self.cursor.saturating_sub(1);
                if let Some(h) = self.current_highlighted() {
                    return PickerResult::Highlighted(h.to_string());
                }
            }
            KeyCode::Down => {
                if !self.filtered.is_empty() {
                    self.cursor = (self.cursor + 1).min(self.filtered.len() - 1);
                }
                if let Some(h) = self.current_highlighted() {
                    return PickerResult::Highlighted(h.to_string());
                }
            }
            KeyCode::Backspace => {
                self.search.pop();
                self.cursor = 0;
                self.refilter();
                if let Some(h) = self.current_highlighted() {
                    return PickerResult::Highlighted(h.to_string());
                }
            }
            KeyCode::Char(c) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.search.push(c);
                    self.cursor = 0;
                    self.refilter();
                    if let Some(h) = self.current_highlighted() {
                        return PickerResult::Highlighted(h.to_string());
                    }
                }
            }
            _ => {}
        }
        PickerResult::Active
    }
}

pub struct Picker<'a> {
    pub state: &'a PickerState,
    pub theme: &'a ThemeColors,
}

impl Widget for Picker<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let accent = parse_hex(&self.theme.accent);
        let text_color = parse_hex(&self.theme.text);
        let dim = parse_hex(&self.theme.text_dim);
        let correct = parse_hex(&self.theme.correct);
        let incorrect = parse_hex(&self.theme.incorrect);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(accent))
            .title("");
        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height == 0 || inner.width == 0 {
            return;
        }

        let mut y = inner.y;

        // Search bar
        let search_line = Line::from(vec![
            Span::styled("search: ", Style::default().fg(dim)),
            Span::styled(&self.state.search, Style::default().fg(text_color)),
            Span::styled("_", Style::default().fg(accent)),
        ]);
        buf.set_line(inner.x + 1, y, &search_line, inner.width);
        y += 2;

        let safe_cursor = self
            .state
            .cursor
            .min(self.state.filtered.len().saturating_sub(1));

        // Calculate visible rows from actual available area
        let visible_rows = inner.height.saturating_sub(OVERHEAD_ROWS) as usize;
        let visible_rows = visible_rows.max(1);

        // Scroll window
        let half_window = visible_rows / 2;
        let mut scroll_start = safe_cursor.saturating_sub(half_window);
        let scroll_end = (scroll_start + visible_rows).min(self.state.filtered.len());
        if scroll_end - scroll_start < visible_rows {
            scroll_start = scroll_end.saturating_sub(visible_rows);
        }
        let visible = &self.state.filtered[scroll_start..scroll_end];
        let has_scroll = self.state.filtered.len() > visible_rows;
        let at_top = scroll_start == 0;
        let at_bottom = scroll_end >= self.state.filtered.len();

        // Scroll indicator top
        if has_scroll && y < inner.y + inner.height {
            let indicator = if at_top { "         " } else { "  ▲ more " };
            let line = Line::from(Span::styled(indicator, Style::default().fg(dim)));
            buf.set_line(inner.x + 1, y, &line, inner.width);
            y += 1;
        }

        // Items
        for (vi, &global_idx) in visible.iter().enumerate() {
            if y >= inner.y + inner.height {
                break;
            }
            let real_cursor = scroll_start + vi;
            let is_cursor = real_cursor == safe_cursor;
            let item = &self.state.items[global_idx];
            let is_selected = *item == self.state.selected;
            let display = item.replace('_', " ");

            let prefix = if is_cursor { " > " } else { "   " };
            let prefix_style = Style::default().fg(if is_cursor { accent } else { dim });

            let item_style = if is_cursor {
                Style::default()
                    .fg(accent)
                    .add_modifier(Modifier::BOLD)
            } else if is_selected {
                Style::default().fg(correct)
            } else {
                Style::default().fg(text_color)
            };

            let mut spans = vec![
                Span::styled(prefix, prefix_style),
                Span::styled(display, item_style),
            ];

            if is_selected {
                spans.push(Span::styled(" ●", Style::default().fg(correct)));
            }

            let line = Line::from(spans);
            buf.set_line(inner.x, y, &line, inner.width);
            y += 1;
        }

        // No matches
        if self.state.filtered.is_empty() && y < inner.y + inner.height {
            let line = Line::from(Span::styled(
                "  no matches",
                Style::default().fg(incorrect),
            ));
            buf.set_line(inner.x, y, &line, inner.width);
            y += 1;
        }

        // Scroll indicator bottom
        if has_scroll && y < inner.y + inner.height {
            let indicator = if at_bottom {
                "         "
            } else {
                "  ▼ more "
            };
            let line = Line::from(Span::styled(indicator, Style::default().fg(dim)));
            buf.set_line(inner.x + 1, y, &line, inner.width);
            y += 1;
        }

        // Footer
        if y + 1 < inner.y + inner.height {
            y += 1;
            let footer = Line::from(Span::styled(
                "type to search  enter: select  esc: cancel",
                Style::default().fg(dim),
            ));
            buf.set_line(inner.x + 1, y, &footer, inner.width);
        }
    }
}

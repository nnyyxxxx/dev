use crate::{float::FloatContent, hint::Shortcut, theme::Theme};
use clap::ValueEnum;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};

pub struct ThemeSelector {
    themes: Vec<Theme>,
    state: ListState,
    pub selected_theme: Theme,
    initial_theme: Theme,
}

impl ThemeSelector {
    pub fn new(current_theme: Theme) -> Self {
        let themes = Theme::value_variants();
        let mut state = ListState::default();
        state.select(Some(
            themes.iter().position(|&t| t == current_theme).unwrap_or(0),
        ));
        Self {
            themes: themes.to_vec(),
            state,
            selected_theme: current_theme,
            initial_theme: current_theme,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.themes.len(),
            None => 0,
        };
        self.state.select(Some(i));
        self.selected_theme = self.themes[i];
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + self.themes.len() - 1) % self.themes.len(),
            None => 0,
        };
        self.state.select(Some(i));
        self.selected_theme = self.themes[i];
    }
}

impl FloatContent for ThemeSelector {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Theme Selector")
            .title_alignment(ratatui::layout::Alignment::Center);

        frame.render_widget(block.clone(), area);
        let inner_area = block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(inner_area);

        let preview = Paragraph::new(vec![
            Line::from(vec![
                Span::styled(
                    "Directory",
                    Style::default().fg(self.selected_theme.dir_color()),
                ),
                Span::raw(" "),
                Span::styled(
                    "Command",
                    Style::default().fg(self.selected_theme.cmd_color()),
                ),
                Span::raw(" "),
                Span::styled("Tab", Style::default().fg(self.selected_theme.tab_color())),
            ]),
            Line::from(vec![
                Span::styled(
                    "Success",
                    Style::default().fg(self.selected_theme.success_color()),
                ),
                Span::raw(" "),
                Span::styled(
                    "Fail",
                    Style::default().fg(self.selected_theme.fail_color()),
                ),
                Span::raw(" "),
                Span::styled(
                    "Focused",
                    Style::default().fg(self.selected_theme.focused_color()),
                ),
            ]),
        ])
        .block(Block::default().borders(Borders::ALL).title("Preview"));

        frame.render_widget(preview, chunks[0]);

        let items: Vec<Line> = self
            .themes
            .iter()
            .map(|t| Line::from(format!("{:?}", t)))
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Themes"))
            .highlight_style(
                Style::default()
                    .bg(self.selected_theme.focused_color())
                    .fg(Color::Black)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            );

        frame.render_stateful_widget(list, chunks[1], &mut self.state);
    }

    fn handle_key_event(&mut self, key: &KeyEvent) -> bool {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => self.next(),
            KeyCode::Up | KeyCode::Char('k') => self.previous(),
            KeyCode::Enter => {
                return true;
            }
            KeyCode::Esc => {
                self.selected_theme = self.initial_theme;
                return true;
            }
            _ => {}
        }
        false
    }

    fn is_finished(&self) -> bool {
        false
    }

    fn get_shortcut_list(&self) -> (&str, Box<[Shortcut]>) {
        (
            "Theme Selector",
            Box::new([
                Shortcut::new("Next theme", ["j", "Down"]),
                Shortcut::new("Previous theme", ["k", "Up"]),
                Shortcut::new("Select theme", ["Enter"]),
                Shortcut::new("Cancel", ["Esc"]),
            ]),
        )
    }
}

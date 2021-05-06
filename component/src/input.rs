use std::time::{Duration, Instant};

use tui::{
    backend::Backend,
    layout::Rect,
    style::*,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use tui_wrapper::widget::*;

#[derive(Debug)]
enum Mode {
    Show,
    Hide,
}

impl Mode {
    fn toggle(&mut self) {
        match self {
            Mode::Show => *self = Mode::Hide,
            Mode::Hide => *self = Mode::Show,
        }
    }
}

#[derive(Debug)]
struct Cursor {
    cursor: char,
    pos: usize,
    last_tick: Instant,
    tick_rate: Duration,
    mode: Mode,
}

impl Cursor {
    fn forward(&mut self) {
        self.pos = self.pos.saturating_add(1);
        self.last_tick = Instant::now();
        self.mode = Mode::Show;
    }

    fn back(&mut self) {
        self.pos = self.pos.saturating_sub(1);
        self.last_tick = Instant::now();
        self.mode = Mode::Show;
    }

    fn update_tick(&mut self) {
        if self.tick_rate <= self.last_tick.elapsed() {
            self.mode.toggle();
            self.last_tick = Instant::now();
        }
    }

    fn cursor_style(&self) -> Style {
        match self.mode {
            Mode::Show => Style::default().add_modifier(Modifier::REVERSED),
            Mode::Hide => Style::default(),
        }
    }

    fn pos(&self) -> usize {
        self.pos
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            cursor: ' ',
            pos: 0,
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(500),
            mode: Mode::Show,
        }
    }
}

#[derive(Debug)]
pub struct InputForm<'a> {
    content: String,
    cursor: Cursor,
    widget: Widget<'a>,
    width: usize,
    chunk: Rect,
}

impl Default for InputForm<'_> {
    fn default() -> Self {
        Self {
            content: String::default(),
            cursor: Cursor::default(),
            widget: Widget::Text(Text::default()),
            width: 1,
            chunk: Rect::default(),
        }
    }
}

impl<'a> InputForm<'a> {
    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        self.cursor.update_tick();

        let spans = Self::render_content(self.content.as_str(), &self.cursor);

        let widget = Paragraph::new(spans).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(Span::styled("Filter", Style::reset()))
                .title_offset(1),
        );

        f.render_widget(widget, self.chunk);
    }

    fn render_content(content: &str, cursor: &Cursor) -> Spans<'a> {
        match (content.len(), cursor.pos()) {
            (0, _) => Spans::from(Span::styled(" ", cursor.cursor_style())),
            (len, pos) if pos < len => Spans::from(
                content
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i == pos {
                            Span::styled(c.to_string(), cursor.cursor_style())
                        } else {
                            Span::raw(c.to_string())
                        }
                    })
                    .collect::<Vec<Span>>(),
            ),
            _ => Spans::from(vec![
                Span::raw(content.to_string()),
                Span::styled(" ", cursor.cursor_style()),
            ]),
        }
    }

    pub fn update_chunk(&mut self, chunk: Rect) {
        self.chunk = chunk;
    }

    pub fn insert_char(&mut self, c: char) {
        self.content.insert(self.cursor.pos(), c);
        self.cursor.forward();
    }

    pub fn remove_char(&mut self) {
        if !self.content.is_empty() && 0 < self.cursor.pos() {
            self.cursor.back();
            self.content.remove(self.cursor.pos());
        }
    }

    pub fn forward_cursor(&mut self) {
        if self.cursor.pos() < self.content.len() {
            self.cursor.forward()
        }
    }
    pub fn back_cursor(&mut self) {
        self.cursor.back();
    }

    pub fn content(&self) -> &str {
        self.content.as_str()
    }

    pub fn clear_content(&mut self) {
        self.content.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod cursor {
        use super::*;
        use pretty_assertions::assert_eq;
        #[test]
        fn move_forward() {
            let mut cursor = Cursor::default();
            cursor.forward();

            assert_eq!(cursor.pos(), 1);
        }

        #[test]
        fn move_back() {
            let mut cursor = Cursor::default();
            cursor.forward();
            cursor.forward();
            cursor.back();

            assert_eq!(cursor.pos(), 1);
        }
    }

    mod input_form {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn push_char() {
            let mut form = InputForm::default();

            let input = "test";

            input.chars().for_each(|c| form.insert_char(c));

            assert_eq!(input, form.content);
        }

        #[test]
        fn insert_char() {
            let mut form = InputForm::default();

            let input = "test";

            input.chars().for_each(|c| form.insert_char(c));

            form.back_cursor();

            form.insert_char('a');

            assert_eq!("tesat", form.content);

            form.forward_cursor();
            form.forward_cursor();

            form.insert_char('b');
            assert_eq!("tesatb", form.content);
        }

        #[test]
        fn render_content_empty() {
            let form = InputForm::default();

            assert_eq!(
                InputForm::render_content(form.content.as_str(), &form.cursor),
                Spans::from(Span::styled(
                    " ",
                    Style::default().add_modifier(Modifier::REVERSED)
                ))
            );
        }

        #[test]
        fn render_content_add_char() {
            let mut form = InputForm::default();

            form.insert_char('a');
            form.insert_char('b');

            assert_eq!(
                InputForm::render_content(form.content.as_str(), &form.cursor),
                Spans::from(vec![
                    Span::raw("ab"),
                    Span::styled(" ", Style::default().add_modifier(Modifier::REVERSED))
                ])
            );
        }

        #[test]
        fn render_content_add_char_and_cursor_back() {
            let mut form = InputForm::default();

            form.insert_char('a');
            form.insert_char('b');
            form.back_cursor();

            assert_eq!(
                InputForm::render_content(form.content.as_str(), &form.cursor),
                Spans::from(vec![
                    Span::raw("a"),
                    Span::styled("b", Style::default().add_modifier(Modifier::REVERSED))
                ])
            );
        }
    }
}

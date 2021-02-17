use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::ListState;

use std::cell::RefCell;

pub struct Window {
    tabs: Vec<Tab>,
    selected_tab_index: usize,
    layout: Layout,
}

pub struct Tab {
    title: String,
    panes: Vec<Pane>,
    layout: Layout,
    selected_pane_index: usize,
    selectable_widgets: Vec<usize>,
}

pub struct Pane {
    widget: Widget,
    chunk_index: usize,
    title: String,
}

pub enum Widget {
    List(List),
}

pub struct List {
    items: Vec<String>,
    state: RefCell<ListState>,
}

impl Window {
    pub fn new(tabs: Vec<Tab>) -> Self {
        let mut w = Window::default();
        w.tabs = tabs;
        w
    }

    pub fn chunks(&self, window_size: Rect) -> Vec<Rect> {
        self.layout.split(window_size)
    }

    pub fn tabs(&self) -> &Vec<Tab> {
        &self.tabs
    }

    pub fn selected_tab_index(&self) -> usize {
        self.selected_tab_index
    }

    pub fn select_next_tab(&mut self) {
        if self.tabs.len() - 1 <= self.selected_tab_index {
            self.selected_tab_index = 0;
        } else {
            self.selected_tab_index += 1;
        }
    }

    pub fn selected_tab(&self) -> &Tab {
        &self.tabs[self.selected_tab_index]
    }

    pub fn select_next_pane(&mut self) {
        self.tabs[self.selected_tab_index].next_pane();
    }

    pub fn select_next_item(&mut self) {
        self.tabs[self.selected_tab_index]
            .selected_pane()
            .next_item();
    }

    pub fn select_prev_item(&mut self) {
        self.tabs[self.selected_tab_index]
            .selected_pane()
            .prev_item();
    }
}

impl Default for Window {
    fn default() -> Self {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref());

        Self {
            tabs: Vec::new(),
            selected_tab_index: 0,
            layout,
        }
    }
}

impl Tab {
    pub fn new(title: String, panes: Vec<Pane>, layout: Layout) -> Self {
        let selectable_widgets = panes
            .iter()
            .enumerate()
            .filter(|&(_, p)| match p.widget {
                Widget::List(_) => true,
            })
            .map(|(i, _)| i)
            .collect();

        Self {
            title,
            panes,
            layout,
            selected_pane_index: 0,
            selectable_widgets,
        }
    }
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn chunks(&self, tab_size: Rect) -> Vec<Rect> {
        self.layout.split(tab_size)
    }

    pub fn panes(&self) -> &Vec<Pane> {
        &self.panes
    }

    pub fn next_pane(&mut self) {
        if self.selectable_widgets.len() - 1 <= self.selected_pane_index {
            self.selected_pane_index = 0;
        } else {
            self.selected_pane_index += 1;
        }
    }

    pub fn selected_pane(&mut self) -> &mut Pane {
        &mut self.panes[self.selected_pane_index]
    }
}

impl Pane {
    pub fn new(title: String, widget: Widget, chunk_index: usize) -> Self {
        Self {
            title,
            widget,
            chunk_index,
        }
    }

    pub fn widget(&self) -> &Widget {
        &self.widget
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn chunk_index(&self) -> usize {
        self.chunk_index
    }

    pub fn next_item(&mut self) {
        match &self.widget {
            Widget::List(l) => l.next(),
        };
    }

    pub fn prev_item(&mut self) {
        match &self.widget {
            Widget::List(l) => l.prev(),
        };
    }
}

impl List {
    pub fn new(items: Vec<String>) -> Self {
        let mut state = ListState::default();
        if 0 < items.len() {
            state.select(Some(0));
        }

        Self {
            items,
            state: RefCell::new(state),
        }
    }
    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
    }

    pub fn next(&self) {
        let i = match self.state.borrow().selected() {
            Some(i) => {
                if self.items.len() - 1 <= i {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.state.borrow_mut().select(Some(i));
    }

    pub fn prev(&self) {
        let i = match self.state.borrow().selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.borrow_mut().select(Some(i));
    }

    pub fn unselect(&self) {
        self.state.borrow_mut().select(None);
    }

    pub fn state(&self) -> &RefCell<ListState> {
        &self.state
    }

    pub fn items(&self) -> &Vec<String> {
        &self.items
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.borrow().selected()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_prev_is_selected_first_index() {
        let list = List::new(vec![
            String::from("Item 0"),
            String::from("Item 1"),
            String::from("Item 2"),
        ]);

        list.prev();
        assert_eq!(Some(0), list.selected())
    }

    #[test]
    fn two_prev_is_selected_last_index() {
        let list = List::new(vec![
            String::from("Item 0"),
            String::from("Item 1"),
            String::from("Item 2"),
        ]);

        list.prev();
        list.prev();
        assert_eq!(Some(2), list.selected())
    }
    #[test]
    fn one_next_is_selected_first_index() {
        let list = List::new(vec![
            String::from("Item 0"),
            String::from("Item 1"),
            String::from("Item 2"),
        ]);

        list.next();
        assert_eq!(Some(0), list.selected())
    }

    #[test]
    fn last_next_is_selected_first_index() {
        let list = List::new(vec![
            String::from("Item 0"),
            String::from("Item 1"),
            String::from("Item 2"),
        ]);

        list.next();
        list.next();
        list.next();
        list.next();
        assert_eq!(Some(0), list.selected())
    }
}

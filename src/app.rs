use crate::days::{init_days, Day};

use std::process::Command;
use tui::widgets::TableState;

pub struct StatefulTable<T> {
    pub state: TableState,
    pub items: Vec<T>,
}

impl<T> StatefulTable<T> {
    pub fn with_rows(items: Vec<T>) -> StatefulTable<T> {
        StatefulTable {
            state: TableState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn current_item(&self) -> &T {
        let i = self.state.selected().unwrap_or(0);
        &self.items[i]
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub day_table: StatefulTable<Day>,
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            day_table: StatefulTable::with_rows(init_days()),
            enhanced_graphics,
        }
    }

    pub fn on_up(&mut self) {
        self.day_table.previous();
    }

    pub fn on_down(&mut self) {
        self.day_table.next();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }

            'o' => {
                let item = self.day_table.current_item();

                Command::new("open")
                    .arg(item.url())
                    .output()
                    .expect("FAILED TO OPEN");
            }

            's' => {
                let item = self.day_table.current_item();
                item.part_1.solve();
                item.part_2.solve();
            }

            'S' => {
                for item in &self.day_table.items {
                    item.part_1.solve();
                    item.part_2.solve();
                }
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {}
}

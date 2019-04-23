extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;

pub struct MainWindow
{
    max_x: i32,
    max_y: i32,
    input_seq: String,
    win: WINDOW
}

impl MainWindow
{
    pub fn new() -> Self
    {
        let mut max_x: i32 = 0;
        let mut max_y: i32 = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        let mut main_window = MainWindow {
            max_x,
            max_y,
            input_seq: String::new(),
            win: newwin(max_y - 1, max_x, 1, 0) // max_y - 1 to account for top bar
        };

        scrollok(main_window.win, true);
        wmove(main_window.win, 0, 0);

        main_window
    }

    pub fn prompt(&mut self)
    {
        wprintw(self.win, "\n>>");
        self.input_seq.clear();
    }

    pub fn get_win(&self) -> WINDOW { self.win }
    pub fn get_max_x(&self) -> i32 { self.max_x }
    pub fn get_max_y(&self) -> i32 { self.max_y }
}
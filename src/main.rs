#[allow(unused_imports)]

extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;

fn main() 
{
    initscr();
    raw();

    // allow for full keeb usage
    keypad(stdscr(), true);
    noecho();

    printw("Hello, World!");

    // refresh screen
    refresh();

    // wait
    getch();

    endwin();
}

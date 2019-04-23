#[allow(unused_imports)]

mod entities;
mod game_window;
mod interactables;

extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;
use entities::*;
use game_window::*;
use interactables::*;

fn init_curses()
{
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
}

fn main() 
{
    init_curses();

    let mut game: Game = Game::new();
    game.run();

    endwin();
}


/*
    useful for inf loops with curses & linux:
        ps aux | grep your_executable_name
        kill the_pid
*/
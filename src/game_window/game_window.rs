/*

extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;

// this will be the top bar on the game window
struct Header
{
    game_title: String,
    curr_room: String,
    player_level: i16,
    player_chealth: i16,
    player_mhealth: i16,
    bar: WINDOW,
    bar_max_x: i32,
    bar_max_y: i32
}

struct MainWindow
{
    max_x: i32,
    max_y: i32,
    escape_seq: String,
    win: WINDOW
}

impl Header
{
    pub fn new() -> Self
    {
        let mut bar_max_x: i32 = 0;
        let mut bar_max_y: i32 = 0;
        getmaxyx(stdscr(), &mut bar_max_y, &mut bar_max_x);

        Header {
            game_title: String::from("Rustales"),
            curr_room: String::from("Unknown room"),
            player_level: 1,
            player_chealth: 10,
            player_mhealth: 10,
            bar_max_x,
            bar_max_y,
            bar: newwin(1, bar_max_x, 0, 0),
        }
    }

    pub fn update()
    {

    }
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
            escape_seq: String::new(),
            win: newwin(max_y - 1, max_x, 1, 0) // max_y - 1 to account for top bar
        };

        scrollok(main_window.win, true);
        wmove(main_window.win, 0, 0);

        main_window
    }

    pub fn run()
    {

    }
}


*/
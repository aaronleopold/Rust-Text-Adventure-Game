extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;

pub struct Header
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
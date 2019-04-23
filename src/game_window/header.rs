use crate::game_window::*;

extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;

pub struct Header
{
    game_title: String,
    curr_room: String,
    player_level: u16,
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

    pub fn update(
        &mut self, new_room: String, new_level: u16, c_health: i16, 
        m_health: i16)
    {
        self.curr_room = new_room.clone();
        self.player_level = new_level;
        self.player_chealth = c_health;
        self.player_mhealth = m_health;

        let title = format! (" {} | {} ", self.game_title, self.curr_room);
        let whole_bar = format! (
            "{:1$} Health: {2:}/{3:} | Level: {4:}", title, (self.bar_max_x as usize - 
            (title.len())), self.player_chealth, self.player_mhealth,
            self.player_level);

        leaveok(self.bar, true);
        wmove(self.bar, 0, 0);
        wattron(self.bar, A_BOLD() | A_REVERSE());
        wprintw(self.bar, &whole_bar);
        wattroff(self.bar, A_BOLD() | A_REVERSE());
        wrefresh(self.bar);
    }
}
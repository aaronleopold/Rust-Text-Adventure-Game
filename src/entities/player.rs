extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;

use crate::interactables::*;
use crate::game_window::*;

pub struct Player
{
    //name: String,
    level: u16,
    max_health: i16,
    curr_health: i16,
    alive: bool,
    curr_location: u32
}

impl Player
{
    pub fn new() -> Self 
    {
        Player {
            //name: String::from(name),
            // default levels?
            level: 10,
            max_health: 10,
            curr_health: 10,
            alive: true,
            curr_location: 1000
        }
    }

    pub fn take_damage(&mut self, amount: i16) -> bool
    {
        // function will return false upon death
        self.curr_health -= amount;
        if self.curr_health <= 0 {
            self.alive = false;
        }

        self.alive // return syntax?
    }

    pub fn look_at_room(&mut self, win: &mut MainWindow)
    {
        wprintw(win.get_win(), "\n");
        let message = "Finally, you're awake. Do you know where you are?\n";
        wprintw(win.get_win(), &fill(message, win.get_max_x() as usize));
    }

    pub fn heal(&mut self, amount: i16) { self.curr_health += amount; }

    pub fn is_alive(&self) -> bool { self.alive }

    pub fn get_chealth(&self) -> i16 { self.curr_health }

    pub fn get_mhealth(&self) -> i16 { self.max_health }

    pub fn get_location(&self) -> u32 { self.curr_location }

    pub fn get_level(&self) -> u16 { self.level }
}
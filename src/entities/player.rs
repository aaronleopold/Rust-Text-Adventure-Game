extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;

use crate::interactables::*;
use crate::game_window::*;

pub struct Player
{
    //name: String,
    level: u32,
    max_health: i32,
    curr_health: i32,
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
            level: 1,
            max_health: 10,
            curr_health: 10,
            alive: true,
            curr_location: 1000
        }
    }

    pub fn take_damage(&mut self, amount: i32) -> bool
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
        let message = "Finally, you're awake. Do you know where you are?";
        wprintw(win.get_win(), &fill(message, win.get_max_x() as usize));
    }

    pub fn heal(&mut self, amount: i32) { self.curr_health += amount; }

    pub fn is_alive(&self) -> bool { self.alive }

    pub fn get_chealth(&self) -> i32 { self.curr_health }
}
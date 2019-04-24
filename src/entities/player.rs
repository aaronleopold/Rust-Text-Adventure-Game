extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;

use crate::interactables::*;
use crate::game_window::*;

pub enum Direction {NORTH, EAST, SOUTH, WEST}

pub struct Player
{
    //name: String,
    level: u16,
    max_health: i16,
    curr_health: i16,
    alive: bool,
    curr_room: Room
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
            curr_room: Room::new(
                String::from("Unknown room"),
                String::from("\nThere is nothing here, just darkness. The faintest light, however, peers from the east\n"),
                1000, -1, 1002, -1, -1) // hard coded for now
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
        //wprintw(win.get_win(), "\n");
        wprintw(win.get_win(), &fill(
            self.curr_room().get_prompt().as_str(), win.get_max_x() as usize));
    }

    pub fn travel(&mut self, direction: Direction, rooms: &RoomGraph) -> bool
    {
        let result = match direction 
        {
            Direction::NORTH => {
                if self.curr_room.get_north() == -1 { // doesn't exist
                    None
                }
                else {
                    self.curr_room = Room::clone(rooms.get_room(self.curr_room().get_north()));
                    Some(true)
                }
            }
            
            Direction::EAST => {
                if self.curr_room.get_east() == -1 { // doesn't exist
                    None
                }
                else {
                    self.curr_room = Room::clone(rooms.get_room(self.curr_room().get_east()));
                    Some(true)
                }

            }

            Direction::SOUTH => {
                if self.curr_room.get_south() == -1 { // doesn't exist
                    None
                }
                else {
                    self.curr_room = Room::clone(rooms.get_room(self.curr_room().get_south()));
                    Some(true)
                }

            }

            Direction::WEST => {
                if self.curr_room.get_west() == -1 { // doesn't exist
                    None
                }
                else {
                    self.curr_room = Room::clone(rooms.get_room(self.curr_room().get_west()));
                    Some(true)
                }

            }
        };

        if result.is_some() {
            true
        }
        else {
            false
        }
    }

    pub fn heal(&mut self, amount: i16) { self.curr_health += amount; }
    pub fn is_alive(&self) -> bool { self.alive }
    pub fn get_chealth(&self) -> i16 { self.curr_health }
    pub fn get_mhealth(&self) -> i16 { self.max_health }
    pub fn curr_room(&self) -> Room { Room::clone(&self.curr_room) }
    pub fn get_level(&self) -> u16 { self.level }
}
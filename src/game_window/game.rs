extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;
use std::collections::VecDeque;

use crate::interactables::*;
use crate::entities::*;
use crate::entities::Direction;
use crate::game_window::*;

pub struct Game
{
    header: Header,
    main_window: MainWindow,
    player: Player,
    rooms: RoomGraph,
    prev_command: Vec<String>,
    curr_command: Vec<String>
}

impl Game
{
    pub fn new() -> Self
    {
        let mut game = Game {
            header: Header::new(), 
            main_window: MainWindow::new(),
            player: Player::new(),
            rooms: RoomGraph::new(),
            prev_command: Vec::new(),
            curr_command: Vec::new()
        };

        game.rooms.fill(game.player.curr_room());

        game
    }

    pub fn run(&mut self)
    {
        /* INITIAL SETUP */
        // load in rooms, items and npcs

        let mut playing: bool = true;
        let mut x = 0;
        let mut y = 0;
        let mut input: String = String::new();
        
        self.update_header();
        //self.player.look_at_room(&mut(self.main_window));
        wprintw(self.main_window.get_win(), "\nFinally, you're awake. Do you know where you are?\n");
        self.main_window.prompt();

        while playing
        {
            let letter = wgetch(self.main_window.get_win());
            getyx(self.main_window.get_win(), &mut y, &mut x);

            if letter == '\n' as i32 { // user hit enter
                wprintw(self.main_window.get_win(), "\n");

                let full_message = match input.as_ref() {
                    "Unknown room" => {
                        if self.player.curr_room().get_name() == String::from("Unknown room") && self.player.get_moves() == 0 {
                            wprintw(self.main_window.get_win(),"\nWow. Yeah. Well, you got it. You win, I guess. Thanks for playing!\n(Hit any key to quit)\n");
                            wgetch(self.main_window.get_win());
                            break;
                        }
                        else {
                            Some(String::from("\nWhat?\n"))
                        }
                    }

                    "quit" => {
                        playing = false;
                        break;
                    }

                    "look" => {
                        self.player.look_at_room(&mut self.main_window);
                        None
                    }

                    "where am i" => {
                        Some(String::from(format!(
                            "\nCurrent Location: {}\n", self.player.curr_room().get_name()
                        )))
                    }

                    "move north" => {
                        let dir: Direction = Direction::NORTH;
                        let able_to_move = self.player.travel(dir, &self.rooms);
                        if able_to_move {
                            self.player.moved();
                            Some(self.player.curr_room().get_prompt())
                        }
                        else {
                            Some(String::from("\nThere is nothing in that direction\n"))
                        }
                    }

                    "move east" => {
                        let dir: Direction = Direction::EAST;
                        let able_to_move = self.player.travel(dir, &self.rooms);
                        if able_to_move {
                            Some(self.player.curr_room().get_prompt())
                        }
                        else {
                            Some(String::from("\nThere is nothing in that direction\n"))
                        }
                    }

                    "move south" => {
                        let dir: Direction = Direction::SOUTH;
                        let able_to_move = self.player.travel(dir, &self.rooms);
                        if able_to_move {
                            Some(self.player.curr_room().get_prompt())
                        }
                        else {
                            Some(String::from("\nThere is nothing in that direction\n"))
                        }
                    }

                    "move west" => {
                        let dir: Direction = Direction::WEST;
                        let able_to_move = self.player.travel(dir, &self.rooms);
                        if able_to_move {
                            Some(self.player.curr_room().get_prompt())
                        }
                        else {
                            Some(String::from("\nThere is nothing in that direction\n"))
                        }
                    }

                    "flirt" => {
                        if self.player.curr_room().get_npcs().len() == 0 {
                            if !self.player.get_inventory().contains(String::from("mirror")) {
                                Some(String::from("\nYou are completely alone, you don't have a mirror and I'm omniscient. Who will you flirt with?\n"))
                            }
                            else {
                                Some(String::from("\nYou stare intensely into your mirror. My god, you're gorgeous.\n"))
                            }
                            
                        }
                        else {
                            Some(String::from(""))
                        }
                    }

                    "help" => {
                        Some(Game::help_prompt())
                    }

                    _=> {
                        Some(String::from("\nI can't do that yet\n"))
                    }
                };

                match full_message {
                    None => {},
                    Some(message) => {
                        wprintw(self.main_window.get_win(), message.as_ref());
                    }
                };

                self.main_window.prompt();
                self.prev_command.push(input.clone());
                input.clear();
            }

            // if delete (127) was pressed, delete the char on the screen
            else if letter == 127 {
                if input.len() > 0 {
                mvwdelch(self.main_window.get_win(), y, x - 1);
                input.pop(); // remove the char from string too
                }
            }

            // add support for atypical key presses (eg: ~!,<[]= etc)
            else if letter == KEY_UP {
                if self.prev_command.len() > 0 {
                    self.curr_command.push(input.clone());
                    if input != String::from("") {
                        mvwdelch(self.main_window.get_win(), y, x - (input.len() as i32));
                    }

                    input.clear();
                    input = self.prev_command.pop().unwrap();
                    wprintw(self.main_window.get_win(), input.as_str());
                }
            }

            else if letter == KEY_DOWN {
                if self.curr_command.len() > 0 {
                    self.prev_command.push(input.clone());
                    if input != String::from("") {
                        mvwdelch(self.main_window.get_win(), y, x - (input.len() as i32));
                    }                

                    input.clear();
                    input = self.curr_command.pop().unwrap();
                    wprintw(self.main_window.get_win(), input.as_str());
                }
            }

            else {
                waddch(self.main_window.get_win(), letter as u32);
                input.push((letter as u8) as char);
            }

            self.update_header();
            wrefresh(self.main_window.get_win());

        } // while loop
    } // fn

    pub fn get_window(&self) -> &MainWindow { &self.main_window }

    pub fn update_header(&mut self)
    {
        self.header.update(
            self.player.curr_room().get_name(),
            self.player.get_level(), self.player.get_chealth(),
            self.player.get_mhealth()
        );
    }

    #[inline]
    pub fn help_prompt() -> String
    {
        String::from(
            "\nPossible actions:\n
            help                        brings up this help screen\n
            where am i                  displays current room\n
            up arrow                    retrieve last input\n
            down arrow                  retrieve next (from previous) input\n
            move [cardinal direction]   moves player in direction\n
            flee [cardinal direction]   flees from fight to direction\n
            look                        player inspects room\n
            speak to [name of npc]      initiates dialogue between player and npc\n
            attack                      attacks enemy\n
            use [item in inventory]     use item selected\n
            collect [item in room]      collect item from room\n
            equipt [item in inventory]  equipt item from inventory\n
            drop [item in inventory]    remove item from inventory\n" 
        )
    }

}


/*
    STACK:                          STACK2:
        help (prev)
        help (prev)
        move east (prev)

    up arrow                            ""
        
        STACK:
            help
            help
        CURR move east

    up arrow

        STACK:                      STACK2:
            help                        ""
        CURR help                       move east
*/
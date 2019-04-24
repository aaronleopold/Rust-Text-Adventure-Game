extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;

use crate::interactables::*;
use crate::entities::*;
use crate::game_window::*;

pub struct Game
{
    header: Header,
    main_window: MainWindow,
    player: Player,
    rooms: RoomGraph,
}

impl Game
{
    pub fn new() -> Self
    {
        Game {
            header: Header::new(), 
            main_window: MainWindow::new(),
            rooms: RoomGraph::new(),
            player: Player::new(),
        }
    }

    pub fn run(&mut self)
    {
        let mut playing: bool = true;
        let mut x = 0;
        let mut y = 0;
        let mut input: String = String::new();
        
        self.update_header();
        self.player.look_at_room(&mut(self.main_window));
        self.main_window.prompt();

        while playing
        {
            let letter = wgetch(self.main_window.get_win());
            getyx(self.main_window.get_win(), &mut y, &mut x);

            if letter == '\n' as i32 { // user hit enter
                wprintw(self.main_window.get_win(), "\n");

                let full_message = match input.as_ref() {
                    "quit" => {
                        playing = false;
                        break;
                    }

                    "look" => {
                        self.player.look_at_room(&mut self.main_window);
                        None
                    }

                    _=> {
                        Some("\nI can't do that yet\n")
                    }
                };

                match full_message {
                    None => {},
                    Some(message) => {
                        wprintw(self.main_window.get_win(), message);
                    }
                };

                self.main_window.prompt();
                input.clear();
            }

            // if delete (127) was pressed, delete the char on the screen
            else if letter == 127 {
                mvwdelch(self.main_window.get_win(), y, x - 1);
                input.pop(); // remove the char from string too
            }

            // add support for atypical key presses (eg: ~!,<[]= etc)

            else {
                waddch(self.main_window.get_win(), letter as u32);
                input.push((letter as u8) as char);
            }

            wrefresh(self.main_window.get_win());
        }
    }

    pub fn update_header(&mut self)
    {
        self.header.update(
            self.rooms.get_room(self.player.get_location()).get_name(),
            self.player.get_level(), self.player.get_chealth(),
            self.player.get_mhealth()
        );
    }

}


/*

GET LINE METHOD, BUGGED

pub fn run(&mut self)
    {
        let mut playing: bool = true;
        let mut x = 0;
        let mut y = 0;
        //let mut input: String = String::new();
        
        self.update_header();
        self.player.look_at_room(&mut(self.main_window));
        self.main_window.prompt();

        while playing
        {
            let mut input: String = String::new();
            wgetstr(self.main_window.get_win(), &mut input);
            getyx(self.main_window.get_win(), &mut y, &mut x);

            let result: Option<&str> = match input.as_ref() {
                "quit" => {
                    playing = false;
                    break;
                }

                "look" => {
                    self.update_header();
                    self.player.look_at_room(&mut(self.main_window));
                    None
                }

                _ => {
                    self.update_header();
                    self.player.look_at_room(&mut(self.main_window));
                    None
                }
            };

            match result {
                None => {},
                Some(message) => {
                    wprintw(self.main_window.get_win(), message);
                }
            };
            self.main_window.prompt();
            wrefresh(self.main_window.get_win());
        }
    }
*/
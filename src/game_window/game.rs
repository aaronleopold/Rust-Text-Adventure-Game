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
        let mut x = 0;
        let mut y = 0;
        let mut input: String = String::new();

        self.player.look_at_room(&mut(self.main_window));
        self.main_window.prompt();

        loop 
        {
            let ch = wgetch(self.main_window.get_win());
            getyx(self.main_window.get_win(), &mut y, &mut x);

            if ch == '\n' as i32 {
                wprintw(self.main_window.get_win(), "\n");
                let result: Option<&str> = match input.as_ref() {
                    "quit" => {
                        break;
                    }

                    "look" => {
                        self.player.look_at_room(&mut(self.main_window));
                        None
                    }

                    _ => {
                        Some("I don't understand yet.\n")
                    }
                };

                match result {
                    None => {},
                    Some(message) => {
                        wprintw(self.main_window.get_win(), message);
                    }
                };
                self.main_window.prompt();
            }

            else if ch == 'q' as i32 {
                break;
            }
        }

    }
}
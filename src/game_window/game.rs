extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

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
    curr_command: Vec<String>,
    playing: bool
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
            curr_command: Vec::new(),
            playing: true
        };

        game.rooms.insert(game.player.curr_room());

        game
    }

    pub fn run(&mut self)
    {
        let mut playing: bool = true;
        let mut x = 0;
        let mut y = 0;
        let mut input: String = String::new();
        
        self.update_header();
        //self.player.look_at_room(&mut(self.main_window));
        wprintw(self.main_window.get_win(), "\nFinally, you're awake. Do you know where you are?\n\n");
        //self.main_window.prompt();

        while self.playing
        {
            let mut input: String = self.get_input();
            self.interpret_input_explore(&mut input);
            self.update_header();
        }
    }

    pub fn load(&mut self) 
    {
        // open loading file
        let filename = "test_loading.txt";
        // load in rooms, items and npc object
        let file = File::open(filename).expect("FILE NOT FOUND!\n");

        let reader = BufReader::new(file);        

        for (index, line) in reader.lines().enumerate()
        {
            if index == 0 { continue; } // start room is hardcoded

            let line = line.unwrap();
            let mut split = line.split(":");
            let room_info = split.collect::<Vec<&str>>();
            let mut it = 0;

            // get room name and descr
            let room_name: String = String::from(room_info[0]);
            let room_descr: String = String::from(room_info[1]);

            // get room id, and id of rooms NESW
            let room_id: i32 = room_info[2].parse().unwrap();
            let room_north: i32 = room_info[3].parse().unwrap();
            let room_east: i32 = room_info[4].parse().unwrap();
            let room_south: i32 = room_info[5].parse().unwrap();
            let room_west: i32 = room_info[6].parse().unwrap();

            // get npc count
            let npc_count: i16 = room_info[7].parse().unwrap();
            it += 7; // must start using iteration placeholder

            let mut room_npcs: Vec<NPC> = Vec::new();
            if npc_count != 0 {
                // loop for each npc
                for i in 0..npc_count {
                    let npc_name = String::from(room_info[it + 1]);
                    let npc_nice: bool = room_info[it + 2].parse().unwrap();
                    let npc_name_response: String = String::from(room_info[it + 3]);
                    let npc_dialogue = String::from(room_info[it + 4]);
                    let npc_description: String = String::from(room_info[it + 5]);
                    it += 5;

                    room_npcs.push(
                        NPC::new(npc_name, npc_dialogue, npc_nice, npc_name_response, npc_description)
                    );
                }
            }
            it += 1;        

            // get item count
            let item_count: u16 = room_info[it].parse().unwrap();
            //it += 1;


            // loop for each item
            let mut room_items: Vec<Item> = Vec::new();
            if item_count != 0 {
                for i in 0..item_count {
                    let item_name: String = String::from(room_info[it + 1]);
                    let item_desc: String = String::from(room_info[it + 2]);

                    it += 2;

                    room_items.push(Item::new(item_name, item_desc, false));
                }
            }
            it += 1;

            // get weapon count
            let weapon_count: u16 = room_info[it].parse().unwrap();

            // loop for each weapon
            let mut room_weapons: Vec<Weapon> = Vec::new();
            if weapon_count != 0 {
                for i in 0..weapon_count {
                    let weapon_name: String = String::from(room_info[it + 1]);
                    let weapon_damage: u8 = room_info[it + 2].parse().unwrap();

                    it += 2;
                }
            }
            it += 1;

            // get enemy count
            let enemy_count: u8 = room_info[it].parse().unwrap();

            // loop for each enemy
            let mut room_enemies: Vec<Enemy> = Vec::new();
            if enemy_count != 0 {
                for i in 0..enemy_count {
                    let enemy_name: String = String::from(room_info[it + 1]);
                    let enemy_level: u8 = room_info[it + 2].parse().unwrap();
                    let enemy_mhealth: i8 = room_info[it + 3].parse().unwrap();
                    let enemy_chealth: i8 = room_info[it + 4].parse().unwrap();

                    it += 4;
                }
            }
            it += 1;

            // create room and add to self.rooms
            let mut new_room = Room::new(
                room_name, room_descr, room_id, room_north, room_east, room_south, room_west,
                room_npcs, room_items, room_weapons, room_enemies
            );

            self.rooms.insert(new_room);
        }
    }

    pub fn get_input(&mut self) -> String
    {
        let mut x = 0;
        let mut y = 0;
        let mut input: String = String::new();

        self.main_window.prompt();

        loop
        {
            let letter = wgetch(self.main_window.get_win());
            getyx(self.main_window.get_win(), &mut y, &mut x);
            
            if letter == '\n' as i32 { // user hit enter
                wprintw(self.main_window.get_win(), "\n");

                //self.main_window.prompt();
                self.prev_command.push(input.clone());

                break;
            }

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
        }

        input.clone()
    }

    pub fn interpret_input_explore(&mut self, input: &mut String)
    {
        if input.to_lowercase().contains("speak to ") {
            let name: String = String::from(&input[9..]);
            //wprintw(self.main_window.get_win(), name.as_ref());
            let mut message = String::new();

            // check the name exists in room
            if self.player.curr_room().has_npc(name.clone()) {
                let dialogue: String = self.player.curr_room().get_npc_dialogue(name.clone());

                if dialogue == "" { message = String::from("\nthere is nobody in the room who goes by that\n"); }
                else {
                    message = String::from("\n\n") + self.player.curr_room().get_npc_dialogue(name.clone()).as_str() + "\n\n";
                }
            }

            else {
                message = String::from("\n\nthere is nobody in the room who goes by that\n\n");
            }

            wprintw(self.main_window.get_win(), message.as_ref());
            //self.main_window.prompt();
            self.prev_command.push(input.clone());
            input.clear();

            return;
        }


        if input.to_lowercase().contains("collect ") {
            let item: String = String::from(&input[8..]);
            let mut message = String::new();

            // check that the item exists in the room
            if self.player.curr_room().has_item(item.clone()) || self.player.curr_room().has_weapon(item.clone()) {
                let can_collect: bool = !self.player.get_inventory().is_full();

                if can_collect {
                    self.player.add_to_inventory(item.clone());
                    message = String::from(format!("\n\n{} added to inventory\n\n", item.clone()));
                }
                else {
                    message = String::from("\n\ninventory is full\n\n");
                }
            }

            else {
                message = String::from(format!("\n\nthere is no item {} in the room\n\n", item.as_str()));
            }

            wprintw(self.main_window.get_win(), message.as_ref());
            //self.main_window.prompt();
            self.prev_command.push(input.clone());
            input.clear();
            return;
        }

        let full_message = match input.to_lowercase().as_ref() {
            "unknown room" => {
                if self.player.curr_room().get_name() == String::from("Unknown room") && self.player.get_moves() == 0 {
                    wprintw(self.main_window.get_win(),"\nwow. yeah. well, you got it. you win, i guess. thanks for playing!\n(hit any key to quit)\n");
                    wgetch(self.main_window.get_win());
                    return;
                }
                else {
                    Some(String::from("\nwhat?\n"))
                }
            }

            "no" => {
                if self.player.curr_room().get_name() == String::from("unknown room") && self.player.get_moves() == 0 {
                    Some(Game::help_prompt())
                }

                else {
                    Some(String::from("\nhow does that make sense? why did you even try that?\n"))
                }
            }

            "quit" => {
                self.playing = false;
                return;
            }

            "look" => {
                self.player.look_at_room(&mut self.main_window);
                None
            }

            "where am i" => {
                Some(String::from(format!(
                    "\ncurrent Location: {}\n", self.player.curr_room().get_name()
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
                    Some(String::from("\nthere is nothing in that direction\n"))
                }
            }

            "move east" => {
                let dir: Direction = Direction::EAST;
                let able_to_move = self.player.travel(dir, &self.rooms);
                if able_to_move {
                    Some(self.player.curr_room().get_prompt())
                }
                else {
                    Some(String::from("\nthere is nothing in that direction\n"))
                }
            }

            "move south" => {
                let dir: Direction = Direction::SOUTH;
                let able_to_move = self.player.travel(dir, &self.rooms);
                if able_to_move {
                    Some(self.player.curr_room().get_prompt())
                }
                else {
                    Some(String::from("\nthere is nothing in that direction\n"))
                }
            }

            "move west" => {
                let dir: Direction = Direction::WEST;
                let able_to_move = self.player.travel(dir, &self.rooms);
                if able_to_move {
                    Some(self.player.curr_room().get_prompt())
                }
                else {
                    Some(String::from("\nthere is nothing in that direction\n"))
                }
            }

            "ask name" => {
                if self.player.curr_room().get_npcs().len() == 1 {
                    if self.player.curr_room().get_npcs()[0].is_nice() {
                        let response: String = 
                        String::from("\n") +
                        self.player.curr_room().get_npcs()[0].get_name_response().as_str() +
                        "\n";
                        
                        Some(response)
                    }
                    else {
                        let response: String = 
                        String::from("\nthe ") +
                        self.player.curr_room().get_npcs()[0].get_description().as_str() +
                        " doesn't seem to want to share\n";
                        
                        Some(response)
                    }
                    
                }
                else if self.player.curr_room().get_npcs().len() == 0 {
                    Some(String::from("\nThere is nobody else here. You're completely alone\n"))
                }
                else {
                    Some(String::from("\nI can't handle that yet. Try again soon\n"))
                }
            }

            "show inventory" => {
                if self.player.get_inventory().get_len() == 0 {
                    let inventory = String::from("\ninventory is empty, there is nothing to show\n");
                    Some(inventory)
                }
                else {
                    let mut inventory = String::from("\n\n");

                    for i in 0..self.player.get_inventory().get_items().len() {
                        inventory += self.player.get_inventory().get_items()[i].get_name().as_str();
                        inventory += "\n";
                    }

                    inventory += "\n";

                    Some(inventory)

                }
            }

            "room items" => {
                if self.player.curr_room().get_items().len() == 0 && self.player.curr_room().get_weapons().len() == 0 {
                    Some(String::from("\nthis room has no items\n"))
                }
                else {
                    let mut message = String::from("\n");

                    for i in 0..self.player.curr_room().get_items().len() {
                        message += self.player.curr_room().get_items()[i].get_name().as_str();
                        message += "\n";
                    }

                    for i in 0..self.player.curr_room().get_weapons().len() {
                        message += self.player.curr_room().get_weapons()[i].get_name().as_str();
                        message += "\n";
                    }

                    message += "\n";

                    Some(message)
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

        //self.main_window.prompt();
        self.prev_command.push(input.clone());
        input.clear();
    }

    pub fn interpret_input_fight(&mut self)
    {

    }

    pub fn combat_state(&mut self) -> bool
    {
        // returns false upon death
        while self.player.is_fighting()
        {
            // check for alive enemies in room


            // get_input to select enemy to attack
            // calculate chances to miss enemy
            // attack if not missed
            // calculate enemy chance to miss
            // attack player if not missed
            // check alive status of player/enemies

            // update player.is_fighting upon enemy/player death
        }

        false
    }

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
            "\na few possible actions:\n
            help                        brings up this help screen\n
            where am i                  displays current room\n
            up arrow                    retrieve last input\n
            down arrow                  retrieve next (from previous) input\n
            move [cardinal direction]   moves player in direction\n
            flee [cardinal direction]   flees from fight to direction\n
            look                        player inspects room\n
            room items                  displays collectable items in room\n
            ask name                    npc may reveal name to you\n
            speak to [name of npc]      initiates dialogue between player and npc\n
            attack                      attacks enemy\n
            show inventory              displays contents of inventory\n
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



/*
pub fn run(&mut self)
    {
        let mut playing: bool = true;
        let mut x = 0;
        let mut y = 0;
        let mut input: String = String::new();
        
        self.update_header();
        //self.player.look_at_room(&mut(self.main_window));
        wprintw(self.main_window.get_win(), "\nFinally, you're awake. Do you know where you are?\n\n");
        self.main_window.prompt();

        while playing
        {
            let letter = wgetch(self.main_window.get_win());
            getyx(self.main_window.get_win(), &mut y, &mut x);
            
            if letter == '\n' as i32 { // user hit enter
                wprintw(self.main_window.get_win(), "\n");

                if input.to_lowercase().contains("speak to ") {
                    let name: String = String::from(&input[9..]);
                    //wprintw(self.main_window.get_win(), name.as_ref());
                    let mut message = String::new();

                    // check the name exists in room
                    if self.player.curr_room().has_npc(name.clone()) {
                        let dialogue: String = self.player.curr_room().get_npc_dialogue(name.clone());

                        if dialogue == "" { message = String::from("\nthere is nobody in the room who goes by that\n"); }
                        else {
                            message = String::from("\n\n") + self.player.curr_room().get_npc_dialogue(name.clone()).as_str() + "\n\n";
                        }
                    }

                    else {
                        message = String::from("\n\nthere is nobody in the room who goes by that\n\n");
                    }

                    wprintw(self.main_window.get_win(), message.as_ref());
                    self.main_window.prompt();
                    self.prev_command.push(input.clone());
                    input.clear();
                    continue;

                }

                if input.to_lowercase().contains("collect ") {
                    let item: String = String::from(&input[8..]);
                    let mut message = String::new();

                    // check that the item exists in the room
                    if self.player.curr_room().has_item(item.clone()) || self.player.curr_room().has_weapon(item.clone()) {
                        let can_collect: bool = !self.player.get_inventory().is_full();

                        if can_collect {
                            self.player.add_to_inventory(item.clone());
                            message = String::from(format!("\n\n{} added to inventory\n\n", item.clone()));
                        }
                        else {
                            message = String::from("\n\ninventory is full\n\n");
                        }
                    }
                    else {
                        message = String::from(format!("\n\nthere is no item {} in the room\n\n", item.as_str()));
                    }

                    wprintw(self.main_window.get_win(), message.as_ref());
                    self.main_window.prompt();
                    self.prev_command.push(input.clone());
                    input.clear();
                    continue;
                }

                let full_message = match input.to_lowercase().as_ref() {
                    "unknown room" => {
                        if self.player.curr_room().get_name() == String::from("Unknown room") && self.player.get_moves() == 0 {
                            wprintw(self.main_window.get_win(),"\nwow. yeah. well, you got it. you win, i guess. thanks for playing!\n(hit any key to quit)\n");
                            wgetch(self.main_window.get_win());
                            break;
                        }
                        else {
                            Some(String::from("\nwhat?\n"))
                        }
                    }

                    "no" => {
                        if self.player.curr_room().get_name() == String::from("unknown room") && self.player.get_moves() == 0 {
                            Some(Game::help_prompt())
                        }

                        else {
                            Some(String::from("\nhow does that make sense? why did you even try that?\n"))
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
                            "\ncurrent Location: {}\n", self.player.curr_room().get_name()
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
                            Some(String::from("\nthere is nothing in that direction\n"))
                        }
                    }

                    "move east" => {
                        let dir: Direction = Direction::EAST;
                        let able_to_move = self.player.travel(dir, &self.rooms);
                        if able_to_move {
                            Some(self.player.curr_room().get_prompt())
                        }
                        else {
                            Some(String::from("\nthere is nothing in that direction\n"))
                        }
                    }

                    "move south" => {
                        let dir: Direction = Direction::SOUTH;
                        let able_to_move = self.player.travel(dir, &self.rooms);
                        if able_to_move {
                            Some(self.player.curr_room().get_prompt())
                        }
                        else {
                            Some(String::from("\nthere is nothing in that direction\n"))
                        }
                    }

                    "move west" => {
                        let dir: Direction = Direction::WEST;
                        let able_to_move = self.player.travel(dir, &self.rooms);
                        if able_to_move {
                            Some(self.player.curr_room().get_prompt())
                        }
                        else {
                            Some(String::from("\nthere is nothing in that direction\n"))
                        }
                    }

                    "ask name" => {
                        if self.player.curr_room().get_npcs().len() == 1 {
                            if self.player.curr_room().get_npcs()[0].is_nice() {
                                let response: String = 
                                String::from("\n") +
                                self.player.curr_room().get_npcs()[0].get_name_response().as_str() +
                                "\n";
                                
                                Some(response)
                            }
                            else {
                                let response: String = 
                                String::from("\nthe ") +
                                self.player.curr_room().get_npcs()[0].get_description().as_str() +
                                " doesn't seem to want to share\n";
                                
                                Some(response)
                            }
                            
                        }
                        else if self.player.curr_room().get_npcs().len() == 0 {
                            Some(String::from("\nThere is nobody else here. You're completely alone\n"))
                        }
                        else {
                            Some(String::from("\nI can't handle that yet. Try again soon\n"))
                        }
                    }

                    "show inventory" => {
                        if self.player.get_inventory().get_len() == 0 {
                            let inventory = String::from("\ninventory is empty, there is nothing to show\n");
                            Some(inventory)
                        }
                        else {
                            let mut inventory = String::from("\n\n");

                            for i in 0..self.player.get_inventory().get_items().len() {
                                inventory += self.player.get_inventory().get_items()[i].get_name().as_str();
                                inventory += "\n";
                            }

                            inventory += "\n";

                            Some(inventory)

                        }
                    }

                    "room items" => {
                        if self.player.curr_room().get_items().len() == 0 && self.player.curr_room().get_weapons().len() == 0 {
                            Some(String::from("\nthis room has no items\n"))
                        }
                        else {
                            let mut message = String::from("\n");

                            for i in 0..self.player.curr_room().get_items().len() {
                                message += self.player.curr_room().get_items()[i].get_name().as_str();
                                message += "\n";
                            }

                            for i in 0..self.player.curr_room().get_weapons().len() {
                                message += self.player.curr_room().get_weapons()[i].get_name().as_str();
                                message += "\n";
                            }

                            message += "\n";

                            Some(message)
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
*/
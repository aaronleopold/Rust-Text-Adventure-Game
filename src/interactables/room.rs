use std::collections::HashMap;
use std::boxed::Box;

use std::process;
use crate::entities::*;
use crate::interactables::*;

pub struct Room
{
    name: String,
    room_prompt: String,
    id: i32,
    north_room: i32,
    east_room: i32,
    south_room: i32,
    west_room: i32,
    npcs: Vec<NPC>,
    items: Vec<Item>,
    weapons: Vec<Weapon>,
    enemies: Vec<Enemy>
}

pub struct RoomGraph
{
    graph: HashMap<i32, Room>,
    size: u32
}

// fix hard coding when bin file created
// create diff formula for calculating NESW rooms?? 
impl Room
{
    pub fn new(
        name: String, room_prompt: String, id: i32, north: i32, east: i32, 
        south: i32, west: i32, npcs: Vec<NPC>, items: Vec<Item>,
        weapons: Vec<Weapon>, enemies: Vec<Enemy>) -> Self
    {
        Room {
            name: name.clone(),
            room_prompt: room_prompt.clone(),
            id: id,
            north_room: north,
            east_room: east,
            south_room: south,
            west_room: west,
            npcs: npcs,
            items: items,
            weapons: weapons,
            enemies: enemies
        }
    }

    #[inline]
    pub fn clone(room: &Room) -> Self 
    {
        let mut new_room = Room {
            name: room.get_name(),
            room_prompt: room.get_prompt(),
            id: room.get_id(),
            north_room: room.get_north(),
            east_room: room.get_east(),
            south_room: room.get_south(),
            west_room: room.get_west(),
            npcs: Vec::new(),
            items: Vec::new(),
            weapons: Vec::new(),
            enemies: Vec::new()
        };

        for i in 0..(room.get_npcs().len()) {
            let new_npc: NPC = NPC::clone(&room.get_npcs()[i]);
            new_room.npcs.push(new_npc);
        }

        for i in 0..(room.get_items().len()) {
            let new_item: Item = Item::clone(&room.get_items()[i]);
            new_room.items.push(new_item);
        }

        for i in 0..(room.get_weapons().len()) {
            let new_weapon: Weapon = Weapon::clone(&room.get_weapons()[i]);
            new_room.weapons.push(new_weapon);
        }

        for i in 0..(room.get_enemies().len()) {

        }

        new_room
    }

    pub fn get_name(&self) -> String { self.name.clone() }

    pub fn get_prompt(&self) -> String 
    {
        let mut ret: String = String::new();
        ret += "\n";
        ret += self.room_prompt.clone().as_str();
        ret += "\n";
        
        ret
    }

    pub fn get_id(&self) -> i32 { self.id }
    pub fn get_north(&self) -> i32 { self.north_room }
    pub fn get_east(&self) -> i32 { self.east_room }
    pub fn get_south(&self) -> i32 { self.south_room }
    pub fn get_west(&self) -> i32 { self.west_room }
    pub fn get_npcs(&self) -> &Vec<NPC> { &self.npcs }
    pub fn get_items(&self) -> &Vec<Item> { &self.items }
    pub fn get_weapons(&self) -> &Vec<Weapon> { &self.weapons }
    pub fn get_enemies(&self) -> &Vec<Enemy> { &self.enemies }
}

impl RoomGraph
{
    pub fn new() -> Self
    {
        let mut graph = RoomGraph {
            graph: HashMap::new(),
            size: 0
        };

        //graph.fill();
        graph
    }

    pub fn get_room(&self, id: i32) -> &Room
    {
        let ret = self.graph.get(&id);
        if ret.is_some() {
            ret.unwrap()
        }
        else {
            process::exit(1);
        }
    }

    pub fn insert(&mut self, room: Room) { self.graph.insert(room.get_id(), room); }

    pub fn get_graph(&self) -> &HashMap<i32, Room> { &self.graph }
}

/*
    to do: 
        - create custom bin file of rooms and locations in relation to each other
        - implement file io to recursively create rooms for the graph of rooms in 
        game.rs
*/
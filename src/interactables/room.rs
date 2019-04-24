use std::collections::HashMap;
use std::boxed::Box;

use std::process;

pub struct Room
{
    name: String,
    room_prompt: String,
    id: i32,
    north_room: i32,
    east_room: i32,
    south_room: i32,
    west_room: i32
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
        name: String, room_prompt: String, id: i32,
        north: i32, east: i32, south: i32, west: i32) -> Self
    {
        Room {
            name: name.clone(),
            room_prompt: room_prompt.clone(),
            id: id,
            north_room: north,
            east_room: east,
            south_room: south,
            west_room: west
        }
    }

    #[inline]
    pub fn clone(room: &Room) -> Self 
    {
        Room {
            name: room.get_name(),
            room_prompt: room.get_prompt(),
            id: room.get_id(),
            north_room: room.get_north(),
            east_room: room.get_east(),
            south_room: room.get_south(),
            west_room: room.get_west()
        }
    }

    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_prompt(&self) -> String { self.room_prompt.clone() }
    pub fn get_id(&self) -> i32 { self.id }
    pub fn get_north(&self) -> i32 { self.north_room }
    pub fn get_east(&self) -> i32 { self.east_room }
    pub fn get_south(&self) -> i32 { self.south_room }
    pub fn get_west(&self) -> i32 { self.west_room }
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

/*
    pub fn create_room(&mut self, curr: &Room, names: &Vec<String>)
    {

    }
*/

    // hard coded now until bin file type created
    pub fn fill(&mut self, start: Room)
    {
        let mut first_room: Room = Room::clone(&start);
        
        let mut east_room: Room = Room::new(
            String::from("Dingy corridor"),
            String::from("\nThis corridor is really dark and gross. There's a door ahead though.\n"),
            1002, -1, -1, -1, 1000);
        let mut north_room: Room = Room::new(
            String::from("north room"), 
            String::from(""),
            -1, -1, -1, -1, -1);
        let mut south_room: Room = Room::new(
            String::from("south room"), 
            String::from(""),
            -1, -1, -1, -1, -1);
        let mut west_room: Room = Room::new(
            String::from("west room"), 
            String::from(""),
            -1, -1, -1, -1, -1);

        self.graph.insert(1000, first_room);
        self.graph.insert(1002, east_room);
    }
}

/*
    to do: 
        - create custom bin file of rooms and locations in relation to each other
        - implement file io to recursively create rooms for the graph of rooms in 
        game.rs
*/
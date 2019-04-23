use std::collections::HashMap;
use std::boxed::Box;

pub struct Room
{
    name: String,
    id: u32,
    north_room: u32,
    east_room: u32,
    south_room: u32,
    west_room: u32
}

pub struct RoomGraph
{
    graph: HashMap<u32, Room>,
    size: u32
}

// fix hard coding when bin file created
// create diff formula for calculating NESW rooms?? 
impl Room
{
    pub fn new(
        name: String/*, north_name: String, east_name: String, 
        south_name: String, west_name: String */, id: u32) -> Self
    {
        Room {
            name: name.clone(),
            id: id,
            north_room: id + 1,
            east_room: id + 2,
            south_room: id + 3,
            west_room: id + 4
        }
    }
}

impl RoomGraph
{
    pub fn new() -> Self
    {
        let mut graph = RoomGraph {
            graph: HashMap::new(),
            size: 0
        };

        graph.fill();
        graph
    }

/*
    pub fn create_room(&mut self, curr: &Room, names: &Vec<String>)
    {

    }
*/

    // hard coded now until bin file type created
    pub fn fill(&mut self)
    {
        let mut first_room: Room = Room::new(String::from("Unknown room"), 1000);
        let mut east_room: Room = Room::new(String::from("Unknown corridor"), 1002);
        let mut north_room: Room = Room::new(String::from("north room"), 1001);
        let mut south_room: Room = Room::new(String::from("south room"), 1003);
        let mut west_room: Room = Room::new(String::from("west room"), 1004);

        self.graph.insert(1000, first_room);
        self.graph.insert(1002, east_room);
        self.graph.insert(1001, north_room);
        self.graph.insert(1003, south_room);
        self.graph.insert(1004, west_room);
    }
}

/*
    to do: 
        - create custom bin file of rooms and locations in relation to each other
        - implement file io to recursively create rooms for the graph of rooms in 
        game.rs
*/
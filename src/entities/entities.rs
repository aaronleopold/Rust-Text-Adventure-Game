/*//#[crate_id = "entities"];
//#[crate_type = "lib"];

enum EnemyType {GOBLIN, OGRE, MIMIC}
enum NPCType {MALE, FEMALE, OTHER}

// struct declarations
pub struct Player
{
    //name: String,
    level: u32,
    max_health: i32,
    curr_health: i32,
    alive: bool,
}

struct NPC
{
    _type: NPCType,
    name: String,
    // items to give?
}
/*
    npc's should eventually have unique items to give based on level? 
*/

struct Enemy
{
    _type: EnemyType,
    name: String,
    level: u32,
    max_health: i32,
    curr_health: i32,
    alive: bool,
}

// struct implementations
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

    pub fn heal(&mut self, amount: i32) { self.curr_health += amount; }

    pub fn is_alive(&self) -> bool { self.alive }

    pub fn get_chealth(&self) -> i32 { self.curr_health }
}

impl NPC
{
    pub fn new() -> Self
    {
        NPC {
            // randomize typing runtime?
            _type: NPCType::OTHER,
            name: String::from("temporary npc name"),
        }
    }
}

impl Enemy
{
    fn new(_type: EnemyType) -> Self
    {
        Enemy {
            _type: _type,
            // random generation of name from pool
            // using precreated list
            name: String::from("Goblin test"),
            level: 1,
            max_health: 10,
            curr_health: 10,
            alive: true,
        }
    }

    fn take_damage(&mut self, amount: i32) -> bool
    {
        // function will return false upon death
        self.curr_health -= amount;
        if self.curr_health <= 0 {
            self.alive = false;
        }

        self.alive // return syntax?
    }

    fn heal(&mut self, amount: i32) { self.curr_health += amount; }
}

/*
    Notes:
        I feel like rust would benefit from a more fleshed out, traditional inheritance
        model. The lack of reusabiliy is a major drawback.
*/


*/
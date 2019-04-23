enum EnemyType {GOBLIN, OGRE, MIMIC}

pub struct Enemy
{
    _type: EnemyType,
    name: String,
    level: u32,
    max_health: i32,
    curr_health: i32,
    alive: bool,
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
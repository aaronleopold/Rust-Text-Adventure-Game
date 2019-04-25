pub struct Enemy
{
    name: String,
    level: u8,
    max_health: i8,
    curr_health: i8,
    alive: bool,
}

impl Enemy
{
    fn new(name: String, level: u8, mhealth: i8, chealth: i8) -> Self
    {
        Enemy {
            name: name,
            level: level,
            max_health: 10,
            curr_health: 10,
            alive: true,
        }
    }

    fn take_damage(&mut self, amount: i8) -> bool
    {
        // function will return false upon death
        self.curr_health -= amount;
        if self.curr_health <= 0 {
            self.alive = false;
        }

        self.alive // return syntax?
    }

    fn heal(&mut self, amount: i8) { self.curr_health += amount; }
}
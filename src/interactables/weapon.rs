pub struct Weapon
{
    name: String,
    damage: u8
}

impl Weapon
{
    pub fn new(name: String, damage: u8) -> Self
    {
        Weapon {
            name: name,
            damage: damage
        }
    }

    pub fn clone(weapon: &Weapon) -> Self
    {
        Weapon {
            name: weapon.get_name(),
            damage: weapon.get_damage()
        }
    }

    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_damage(&self) -> u8 { self.damage }
}
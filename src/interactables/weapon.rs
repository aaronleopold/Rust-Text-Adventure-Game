pub struct Weapon
{
    name: String
}

impl Weapon
{
    pub fn new(name: String) -> Self
    {
        Weapon {
            name: name
        }
    }

    pub fn get_name(&self) -> String { self.name.clone() }
}
use crate::interactables::*;

pub struct Inventory
{
    max_items: u8,
    curr_items: u8,
    weapons: Vec<Weapon>,
    items: Vec<Item>
}

impl Inventory
{
    pub fn new() -> Self
    {
        // starter note
        let mut note: Item = Item::new(String::from("strange note"), 
            String::from("\neventually, this will actually say something of value to the story\n"), 
            false);

        let mut inventory = Inventory {
            max_items: 10,
            curr_items: 1,
            weapons: Vec::new(),
            items: Vec::new()
        };

        inventory.items.push(note);

        inventory
    }

    pub fn contains(&self, item: String) -> bool
    {
        let mut ret: bool = false; 

        for i in 0..self.weapons.len() {
            if self.weapons[i].get_name() == item {
                ret = true;
            }
        }

        for i in 0..self.items.len() {
            if self.items[i].get_name() == item {
                ret = true;
            }
        }

        ret
    }

    pub fn add_item(&mut self, item: Item) -> bool 
    {
        let mut ret: bool = false;

        if self.get_len() < self.max_items as usize {
            // add it
            self.items.push(item);
            self.curr_items += 1;

            ret = true;
        }

        ret
    }

    pub fn add_weapon(&mut self, weapon: Weapon) -> bool 
    {
        let mut ret: bool = false;

        if self.get_len() < self.max_items as usize {
            // add it
            self.weapons.push(weapon);
            self.curr_items += 1;

            ret = true;
        }

        ret
    }

    pub fn get_len(&self) -> usize { self.weapons.len() + self.items.len() }
    pub fn get_items(&self) -> &Vec<Item> { &self.items }
    pub fn get_weapons(&self) -> &Vec<Weapon> { &self.weapons }
    pub fn is_full(&self) -> bool { self.curr_items == self.max_items }
}

pub struct Item
{
    name: String,
    description: String,
    used: bool
}

impl Item 
{
    pub fn new(name: String, description: String, used: bool) -> Self
    {
        Item {
            name: name,
            description: description,
            used: used
        }
    }

    pub fn clone(item: &Item) -> Self
    {
        Item {
            name: item.get_name(),
            description: item.get_description(),
            used: item.get_status()
        }
    }

    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_description(&self) -> String { self.description.clone() }
    pub fn get_status(&self) -> bool { self.used }

    pub fn set_status(&mut self, new_status: bool) { self.used = new_status}
}
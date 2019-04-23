enum NPCType {MALE, FEMALE, OTHER}

pub struct NPC
{
    _type: NPCType,
    name: String,
    // items to give?
}
/*
    npc's should eventually have unique items to give based on level? 
*/

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


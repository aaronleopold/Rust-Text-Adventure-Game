enum NPCType {MALE, FEMALE, OTHER}

#[derive (Clone)]
pub struct NPC
{
    //_type: NPCType,
    name: String
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
            //_type: NPCType::OTHER,
            name: String::from("temporary npc name"),
        }
    }

    #[inline]
    pub fn clone(npc: &NPC) -> Self 
    {
        NPC {
            //_type: npc._type,
            name: npc.get_name()
        }
    }

    pub fn get_name(&self) -> String { self.name.clone() }
    //pub fn get_type(&self) -> NPCType { self._type }
}


pub struct NPC
{
    name: String,
    dialogue: String,
    nice: bool,
    name_response: String,
    description: String
}

/*
    npc's should eventually have unique items to give based on level? 
*/

impl NPC
{
    pub fn new(name: String, dialogue: String, nice: bool, name_response: String, description: String) -> Self
    {
        NPC {
            // randomize typing runtime?
            //_type: NPCType::OTHER,
            name: name,
            dialogue: dialogue,
            nice: nice,
            name_response: name_response,
            description: description
        }
    }

    #[inline]
    pub fn clone(npc: &NPC) -> Self 
    {
        NPC {
            //_type: npc._type,
            name: npc.get_name(),
            dialogue: npc.get_dialogue(),
            name_response: npc.get_name_response(),
            nice: npc.is_nice(),
            description: npc.get_description()
        }
    }

    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_dialogue(&self) -> String { self.dialogue.clone() }
    pub fn is_nice(&self) -> bool { self.nice }
    pub fn get_name_response(&self) -> String { self.name_response.clone() }
    pub fn get_description(&self) -> String { self.description.clone() }
}


#[derive (Clone)]
pub struct NPC
{
    //_type: NPCType,
    name: String,
    dialogue: String
}

/*
    npc's should eventually have unique items to give based on level? 
*/

impl NPC
{
    pub fn new(name: String, dialogue: String) -> Self
    {
        NPC {
            // randomize typing runtime?
            //_type: NPCType::OTHER,
            name: name,
            dialogue: dialogue
        }
    }

    #[inline]
    pub fn clone(npc: &NPC) -> Self 
    {
        NPC {
            //_type: npc._type,
            name: npc.get_name(),
            dialogue: npc.get_dialogue()
        }
    }

    /*
    pub fn reset_dialogue(&mut self)
    {

    }
    */

    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_dialogue(&self) -> String { self.dialogue.clone() }
    //pub fn get_type(&self) -> NPCType { self._type }
}


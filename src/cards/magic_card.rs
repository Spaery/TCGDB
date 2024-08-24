use super::Card;

pub struct MagicCard {
    pub name: String,
    pub id: i64,
    pub edition: String,
    pub condition: String,
    pub foil: bool,
}

impl MagicCard {
    pub fn new(name: &str, id: i64, edition: &str, condition: &str) -> MagicCard {
        MagicCard {
            name: name.to_string(),
            id,
            edition: edition.to_string(),
            condition: condition.to_string(),
            foil: false,
        }
    }
}

// Implement the Card trait for MagicCard
impl Card for MagicCard {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_set(&self) -> &str {
        &self.edition
    }
    
    fn get_foil(&self) -> bool {
        self.foil
    }
    
    fn get_condition(&self) -> &str {
        &self.condition
    }
}
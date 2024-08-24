// Module for the cards in the game
pub mod magic_card;
// Define the Card trait
pub trait Card {
    // Get the name of the card
    fn get_name(&self) -> &str;
    // Get the Collector ID of the card
    fn get_id(&self) -> i64;
    // Get the set of the card
    fn get_set(&self) -> &str;
    // Get whether the card is foil
    fn get_foil(&self) -> bool;
    // Get the condition of the card
    fn get_condition(&self) -> &str;
}
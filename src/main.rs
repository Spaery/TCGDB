mod mtg; // Module for Magic: The Gathering specific constants and functions
mod cards; // Module for card-related structures and functions
mod tables; // Module for table-related structures and functions

use rusqlite::{Connection, Result}; // Importing rusqlite for SQLite database operations
use cards::{Card, magic_card::MagicCard as MagicCard}; // Importing Card trait and MagicCard struct
use tables::{Table, magic_table::MTGTable as MTGTable}; // Importing Table trait and MTGTable struct
fn main() -> Result<()> {
    // Establish a connection to the SQLite database
    let conn = Connection::open("tcg.db")?;
    // Create the MTG table
    let mtgtable = MTGTable::new(mtg::MTG_TABLE_NAME, mtg::MTG_TABLE_FIELDS, mtg::MTG_MATCH_FIELDS);

    create_table(&conn, &mtgtable);

    // Create a card
    let card = MagicCard::new("Black Lotus", 1, "Alpha", "LP");

    // Insert the card into the database
    increment_card(&conn, &mtgtable, &card);
    update_prices(&conn, &mtgtable, &card, 12.00);
    Ok(())
}

// Function to create a table in the database
fn create_table<T: Table>(connection: &Connection, table: &T) {
    let query = format!("CREATE TABLE IF NOT EXISTS {} ({})", table.get_table_name(), table.get_fields());
    match connection.execute(&query, ()) {
        Ok(_result) => println!("Table created."),
        Err(err) => println!("Error creating table! {}", err),
    }
}

// Function to check the quantity of a specific card in the table
fn check_card_quantity<T: Table, C: Card>(connection: &Connection, table: &T, card: &C) -> i64 {
    let sql_query = format!("SELECT quantity FROM {} WHERE {}", table.get_table_name(), table.get_match_fields());
    match connection.query_row(&sql_query, (card.get_set(),card.get_id(),card.get_foil(),card.get_condition()), |row| row.get::<usize, i64>(0)) {
        Ok(result) => {
            return result
        },
        Err(err) =>  {
            return 0;
        }
    }
}

// Function to increment the quantity of a specific card in the table
fn increment_card<T: Table, C: Card>(connection: &Connection, table: &T, card: &C) {
    let exists = check_card_quantity(connection, table, card);
    if exists > 0 {
        let sql_query = format! ("UPDATE {} SET quantity = quantity + 1 WHERE {}", table.get_table_name(), table.get_match_fields());
        match connection.execute(&sql_query, (card.get_set(), card.get_id(), card.get_foil(), card.get_condition())){
            Ok(rows_updated) => println!("Rows updated: {}", rows_updated),
            Err(err) => println!("Update table error!: {}", err)
        }; 
    } else {
        create_in_table(connection, table, card);
    }
}

// Function to decrement the quantity of a card in the table
fn decrement_card<T: Table, C: Card>(connection: &Connection, table: &T, card: &C) {
    let exists = check_card_quantity(connection, table, card);
    println!("{}",exists);
    if exists > 1 {
        let sql_query = format!("UPDATE {} SET quantity = quantity - 1 WHERE {}", table.get_table_name(), table.get_match_fields());
        match connection.execute(&sql_query, (card.get_set(), card.get_id(), card.get_foil(), card.get_condition())){
            Ok(result) => println!("Update successful, {} lines updated.", result),
            Err(err) => println!("Update failed: {}", err)
        }
    } else if exists <= 1 {
        let sql_query = format!("DELETE FROM {} WHERE {}", table.get_table_name(), table.get_match_fields());
        match connection.execute(&sql_query, (card.get_set(), card.get_id(), card.get_foil(), card.get_condition())){
            Ok(result) => println!("Deleted blank entry {} lines updated", result),
            Err(_) => todo!(),
        }
    } else {
        println!("Entry does not exist, no lines updated.");
    }
}

// Function to create a new card entry in the table
fn create_in_table<T: Table, C: Card>(connection: &Connection, table: &T, card: &C) {
    let sql_query = format!("INSERT INTO {} VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", table.get_table_name());
    match connection.execute(&sql_query,( card.get_set(),card.get_id(), card.get_name(), card.get_foil(), 1, card.get_condition(), 0, 0)) {
        Ok(_result) => println!("Created card entry in table"),
        Err(err) => println!("Entry creation error: {}",err)
    };
}

// Function to update the prices of a card in the table
fn update_prices<T: Table, C: Card>(connection: &Connection, table: &T, card: &C, tcgmarket: f64) {
    let sql_query = format!("UPDATE {} SET tcgmarket = ?5, instore = ?6 WHERE {}", table.get_table_name(), table.get_match_fields());
    let condition = card.get_condition();
    let markup = get_markup(condition);
    let instore = tcgmarket * markup;
    match connection.execute(&sql_query, (card.get_set(), card.get_id(), card.get_foil(), card.get_condition(),tcgmarket, instore)) {
        Ok(result) => println!("Updated prices for card: {}", result),
        Err(err) => println!("Error updating prices: {}", err)
    }
}

// Function to get the markup value based on the card's condition
fn get_markup(condition: &str) -> f64 {
    match condition {
        "NM" => mtg::NM_CARD_MARKUP,
        "LP" => mtg::LP_CARD_MARKUP,
        "MP" => mtg::MP_CARD_MARKUP,
        "HP" => mtg::HP_CARD_MARKUP,
        "DAMAGED" => mtg::DAMAGED_CARD_MARKUP,
        _ => mtg::DEFAULT_CARD_MARKUP, // Default markup for unknown conditions
    }
}
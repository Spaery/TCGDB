mod mtg;
mod cards;
mod tables;

use rusqlite::{Connection, Result};
use cards::{Card, magic_card::MagicCard as MagicCard};
use tables::{Table, magic_table::MTGTable as MTGTable};
fn main() -> Result<()> {
    let conn = Connection::open("tcg.db")?;
    // Create the MTG table
    let mut mtgtable = MTGTable::new(mtg::MTG_TABLE_NAME, mtg::MTG_TABLE_FIELDS, mtg::MTG_MATCH_FIELDS);

    create_table(&conn, &mtgtable);

    // Create a card
    let mut card = MagicCard::new("Black Lotus", 1, "Alpha", "LP");

    // Insert the card into the database
    decrement_card(&conn, &mtgtable, &card);
    Ok(())
}

fn create_table<T: Table>(connection: &Connection, table: &T) {
    let query = format!("CREATE TABLE IF NOT EXISTS {} ({})", table.get_table_name(), table.get_fields());
    match connection.execute(&query, ()) {
        Ok(_result) => println!("Table created."),
        Err(err) => println!("Error creating table! {}", err),
    }
}


fn check_card_quantity<T:Table, C: Card>(connection: &Connection, table: &T, card: &C) -> i64 {
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

fn increment_card<T:Table, C: Card>(connection: &Connection, table: &T, card: &C) {
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

fn decrement_card<T:Table, C: Card>(connection: &Connection, table: &T, card: &C) {
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

fn create_in_table<T:Table, C: Card>(connection: &Connection, table: &T, card: &C) {
    let sql_query = format!("INSERT INTO {} VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", table.get_table_name());
    match connection.execute(&sql_query,( card.get_set(),card.get_id(), card.get_name(), card.get_foil(), 1, card.get_condition(), 0, 0)) {
        Ok(_result) => println!("Created card entry in table"),
        Err(err) => println!("Entry creation error: {}",err)
    };
}
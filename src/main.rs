use core::num;

use rusqlite::{Connection,Result};
#[derive(Debug)]
struct Card {
    id: u32,
    name: String,
    foil: bool,
}

fn main() -> Result<()> {
    let conn = Connection::open("test.db")?;
    create_table(&conn, "dissension".to_string());
    let co = Card {
        id: 107,
        name: "Coiling Oracle".to_string(),
        foil: true,
    };
    // create_in_db(conn, "dissension".to_string(), co)?;
    update_db(&conn, "dissension".to_string(), co);
    Ok(())
}

fn create_in_db(connection: &Connection, table: String, card: Card) -> Result<()> {
    connection.execute("INSERT INTO ?1 VALUES (?2, ?3, ?4)",( table, card.id, card.name, card.foil))?;
    Ok(())
}

fn delete_from_db(connection: &Connection, card: Card) -> Result<()>{

    Ok(())
}

fn update_db(connection: &Connection,table: String, card: Card){
    match connection.execute("SELECT 1 FROM ?1 WHERE name = ?2", (table, card.name)) {
        Ok(num_of_card) => println!("{} selected", num_of_card),
        Err(error) => println!("Update database error! {}", error)
    };
}

fn create_table(connection: &Connection, table: String) {
    match connection.execute("CREATE TABLE ?1 (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            foil    BLOB
        )", [table]) {
            Ok(updated) => println!("{} updated", updated),
            Err(err) => println!("Create table error! {}", err),
        }
}
use rusqlite::{Connection, Result};
#[derive(Debug)]
struct Card {
    edition: String,
    id: i64,
    name: String,
    foil: bool,
}

fn main() -> Result<()> {
    let conn = Connection::open("tcg.db")?;
    create_tables(&conn);
    let card = Card {
        edition: "Dissension".to_string(),
        id: 107,
        name: "Coiling Oracle".to_string(),
        foil: false,
    };
    //increment_card(&conn, card);
    decrement_card(&conn, card);
    Ok(())
}

fn create_tables(connection: &Connection) {
    match connection.execute("CREATE TABLE IF NOT EXISTS MTG (
            edition     TEXT NOT NULL,
            id          INTEGER NOT NULL,
            name        TEXT NOT NULL,
            foil        BLOB NOT NULL,
            quantity    INTEGER
        )", ()) {
            Ok(_result) => println!("Table created."),
            Err(err) => println!("Error creating table! {}", err),
        }
}

fn increment_card(connection: &Connection, card: Card){
    let exists = check_card_quantity(connection, &card);
    if exists > 0{
        match connection.execute("UPDATE MTG SET quantity = quantity + 1 WHERE edition = ?1 AND id = ?2 AND foil = ?3", (card.edition, card.id, card.foil)){
            Ok(rows_updated) => println!("Rows updated: {}", rows_updated),
            Err(err) => println!("Update table error!: {}", err)
        }; 
    } else {
        create_in_table(connection, card);
    }
}

fn create_in_table(connection: &Connection, card: Card) {
    match connection.execute("INSERT INTO MTG VALUES (?1, ?2, ?3, ?4, ?5)",( card.edition, card.id, card.name, card.foil, 1)) {
        Ok(_result) => println!("Created card entry in table"),
        Err(err) => println!("Entry creation error: {}",err)
    };
}

fn check_card_quantity(connection: &Connection, card: &Card) -> i64 {
    match connection.query_row("SELECT quantity FROM MTG WHERE (edition = ?1 AND name = ?2 AND foil = ?3 AND id = ?4)", (&card.edition, &card.name, &card.foil, &card.id), |row| row.get::<usize, i64>(0)) {
        Ok(result) => {
            return result
        },
        Err(err) =>  {
            return 0;
        }
    }
}

fn decrement_card(connection: &Connection, card: Card) {
    let exists = check_card_quantity(connection, &card);
    println!("{}",exists);
    if exists > 1 {
        match connection.execute("UPDATE MTG SET quantity = quantity - 1 WHERE edition = ?1 AND id = ?2 AND foil = ?3", (card.edition, card.id, card.foil)){
            Ok(result) => println!("Update successful, {} lines updated.", result),
            Err(err) => println!("Update failed: {}", err)
        }
    } else if exists <= 1 {
        match connection.execute("DELETE FROM MTG WHERE edition = ?1 AND id = ?2 AND foil = ?3", (card.edition, card.id, card.foil)){
            Ok(result) => println!("Deleted blank entry {} lines updated", result),
            Err(_) => todo!(),
        }
    } else {
        println!("Entry does not exist, no lines updated.");
    }
}
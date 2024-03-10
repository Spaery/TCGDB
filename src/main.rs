use rusqlite::{Connection, Result};
#[derive(Debug)]
struct Card {
    edition: String,
    id: i64,
    name: String,
    foil: bool,
}

fn main() -> Result<()> {
    let conn = Connection::open("test.db")?;
    create_tables(&conn);
    let co = Card {
        edition: "Dissension".to_string(),
        id: 107,
        name: "Coiling Oracle".to_string(),
        foil: true,
    };
    update_table(&conn, co);
    Ok(())
}

fn create_tables(connection: &Connection) {
    match connection.execute("CREATE TABLE IF NOT EXISTS MTG (
            edition     TEXT NOT NULL,
            id      INTEGER NOT NULL,
            name    TEXT NOT NULL,
            foil    BLOB NOT NULL,
            quantity INTEGER
        )", ()) {
            Ok(_result) => println!("Table created."),
            Err(err) => println!("Error creating table! {}", err),
        }
}

fn update_table(connection: &Connection, card: Card){
    let temp_edition = &card.edition;
    let temp_name = &card.name;
    let temp_foil = &card.foil;
    let temp_id = &card.id;
    match connection.query_row("SELECT quantity FROM MTG WHERE EXISTS (SELECT id FROM MTG WHERE edition = ?1 AND name = ?2 AND foil = ?3 AND id = ?4)", (temp_edition, temp_name, temp_foil, temp_id), |row| row.get::<usize, i64>(0)) {
        Ok(num_of_card) => { println!("Card found, updating amount!");
            match connection.execute("UPDATE MTG SET quantity = ?1 WHERE edition = ?2 AND id = ?3 AND foil = ?4", (num_of_card + 1, card.edition, card.id, card.foil)){
                Ok(rows_updated) => println!("Rows updated: {}", rows_updated),
                Err(err) => println!("Update table error!: {}", err)
            }; 
        }
        Err(_err) => {
            create_in_table(connection, card);
        }
    };
}

fn create_in_table(connection: &Connection, card: Card) {
    match connection.execute("INSERT INTO MTG VALUES (?1, ?2, ?3, ?4, ?5)",( card.edition, card.id, card.name, card.foil, 1)) {
        Ok(_result) => println!("Created card entry in table"),
        Err(err) => println!("Entry creation error: {}",err)
    };
}

fn delete_from_db(connection: &Connection, card: Card) -> Result<()>{

    Ok(())
}
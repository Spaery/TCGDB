use rusqlite::{Connection, Result};
use csv;
mod mtg;
pub enum Card {
    MTGCard {
        edition: String,
        id: i64,
        name: String,
        foil: bool,
        quantity: i64,
        condition: String,
        tcgmarket: f64,
        instore: f64,
    },
}
impl Table {
    fn get_table_name(&self) -> &str {
        match self {
            Table::MTGTable { table_name, fields, match_fields } => table_name,
        }
    }
    fn get_fields(&self) -> &str {
        match self {
            Table::MTGTable { table_name, fields, match_fields } => fields,
        }
    }
    fn get_match_fields(&self) -> &str {
        match self {
            Table::MTGTable { table_name, fields, match_fields } => match_fields,
        }
    }
}

impl Card {
    fn get_edition(&self) -> &str {
        match self {
            Card::MTGCard { edition, id, name, foil, quantity, condition, tcgmarket, instore } => edition,
        }
    }
    fn get_id(&self) -> i64 {
        match self {
            Card::MTGCard { edition, id, name, foil, quantity, condition, tcgmarket, instore } => *id,
        }
    }
    fn get_name(&self) -> &str {
        match self {
            Card::MTGCard { edition, id, name, foil, quantity, condition, tcgmarket, instore } => name,
        }
    }
    fn get_foil(&self) -> bool {
        match self {
            Card::MTGCard { edition, id, name, foil, quantity, condition, tcgmarket, instore } => *foil,
        }
    }
    fn get_quantity(&self) -> i64 {
        match self {
            Card::MTGCard { edition, id, name, foil, quantity, condition, tcgmarket, instore } => *quantity,
        }
    }
    fn get_condition(&self) -> &str {
        match self {
            Card::MTGCard { edition, id, name, foil, quantity, condition, tcgmarket, instore } => condition,
        }
    }
    fn get_tcgmarket(&self) -> f64 {
        match self {
            Card::MTGCard { edition, id, name, foil, quantity, condition, tcgmarket, instore } => *tcgmarket,
        }
    }
    fn get_instore(&self) -> f64 {
        match self {
            Card::MTGCard { edition, id, name, foil, quantity, condition, tcgmarket, instore } => *instore,
        }
    }
}
pub enum Table {
    MTGTable {
        table_name: &'static str,
        fields: &'static str,
        match_fields: &'static str,
    },
}


fn main() -> Result<()> {
    let conn = Connection::open("tcg.db")?;
    // Create the MTG table
    let mtgtable = Table::MTGTable {
        table_name: mtg::MTG_TABLE_NAME,
        fields: mtg::MTG_TABLE_FIELDS,
        match_fields: mtg::MTG_MATCH_FIELDS,
    };

    create_table(&conn, mtgtable);

    Ok(())
}

fn create_table(connection: &Connection, table: Table) {
    let query = format!("CREATE TABLE IF NOT EXISTS {} ({})", table.get_table_name(), table.get_fields());
    match connection.execute(&query, ()) {
        Ok(_result) => println!("Table created."),
        Err(err) => println!("Error creating table! {}", err),
    }
}


fn check_card_quantity(connection: &Connection, table: Table, card: Card) -> i64 {
    let sql_query = format!("SELECT quantity FROM {} WHERE {}", table.get_table_name(), table.get_match_fields());
    match connection.query_row(&sql_query, (card.get_edition(),card.get_id(),card.get_foil(),card.get_condition()), |row| row.get::<usize, i64>(0)) {
        Ok(result) => {
            return result
        },
        Err(err) =>  {
            return 0;
        }
    }
}

fn increment_card(connection: &Connection,table: Table, card:Card) {
    let exists = check_card_quantity(connection, table, card);
    if exists > 0 {
        let sql_query = format! ("UPDATE {} SET quantity = quantity + 1 WHERE {}", table.get_table_name(), table.get_match_fields());
        match connection.execute(&sql_query, (card.get_edition(), card.get_id(), card.get_foil(), card.get_condition())){
            Ok(rows_updated) => println!("Rows updated: {}", rows_updated),
            Err(err) => println!("Update table error!: {}", err)
        }; 
    } else {
        create_in_table(connection, table, card);
    }
}

fn decrement_card(connection: &Connection, table: Table, card: Card) {
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

fn create_in_table(connection: &Connection, table: Table, card: Card) {
    let sql_query = format!("INSERT INTO {} VALUES (?1, ?2, ?3, ?4)", table.get_table_name());
    match connection.execute("INSERT INTO {} VALUES (?1, ?2, ?3, ?4, ?5)",( card.edition, card.id, card.name, card.foil, 1)) {
        Ok(_result) => println!("Created card entry in table"),
        Err(err) => println!("Entry creation error: {}",err)
    };
}
//methods from main.rs that mention MTG table
use crate::{Card};

const DEFAULT_CARD_MARKUP: f64 = 0.0;
const NM_CARD_MARKUP: f64 = 0.0;
const LP_CARD_MARKUP: f64 = 0.0;
const MP_CARD_MARKUP: f64 = 0.0;
const HP_CARD_MARKUP: f64 = 0.0;
const DAMAGED_CARD_MARKUP: f64 = 0.0;

pub const MTG_TABLE_FIELDS: &str =
"edition     TEXT NOT NULL,
id          INTEGER NOT NULL,
name        TEXT NOT NULL,
foil        BLOB NOT NULL,
quantity    INTEGER,
condition   TEST NOT NULL,
tcgmarket   REAL NOT NULL,
instore     REAL NOT NULL";

pub const MTG_MATCH_FIELDS: &str = "edition = ?1 AND id = ?2 AND foil = ?3 AND condition = ?4";

pub const MTG_TABLE_NAME: &str = "MTG";

pub struct MTGTable {
    table_name: &'static str,
    fields: &'static str,
    match_fields: &'static str,
}

pub struct MTGCard {
    edition: String,
    id: i64,
    name: String,
    foil: bool,
    quantity: i64,
    condition: String,
    tcgmarket: f64,
    instore: f64,
}

pub fn build_card (edition: String, id: i64, name: String, foil: bool, quantity: i64, condition: String, tcgmarket: f64, instore: f64) -> MTGCard {
    MTGCard {
        edition: edition,
        id: id,
        name: name.to_string(),
        foil: foil,
        quantity: quantity,
        condition: condition,
        tcgmarket: tcgmarket,
        instore: instore,
    }
}




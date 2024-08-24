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
condition   TEXT NOT NULL,
tcgmarket   REAL NOT NULL,
instore     REAL NOT NULL";

pub const MTG_MATCH_FIELDS: &str = "edition = ?1 AND id = ?2 AND foil = ?3 AND condition = ?4";

pub const MTG_TABLE_NAME: &str = "MTG";


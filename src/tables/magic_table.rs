use super::Table;

pub struct MTGTable {
    table_name: String,
    fields: String,
    match_fields: String,
}

impl MTGTable {
    pub fn new(table_name: &str, fields: &str, match_fields: &str) -> MTGTable {
        MTGTable {
            table_name: table_name.to_string(),
            fields: fields.to_string(),
            match_fields: match_fields.to_string(),
        }
    }
}

impl Table for MTGTable {
    fn get_table_name(&self) -> &str {
        &self.table_name
    }

    fn get_fields(&self) -> &str {
        &self.fields
    }

    fn get_match_fields(&self) -> &str {
        &self.match_fields
    }
}
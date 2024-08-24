pub mod magic_table;

pub trait Table {
    fn get_table_name(&self) -> &str;
    fn get_fields(&self) -> &str;
    fn get_match_fields(&self) -> &str;
}
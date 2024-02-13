pub struct DbError {
    error: String,
}
impl DbError {
    pub fn new(error: diesel::result::Error) -> DbError {
        DbError {
            error: error.to_string(),
        }
    }
    pub fn get(&self) -> &String {
        &self.error
    }
}

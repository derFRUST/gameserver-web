pub struct Database;

impl Database {
    pub fn new() -> Database {
        Database {}
    }
}

impl juniper::Context for Database {}

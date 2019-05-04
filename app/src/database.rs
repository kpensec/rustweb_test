use std::collections::HashMap;

use model::User;

type DataType = HashMap<String, User>;

pub struct Database {
    pub users: DataType,
    pub next_id: i32,
}

impl Database {
    pub fn add_user(&mut self, name: &str) -> String {
        let id = self.next_id.to_string();
        self.next_id += 1;
        self.users.insert(id.to_string(), User::new(id.as_str(), name));
        id
    }

    pub fn new() -> Database {
        let mut result = Database {
            users: DataType::new(),
            next_id: 0
        };
        result.add_user("Toto");
        result.add_user("Tyty");
        result.add_user("Tutu");
        result
    }
}


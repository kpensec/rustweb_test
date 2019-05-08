use std::collections::HashMap;
use model::{UserCreationParam, UserCreationResult, User};

type DataType = HashMap<String, User>;

pub struct Database {
    pub users: DataType,
    pub next_id: i32,
}

impl Database {

    fn next_id(&mut self) -> String {
        let id = self.next_id;
        self.next_id += 1;
        id.to_string()
    }

    pub fn add_user(&mut self, param: UserCreationParam) -> UserCreationResult {
        let key = self.next_id();
        let new_user = User::new(key.as_str(), param.name.as_str());
        self.users.insert(key.to_string(), new_user);
        UserCreationResult::new(key.as_str())
    }

    pub fn new() -> Database {
        let mut result = Database {
            users: DataType::new(),
            next_id: 0
        };
        result.add_user(UserCreationParam::new("Toto"));
        result.add_user(UserCreationParam::new("Tyty"));
        result.add_user(UserCreationParam::new("Tutu"));
        result
    }
}


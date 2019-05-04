
#[derive(Clone, GraphQLObject)]
pub struct User {
    pub id: String,
    pub name: String,
    pub friends_id: Vec<String>
}

impl User {
    pub fn new(id: &str, name: &str) -> User {
        User {
            id: id.to_string(),
            name: name.to_string(),
            friends_id: vec![]
        }
    }
}


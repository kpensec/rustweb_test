
#[derive(GraphQLObject)]
pub struct UserCreationResult {
    pub id: String
}

impl UserCreationResult {
    pub fn new(id: &str) -> Self {
        UserCreationResult{id: id.to_string()}
    }
}

#[derive(GraphQLInputObject)]
pub struct UserCreationParam {
    pub name: String
}

impl UserCreationParam {
    pub fn new(name: &str) -> Self {
        UserCreationParam{name: name.to_string()}
    }
}

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


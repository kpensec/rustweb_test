/// Definition des objets graphql

use ctx::Ctx;
use juniper::{
    FieldResult,
    Value,
    FieldError,
    RootNode
};

use vagrant::{MachinesStatus, AlterMachineStateParam, AlterMachineStateResult};
use model::{UserCreationParam, UserCreationResult, User};
use vagrant;


pub struct Query;
pub struct Mutation;
pub type Schema = RootNode<'static, Query, Mutation>;

graphql_object!(Query: Ctx as "Query" |&self| {
    description: "Root Query object"
    field hello() -> &str { "Hello" }
    field userWithId(&ex, user_id: String) -> FieldResult<User> {
        let db = ex.context().read_db();
        match db.users.get(user_id.as_str()) {
            Some(user) => Ok(user.clone()),
            None => Err(FieldError::new("User not found", Value::null()))
        }
    }
    field userList(&ex) -> FieldResult<Vec<User>> {
        let db = ex.context().write_db();
        Ok(db.users.iter().map(|(_, data)| data.clone()).collect())
    }

    field userNo(&ex) -> FieldResult<i32> {
        Ok(ex.context().read_db().next_id)
    }
    field vmStatus() -> FieldResult<MachinesStatus> {
       Ok(vagrant::get_status())
    }
});


graphql_object!(Mutation: Ctx as "Mutation" |&self| {
    description: "Root mutation object"

    field addUser(&executor, params: UserCreationParam)
    -> FieldResult<UserCreationResult> {
        let ctx : &Ctx = executor.context();
        Ok(ctx.write_db().add_user(params))
    }

    field alterVm(&executor, params: AlterMachineStateParam)
    -> FieldResult<AlterMachineStateResult> {
        Ok(params.switch_state())
    }
});

pub fn build_schema() -> Schema {
    Schema::new(Query, Mutation)
}


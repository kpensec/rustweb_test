use std::process::{Command};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct MachineStatus {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub state: String,
    pub vagrantfile_path: String,
}

pub type MachinesStatus = Vec<MachineStatus>;

fn execute_cmd(cmd : &mut Command) -> String {
    const ERR_EXEC : &'static str = "ERROR during command execution";
    const ERR_CVT : &'static str = "ERROR during string conversion";
    match cmd.current_dir("..").output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(out) => out,
            Err(_) => ERR_CVT.to_string()
        },
        Err(_) => ERR_EXEC.to_string()
    }
}

pub fn get_status() -> MachinesStatus {

    let json_string = execute_cmd(Command::new("vagrant")
                                  .arg("global-status")
                                  .arg("--json")
    );
    serde_json::from_str::<MachinesStatus>(&json_string).unwrap()
}

pub fn power_on(machine_name: String) -> String {
    execute_cmd(Command::new("vagrant").arg("up").arg(machine_name))
}

pub fn power_off(machine_name: String) -> String {
    execute_cmd(Command::new("vagrant").arg("halt").arg(machine_name))
}

use std::process::{Command, Stdio};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct MachineBoxData {
    pub provider: String,
    pub version: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct MachineStatusExtraData {
    pub r#box: Option<MachineBoxData>
}
#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct MachineStatus {
    pub id: Option<String>,
    pub local_data_path: Option<String>,
    pub name: Option<String>,
    pub provider: Option<String>,
    pub state: Option<String>,
    pub vagrantfile_name: Option<String>,
    pub vagrantfile_path: Option<String>,
    pub updated_at: Option<String>,
    pub extra_data: Option<MachineStatusExtraData>
}

#[derive(Serialize, Deserialize, GraphQLEnum)]
pub enum MachineState {
    On,
    Off
}

#[derive(Serialize, Deserialize, GraphQLInputObject)]
pub struct AlterMachineStateParam {
    pub id: String,
    pub state: MachineState
}

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct AlterMachineStateResult {
    pub message: String
}

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct MachinesStatus {
    pub machines: Vec<MachineStatus>
}

// TODO move this out of there!!!!
fn execute_cmd(cmd : &mut Command, background : bool) -> String {
    const ERR_EXEC : &'static str = "ERROR during command execution";
    const ERR_CVT : &'static str = "ERROR during string conversion";
    let cmd_prepared = cmd.current_dir("..");
    if background {
        cmd_prepared.spawn().expect("{'msg': 'Failed to start command'");
        "{'msg': 'Command Started'}".to_string()
    } else {
        let output = cmd_prepared.stdout(Stdio::piped()).output();
        match output {
            Ok(output) => match String::from_utf8(output.stdout) {
                Ok(out) => out,
                Err(_) => ERR_CVT.to_string()
            },
            Err(_) => ERR_EXEC.to_string()
        }
    }
}

pub fn get_status() -> MachinesStatus {

    let json_string = execute_cmd(Command::new("vagrant")
                                  .arg("json-status"),
                                  false
    );
    serde_json::from_str::<MachinesStatus>(&json_string).unwrap()
}

pub fn power_on(machine_id: String) -> String {
    execute_cmd(Command::new("vagrant").arg("up").arg(machine_id), true)
}

pub fn power_off(machine_id: String) -> String {
    execute_cmd(Command::new("vagrant").arg("halt").arg(machine_id), true)
}

impl AlterMachineStateParam {
    pub fn switch_state(self) -> AlterMachineStateResult {
        AlterMachineStateResult{
            message: match self.state {
                MachineState::On => power_on(self.id),
                MachineState::Off => power_off(self.id)
            }
        }
    }
}

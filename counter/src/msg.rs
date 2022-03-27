use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub studentname: String,
    pub studentid: i32,
    pub universitites: Vec<String>,
    pub degrees: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    //NewStudentAcc { studentname: String, studentid: i32, universitites: Vec<String>, degrees: Vec<String> },
    AddUniversity { studentid: i32, university: String },
    RemoveUniversity { studentid: i32, university: String },
    AddDegree { studentid: i32, degree: String },
    RemoveDegree { studentid: i32, degree: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetStudentAcc { studentid: i32 },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetStudentAccResponse {
    pub studentname: String,
    pub studentid: i32,
    pub universitites: Vec<String>,
    pub degrees: Vec<String>,
}
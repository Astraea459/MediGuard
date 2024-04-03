// dapp/src/contract.rs

// use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage};
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};

// use cosmwasm_std::{
//     Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage, QueryResult,
//     to_binary, QuerierWrapper, QueryRequest,
// };

// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};


// #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
// pub struct HealthRecord {
//     pub patient: String,
//     pub medical_history: String,
//     pub test_results: String,
//     pub prescriptions: String,
// }

// #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
// pub struct AddHealthRecordMsg {
//     pub patient: String,
//     pub medical_history: String,
//     pub test_results: String,
//     pub prescriptions: String,
// }

// pub fn handle(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: AddHealthRecordMsg,
// ) -> StdResult<Response> {
//     let record = HealthRecord {
//         patient: msg.patient,
//         medical_history: msg.medical_history,
//         test_results: msg.test_results,
//         prescriptions: msg.prescriptions,
//     };

//     // Store the record in the contract's storage
//     store_health_record(deps.storage, &record)?;

//     Ok(Response::new())
// }

// pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
//     match msg {
//         QueryMsg::GetHealthRecords => {
//             let records: Vec<HealthRecord> = get_health_records(deps.storage)?;

//             // Serialize and return the records as binary
//             Ok(Binary::from(serde_json::to_string(&records)?.as_bytes()))
//         }
//     }
// }

// fn store_health_record(storage: &mut dyn Storage, record: &HealthRecord) -> StdResult<()> {
//     // Generate a key for the health record
//     let key = format!("health_record_{}", &record.patient);

//     // Serialize and store the health record
//     //cosmwasm_std::storage::set(storage, &key, &serde_json::to_vec(record)?);
//     cosmwasm_std::storage::set(storage, &key, &serde_json::to_vec(record).map_err(|e| e.into())?);


//     Ok(())
// }

// fn get_health_records(storage: &dyn Storage) -> StdResult<Vec<HealthRecord>> {
//     // Iterate over all stored health records and deserialize them
//     let records: Vec<HealthRecord> = cosmwasm_std::storage::range(storage, 0, u64::MAX)?;

//     Ok(records)
// }

// #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
// pub enum QueryMsg {
//     GetHealthRecords,
// }

// use cosmwasm_std::{
//     to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, QueryRequest,
//     StdError, StdResult, Storage,
// };
use cosmwasm_std::{to_json_binary, Api, Binary, Env, HandleResult, InitResult, Querier, QueryRequest, StdError, StdResult, Storage};
use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    // Add any necessary state variables here
}

impl State {
    fn new() -> Self {
        State {}
    }
}

// Define your custom messages here
#[derive(Debug, PartialEq, Eq, Clone)]
enum Message {
    // You can define different messages for upload and retrieve operations
    Upload { data: String },
    Retrieve {},
}

// Initialize the contract
pub fn init(_env: Env, _info: Message) -> InitResponse<State> {
    let state = State::new();
    InitResponse::new(state)
}

// Handle different messages
pub fn handle(env: Env, _ctx: Extern, state: State, msg: Message) -> StdResult<HandleResponse> {
    match msg {
        Message::Upload { data } => handle_upload(env, state, data),
        Message::Retrieve {} => handle_retrieve(env, state),
    }
}

// Custom function to handle the upload message
fn handle_upload(env: Env, mut state: State, data: String) -> StdResult<HandleResponse> {
    // You can implement logic here to store the health record data securely
    // For simplicity, let's just store it in the contract state
    // In a real-world scenario, consider using encryption and decentralized storage solutions

    // Example storage in contract state
    //storage(&mut env.storage).set(b"health_record", &data.into_bytes());
    storage(&mut env.storage).insert(b"health_record".to_vec(), data.into_bytes());


    Ok(HandleResponse::default())
}

// Custom function to handle the retrieve message
fn handle_retrieve(_env: Env, state: State) -> StdResult<HandleResponse> {
    // You can implement logic here to retrieve the health record data
    // For simplicity, let's just retrieve it from the contract state

    let health_record = storage_read(&state, b"health_record")?;
    let response = HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(Binary::from(health_record)),
    };

    Ok(response)
}

// Query function to retrieve the health record data
pub fn query(_env: Env, state: State, _querier: Querier, _request: QueryRequest) -> StdResult<Binary> {
    // You can implement more advanced querying logic here if needed
    let health_record = storage_read(&state, b"health_record")?;
    Ok(Binary::from(health_record))
}

// Helper function to read from contract storage
fn storage_read<T: serde::de::DeserializeOwned>(
    state: &State,
    key: &[u8],
) -> StdResult<T> {
    let data = state.storage.get(key).ok_or_else(|| StdError::generic_err("Key not found"))?;
    to_instance(data)
}

// Helper function to access contract storage

fn storage(storage: &Storage) -> HashMap<Vec<u8>, Vec<u8>>  {
    cosmwasm_std::storage::Map::new(storage, b"storage")
}




// Helper function to deserialize binary data
fn to_instance<T: serde::de::DeserializeOwned>(bin: &[u8]) -> StdResult<T> {
    serde_json::from_slice(bin).map_err(StdError::generic_err)
}

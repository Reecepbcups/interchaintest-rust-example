use localic_std::cosmwasm::CosmWasm;
use reqwest::blocking::Client;
use serde_json::json;

use localic_std::polling::*;
use localic_std::transactions::*;

const API_URL: &str = "http://127.0.0.1:8080";

// ICTEST_HOME=./interchaintest local-ic start juno --api-port 8080
fn main() {    
    let client = Client::new();
    poll_for_start(client.clone(), &API_URL, 150);

    let rb: ChainRequestBuilder = ChainRequestBuilder::new(API_URL.to_string(), "localjuno-1".to_string(), true);

    test_cosmwasm(&rb);
}

fn test_cosmwasm(rb: &ChainRequestBuilder) {
    let cw = CosmWasm::new(&rb);

    // sudo chmod -R +rwx artifacts/
    let file_path = get_contract_path().join("ictest_rust.wasm");    
    let code_id = cw.store_contract("acc0", file_path);    
    if code_id.is_err() {        
        panic!("code_id error: {:?}", code_id);
    }    

    let code_id = code_id.unwrap_or_default();
    if code_id == 0 {
        panic!("code_id is 0");
    }

    let msg = r#"{"count":0}"#;
    let res = cw.instantiate_contract(
        "acc0",
        code_id,
        msg,
        "my-label",
        Some("juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl"),
        "",
    );
    println!("res: {:?}", res);

    let contract = match res {
        Ok(contract) => contract,
        Err(err) => {
            println!("err: {}", err);
            return;
        }
    };

    let prev_res = cw.query_contract(&contract.address, r#"{"get_count":{}}"#);
    assert_eq!(prev_res, json!({"data":{"count":0}}));
    println!("prev_res: {}", prev_res);

    let data = cw.execute_contract(&contract.address, "acc0", r#"{"increment":{}}"#, "--gas=auto --gas-adjustment=2.0");
    println!("unwrap: {}", data.unwrap());

    let updated_res = cw.query_contract(&contract.address, r#"{"get_count":{}}"#);    
    assert_eq!(updated_res, json!({"data":{"count":1}}));
    println!("updated_res: {}", updated_res);
}

fn parent_dir() -> std::path::PathBuf {
    return std::env::current_dir().unwrap().parent().unwrap().to_path_buf();
}

// get_contract_path returns the artifacts dire from parent_dir
fn get_contract_path() -> std::path::PathBuf {
    let path = parent_dir().join("artifacts");    
    path
}
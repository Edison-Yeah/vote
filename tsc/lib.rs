use cdk::runtime;
use std::collections::BTreeMap;
use cdk::types::Address;
extern crate serde_json;



#[no_mangle]
pub fn init() {
    //
    let deps = runtime::make_dependencies();
    let input = deps.api.input();

    let acl_app_address = input.read_str().unwrap();
    let token_app_address = input.read_str().unwrap();
    let voting_app_address = input.read_str().unwrap();

    let mut map:BTreeMao<String, String>;
    map.insert("acl_app".to_string(), acl_app_address.to_string());
    map.insert("token_app".to_string(), token_app_address.to_string());
    map.insert("voting_app".to_string(), voting_app_address.to_string());
}

fn store_apps(map:BTreeMap<String, String>) {
    //
    for (key, value) in map.iter() {
        //
        set_param(&key, value.as_bytes());
    }
}

fn get_apps() {
    //
}

fn get_app_address(app_name: String) -> String {
    //
}

fn query_balances() {
    //
    let app_name = "token".to_string();
    let addr = get_app_address(app_name);
    let app_address = Address::from(addr);
    let deps = runtime::make_dependencies();
    let mut sink = Sink::new(0);
    sink.write_str("query_balances");
    let input = sink.into();
    match deps.api.call_contract(&app_address, &input) {
        Some() => {
            //
        },
        None => {
            return_contract(Err("call contract error"));
        }
    }
}

fn query_votes() {
    //
    let app_name = "voting".to_string();
    let addr = get_app_address(app_name);
    let app_address = Address::from(addr);
    let deps = runtime::make_dependencies();
    let mut sink = Sink::new(0);
    sink.write_str("query_votes");
    let input = sink.into();
    match deps.api.call_contract(&app_address, &input) {
        Some() => {
            //
        },
        None => {
            return_contract(Err("call contract error"));
        }
    }
}



#[no_mangle]
pub fn invoke() {
    //
    let deps = runtime::make_dependencies();
    let input = deps.api.input();
    let method = input.read_str().unwrap();
    match method {
        "query_apps" => {
            //
            get_apps();
        }
        
        "query_votes" => {
            //
            query_votes();
        }

        "query_balances" => {
            //
            query_balances();
        }

        _ => {
            //
            // 返回Error
            return_contract(Err("invoke method not found"));
        }
    }
}


fn set_param(key: &str, value: &[u8]) {
    runtime::make_dependencies()
        .storage
        .set(key.as_bytes(), value)
}

fn get_param(key: &str) -> std::vec::Vec<u8> {
    let val = runtime::make_dependencies()
        .storage
        .get(key.as_bytes())
        .unwrap();
    return val
}
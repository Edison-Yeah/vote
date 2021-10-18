extern crate c123chain_cdk as cdk;
extern crate serde_json;

use cdk::types::{Address, Response};
use cdk::runtime;
use cdk::codec::{Sink, Source};

use std::collections::BTreeMap;

const KEY_ALL_APPS: &str = "all_apps";
const KEY_ACL_APP: &str = "acl_app";
const KEY_TOKEN_APP: &str = "token_app";
const KEY_VOTING_APP: &str = "voting_app";
#[no_mangle]
pub fn init() {
    //
    let deps = runtime::make_dependencies();
    let input = deps.api.input();

    let acl_app_address = input.read_str().unwrap();
    let token_app_address = input.read_str().unwrap();
    let voting_app_address = input.read_str().unwrap();

    let mut map:BTreeMap<String, String> = BTreeMap::new();
    map.insert("acl_app".to_string(), acl_app_address.to_string());
    map.insert("token_app".to_string(), token_app_address.to_string());
    map.insert("voting_app".to_string(), voting_app_address.to_string());
    //save apps;
    store_apps(map);
    
}

fn store_apps(map:BTreeMap<String, String>) {
    //
    let res = serde_json::to_vec(&map).unwrap();
    set_param(KEY_ALL_APPS, &res);
}

fn get_apps() -> Vec<u8> {
    //
    let apps = get_param(KEY_ALL_APPS);
    return apps;
}

fn get_app_address(app_name: String) -> String {
    //
    let app = get_apps();
    let app_map: BTreeMap<String, String> = serde_json::from_slice(&app).unwrap();
    let address = app_map.get(&app_name).unwrap();
    return address.to_string()
}

fn add_app(app_name:String, app_addr:String) {
    //
    let app = get_apps();
    let mut app_map: BTreeMap<String, String> = serde_json::from_slice(&app).unwrap();
    app_map.insert(app_name, app_addr);

    store_apps(app_map);

}

fn remove_app(app_name:String) {
    //
    let app = get_apps();
    let mut app_map: BTreeMap<String, String> = serde_json::from_slice(&app).unwrap();
    app_map.remove(&app_name);
    store_apps(app_map);
    
}

fn query_balances() {
    //
    let app_name = "token".to_string();
    let addr = get_app_address(app_name);
    let app_address = Address::from(&*addr);
    let deps = runtime::make_dependencies();
    let mut sink = Sink::new(0);
    sink.write_str("query_balances");
    let input = sink.into();
    match deps.api.call_contract(&app_address, &input) {
        Some(res) => {
            //
            return_contract(Ok(Response{ data: &res}));
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
    let app_address = Address::from(&*addr);
    let deps = runtime::make_dependencies();
    let mut sink = Sink::new(0);
    sink.write_str("query_votes");
    let input = sink.into();
    match deps.api.call_contract(&app_address, &input) {
        Some(res) => {
            //
            return_contract(Ok(Response{ data: &res}));
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
            let apps = get_apps();
            return_contract(Ok(Response{ data: &apps}));
        }
        
        "query_votes" => {
            //
            let s = input.read_str().unwrap();
            query_votes();
        }

        "query_balances" => {
            //
            query_balances();
        }

        "query_app" => {
            let name = input.read_str().unwrap();
            let address = get_app_address(name.to_string());
            return_contract(Ok(Response{ data: &address.to_string().as_bytes()}));
        }

        "add_app" => {
            //
            let app_name = input.read_str().unwrap();
            let address = input.read_str().unwrap();
            add_app(app_name.to_string(), address.to_string());
        }

        "remove_app" => {
            //
            let app_name = input.read_str().unwrap();
            remove_app(app_name.to_string());
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

fn return_contract<'a>(result: Result<Response, &'a str>) {
    runtime::make_dependencies().api.ret(result)
}
extern crate c123chain_cdk as cdk;

//use cdk::types::Address;
//use cdk::types::Response;
mod types;
mod store;
use types::{MiniMeToken, Vote};
use cdk::types::Address;
use cdk::runtime;




#[no_mangle]
pub fn init() {
    //get params
    //let deps = runtime::make_dependencies();
    //let input = deps.api.input();

    //let _token = input.read_str().unwrap();
    //let _supportRequiredPct = input.read_u64().unwrap();
    //let _minAcceptQuorumPct = input.read_u64().unwrap();
    //let _voteTime = input.read_u64().unwrap();
    let _token = "0x3422482938473294324238204824323327492323";
    let _support_required_pct = 1;
    let _min_accept_quorum_pct = 2;
    let _vote_time = 100;

    initialize(_token, _support_required_pct, _min_accept_quorum_pct, _vote_time)

    //initialize state
    //return_contract(Ok(Response {
        //data: "Success".as_bytes(),
    //}));
}


#[no_mangle]
pub fn invoke() -> String {
    let deps = runtime::make_dependencies();
    let input = deps.api.input();
    let method = input.read_str().unwrap();
   //let method = "get_token_address";
    match method {
        "get_token_address" => {
            //return_contract(Ok(Response {
                //data: get_param(key_token).as_bytes(),
            //}));
            let address = store::get_param(store::KEY_TOKEN);
            return String::from_utf8(address).unwrap()
        }
        "get_balance" => {
            //let s: &str = "0x3422482938473294324238204824323327492323";
            //let token: MiniMeToken = MiniMeToken::new(s);
            let balance = store::get_param(store::KEY_BALANCE);
            //return String::from_utf8(balance).unwrap()
            return "success".to_string()
        }

        //create vote
        "new_vote" => {
            //
            let mut vote = Vote::new();
            vote.new_vote( true, true,  10).to_string()
        }

        //cast vote
        "cast_vote" => {
            //
            let addr = "0x3422482938473294324238204824323327492323";
            let vote_id = 1;
            let supports =  false;
            let executes_if_decided = false;
            let voter= Address::from(addr);
            Vote::cast_vote(vote_id, supports, executes_if_decided, voter);
            return "success".to_string()
        }
         _ => {
            // 返回Error
            //return_contract(Err("invoke method not found"));
            return "error".to_string()
        }
    }
}


fn initialize(_token: &str, _support_required_pct: u64, _min_accept_quorum_pct:u64, _vote_time: u64) {

    let s: &str = "0x3422482938473294324238204824323327492323";
    let token: MiniMeToken = MiniMeToken::new(s);
    //let token: Address = Address::from(s);
    let b: u64 = token.balance_of_at();
    //let bal: &str = &b.to_string();
    //
    //set params
    let a = &_support_required_pct.to_be_bytes();
    store::set_param(store::KEY_TOKEN, _token.as_bytes());
    store::set_param(store::KEY_SUPPORT_REQUIRED_PCT, a);
    store::set_param(store::KEY_MIN_ACCEPT_QUORUM_PCT, &(_min_accept_quorum_pct.to_be_bytes()));
    store::set_param(store::KEY_VOTE_TIME, &(_vote_time.to_be_bytes()));
    store::set_param(store::KEY_BALANCE, &(b.to_be_bytes()));

    //let mut vote = Vote::new();
    //vote.new_vote( true, true,  10).to_string();
}



//fn return_contract<'a>(result: Result<Response, &'a str>) {
    //runtime::make_dependencies().api.ret(result)
//}


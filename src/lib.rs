/*
extern crate c123chain_cdk as cdk;
extern crate serde_json;

use cdk::types::{Address, Response};
use cdk::runtime;
use cdk::codec::{Sink, Source};
use cdk::util::clone_into_array;
use std::collections::BTreeMap;
use Clone;



#[no_mangle]
pub fn init() {
    //get params
    let deps = runtime::make_dependencies();
    let input = deps.api.input();

    let _token = input.read_str().unwrap();
    let _support_required_pct = input.read_u64().unwrap();
    let _min_accept_quorum_pct = input.read_u64().unwrap();
    let _vote_time = input.read_u64().unwrap();

    
    initialize(_token, _support_required_pct, _min_accept_quorum_pct, _vote_time);
    return_contract(Ok(Response { data: &("init complete".to_string()).as_bytes()}));
    //return_contract(Ok(Response { data: &(_vote_time.to_string()).as_bytes()}));

}


#[no_mangle]
pub fn invoke() {
    let deps = runtime::make_dependencies();
    let input = deps.api.input();
    let method = input.read_str().unwrap();
    match method {
        //create vote
        "new_vote" => {
            //
            let addr = input.read_str().unwrap();
            let cast_vote = input.read_bool().unwrap();
            let executes_if_decided = input.read_bool().unwrap();
            //let time = input.read_u64().unwrap();
            let time = runtime::make_dependencies().api.get_timestamp();
            let mut vote = Vote::new();
            let mut voter = Address::from(addr);
            let vote_id = vote.new_vote(&mut voter, cast_vote, executes_if_decided,  time);
            let return_msg = ("vote_id = ".to_string() + &vote_id.to_string()).to_string();
            return_contract(Ok(Response { data: &(return_msg.as_bytes()) }));
            //return_contract(Ok(Response { data: &("invoke complete".to_string()).as_bytes()}));
            //return_contract(Ok(Response { data: &("start_date = ".to_string() + &time.to_string()).as_bytes()}));
        }

        //cast vote
        "cast_vote" => {
            let addr = input.read_str().unwrap();
            let vote_id = input.read_u64().unwrap();
            let supports = input.read_bool().unwrap();
            let executes_if_decided = input.read_bool().unwrap();
            let mut voter= Address::from(addr);
            Vote::cast_vote(vote_id, supports, executes_if_decided, &mut voter);
            return_contract(Ok(Response { data: &("cast vote success".to_string()).as_bytes()}));
        }

        "query_votes" => {
            //
            query_votes();
        }

        "query_vote" => {
            //
            let vote_id = input.read_u64().unwrap();
            query_vote(vote_id);
            //let vote = query_vote(vote_id);
            //return_contract(Ok(Response { data: &vote }));
        }

        "query_public_params" => {
            //
            query_public_params();
        }
         _ => {
            // 返回Error
            return_contract(Err("invoke method not found"));
        }
    }
}


fn initialize(_token: &str, _support_required_pct: u64, _min_accept_quorum_pct:u64, _vote_time: u64) {

    let vote_length: u64 = 0;
    //set params
    set_param(KEY_TOKEN, _token.as_bytes());
    set_param(KEY_SUPPORT_REQUIRED_PCT, &_support_required_pct.to_le_bytes());
    set_param(KEY_MIN_ACCEPT_QUORUM_PCT, &_min_accept_quorum_pct.to_le_bytes());
    set_param(KEY_VOTE_TIME, &_vote_time.to_le_bytes());

    set_param(KEY_VOTE_LENGTH, &(vote_length.to_le_bytes()));
    //return_contract(Ok(Response { data: &("vote_time = ".to_string() + &_vote_time.to_string()).as_bytes()}));

}

fn query_votes() {
    let length = get_param(KEY_VOTE_LENGTH);

    let vote_length = u64::from_le_bytes(clone_into_array(&length));
    
    //let all_res: Vec<String>;
    if vote_length == 0 {
        return_contract(Ok(Response { data: "no votes exists".to_string().as_bytes()}));
    }else {
        let mut res_map: BTreeMap<String, String> = BTreeMap::new();
        let mut i: u64 = 1;
        while i <= vote_length {
            //
            let id = i.to_string();
            let res = get_param(&id);

            //unMarshal.
            let vote = Vote::un_serde(res);
            //return_contract(Ok(Response {data: vote.sponsor.as_bytes()}));
            
            let res = store_in_map(vote);
            let res_str = String::from_utf8(res).unwrap();
            //
            let vote_key = "vote".to_string() + &i.to_string();
            res_map.insert(vote_key, res_str);
            //return_contract(Ok(Response{ data: &res}));
            //let map = store_in_map(vote);
            //let res = write_to_mem(map);
            //let res = vote.serde();
            //return_contract(Ok(Response { data: &res }));
            i = i + 1;
        }
        let all_res = serde_json::to_vec(&res_map).unwrap();
        return_contract(Ok(Response { data: &all_res }));

    }
}

fn store_in_map(vote:Vote) -> Vec<u8> {
    //
    
    let mut map:BTreeMap<String, String> = BTreeMap::new();
    map.insert("sponsor".to_string(), vote.sponsor.to_string());
    map.insert("has_executed".to_string(), vote.executed.to_string());
    map.insert("start_date".to_string(), vote.start_date.to_string());
    map.insert("snap_shot_block".to_string(), vote.snapshot_block.to_string());
    map.insert("support_required_pct".to_string(), vote.support_required_pct.to_string());
    map.insert("min_accept_quorum_pct".to_string(), vote.min_accept_quorum_pct.to_string());
    map.insert("yea".to_string(), vote.yea.to_string());
    map.insert("nay".to_string(), vote.nay.to_string());
    map.insert("vote_voting_power".to_string(), vote.voting_power.to_string());
    let res_voters = serde_json::to_vec(&vote.voters).unwrap();
    map.insert("voters".to_string(), String::from_utf8(res_voters).unwrap());


    let res_vote = serde_json::to_vec(&map).unwrap();
    //return_contract(Ok(Response{ data: &res_vote}));
    
    //let res_voters = serde_json::to_vec(&vote.voters).unwrap();
    //let vs = res_voters.clone();
    //return_contract(Ok(Response{ data: &vs}));

    return res_vote
}

fn query_vote(vote_id: u64) {
    //
    let id = vote_id.to_string();
    let res = get_param(&id);
    let vote = Vote::un_serde(res);

    //vote.voters.into()
    let res = store_in_map(vote);
    return_contract(Ok(Response{ data: &res}));
    //let map = store_in_map(vote);
    //return write_to_mem(map)

    
}

fn query_public_params() {
    //
    let support_required_pct = get_param(KEY_SUPPORT_REQUIRED_PCT);
    let min_accept_quorum_pct = get_param(KEY_MIN_ACCEPT_QUORUM_PCT);
    let vote_open_time = get_param(KEY_VOTE_TIME);
    
    let _support_required_pct = clone_into_array(&support_required_pct);
    let _min_accept_quorum_pct = clone_into_array(&min_accept_quorum_pct);
    let _vote_open_time = clone_into_array(&vote_open_time);

    let mut map:BTreeMap<String, String> = BTreeMap::new();
    map.insert(KEY_VOTE_TIME.to_string(), (u64::from_le_bytes(_vote_open_time)).to_string());
    map.insert(KEY_MIN_ACCEPT_QUORUM_PCT.to_string(), (u64::from_le_bytes(_min_accept_quorum_pct)).to_string());
    map.insert(KEY_SUPPORT_REQUIRED_PCT.to_string(), (u64::from_le_bytes(_support_required_pct)).to_string());

    let res = serde_json::to_vec(&map).unwrap();
    return_contract(Ok(Response{ data: &res}));
    //return write_to_mem(map);
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct MiniMeToken{
    address:Address
}

impl MiniMeToken {

    pub fn new(s: &str) -> MiniMeToken {
        
        return MiniMeToken{
            address: Address::from(s)
        }
    }

    fn balance_of_at(&self, voter: &mut Address, block_number: u64) -> u64 {
        let deps = runtime::make_dependencies();
        
        let mut sink = Sink::new(0);
        sink.write_str("token_account_balance");
        sink.write_str(&(voter.to_hex_string()));
        sink.write_u64(block_number);
        let input = sink.into();

        match deps.api.call_contract(&self.address, &input) {
                Some(res) => {
                    let balance = 1000000;//u64::from_le_bytes(clone_into_array(&res));
                    //return_contract(Ok(Response {data: &("balance = ".to_string() + &balance.to_string()).as_bytes()}));
                    return balance;
                },
                None => {
                    return_contract(Err("call contract error"));
                    return 0;
                },
        };
    }

    fn total_supply_at(&self, block_number: u64) -> u64 {
        let deps = runtime::make_dependencies();

        let mut sink = Sink::new(0);
        sink.write_str("token_total_supply");
        sink.write_u64(block_number);
        let input = sink.into();
        match deps.api.call_contract(&self.address, &input) {
                Some(res) => {
                    let total_supply = 100000000000;//u64::from_le_bytes(clone_into_array(&res));
                    //return_contract(Ok(Response {data: &("total_supply = ".to_string() + &total_supply.to_string()).as_bytes()}));
                    return total_supply;
                },
                None => {
                    return_contract(Err("call contract error"));
                    return 0;
                },
            };
    }
}

struct Voter {
    support: bool,
    stake: u64,
}

impl Voter {
    pub fn new() -> Voter {
        return Voter{
            support: true,
            stake: 0,
        }
    }

    fn serde(&self) -> Vec<u8> {
        let mut sink = Sink::new(0);
        sink.write_bool(self.support);
        sink.write_u64(self.stake);
        return sink.into()
    }

    fn un_serde(data: Vec<u8>) -> Voter {
        //
        let source = Source::new(data);
        let support = source.read_bool().unwrap();
        let stake = source.read_u64().unwrap();

        let mut voter = Voter::new();
        voter.support = support;
        voter.stake = stake;
        return voter;
    }
}

//vote
pub struct Vote {
    sponsor: String,  //Sponsor
    executed:bool,    //has been executed?
    start_date:u64,   //start time
    snapshot_block:u64,   //start block height;
    support_required_pct:u64,  //required supported percent;
    min_accept_quorum_pct:u64, //min accept quorum percent;
    yea:u64,      //supports
    nay:u64,      //opposite
    voting_power:u64,   //voting_power
    voters: BTreeMap<String, Vec<u8>>,   //voter[voter_address, vote_state] vote_state: -1(absent), 0(opposite), 1(support);
}
//vote fun
impl Vote {

    pub fn new() -> Vote {
        //default vote;
        return Vote{
            sponsor: "".to_string(),
            executed: false,
            start_date: 0,
            snapshot_block: 0,
            support_required_pct: 0,
            min_accept_quorum_pct: 0,
            yea: 0,
            nay: 0,
            voting_power: 0,
            voters:  BTreeMap::new(),
        };
    }

    fn set_params(&mut self) {
        //
        let support_required_pct_u8 = get_param(KEY_SUPPORT_REQUIRED_PCT);
        let min_accept_quorum_pct_u8 = get_param(KEY_MIN_ACCEPT_QUORUM_PCT);
        let support_required_pct = u64::from_le_bytes(clone_into_array(&support_required_pct_u8));
        let min_accept_quorum_pct = u64::from_le_bytes(clone_into_array(&min_accept_quorum_pct_u8));
        self.support_required_pct = support_required_pct;
        self.min_accept_quorum_pct = min_accept_quorum_pct;
        //return_contract(Ok(Response {data : self.support_required_pct.to_string().as_bytes()}));
    }

    //pub fn change_support_required_pct(&mut self, support_required_pct: u64) {
        //
    //}

    //pub fn change_min_accept_quorum_pct(&mut self, min_accept_quorum_pct: u64) {
        //
   //}

   //marshal
    fn serde(&self) -> Vec<u8> {
        //
        let mut sink = Sink::new(0);
        sink.write_str(&self.sponsor);
        sink.write_bool(self.executed);
        sink.write_u64(self.start_date);
        sink.write_u64(self.snapshot_block);
        sink.write_u64(self.support_required_pct);
        sink.write_u64(self.min_accept_quorum_pct);
        sink.write_u64(self.yea);
        sink.write_u64(self.nay);
        sink.write_u64(self.voting_power);

        let voters = serde_json::to_vec(&self.voters).unwrap();
        sink.write_bytes(&voters);
        //return_contract(Ok(Response {data: { &("support = ".to_string() + &self.support_required_pct.to_string()).as_bytes()}}));
        return sink.into();
    }

    //unMarshal
    fn un_serde(data: Vec<u8>) -> Vote {
        //
        let source = Source::new(data);
        let sponsor = source.read_str().unwrap();
        let executed = source.read_bool().unwrap();
        let start_date = source.read_u64().unwrap();
        let snapshot_block = source.read_u64().unwrap();
        let support_required_pct = source.read_u64().unwrap();
        let min_accept_quorum_pct = source.read_u64().unwrap();
        let yea = source.read_u64().unwrap();
        let nay = source.read_u64().unwrap();
        let voting_power = source.read_u64().unwrap();
        let voters = source.read_bytes().unwrap();
        //
        let mut vote = Vote::new();
        vote.sponsor = sponsor.to_string();
        vote.executed = executed;
        vote.start_date = start_date;
        vote.snapshot_block = snapshot_block;
        vote.min_accept_quorum_pct = min_accept_quorum_pct;
        vote.support_required_pct = support_required_pct;
        vote.yea = yea;
        vote.nay = nay;
        vote.voting_power = voting_power;
        vote.voters = serde_json::from_slice(voters).unwrap();
       // return_contract(Ok(Response {data: { &("sponsor = ".to_string() + &vote.sponsor).as_bytes()}}));
        //return_contract(Ok(Response {data: { &("support = ".to_string() + &vote.support_required_pct.to_string()).as_bytes()}}));
/*
        return_contract(Ok(Response {data: { &("start_data = ".to_string() + &start_date.to_string()).as_bytes()}}));
        return_contract(Ok(Response {data: { &("snapshot_block = ".to_string() + &snapshot_block.to_string()).as_bytes()}}));
        return_contract(Ok(Response {data: { &("min_accept_quorum_pct = ".to_string() + &min_accept_quorum_pct.to_string()).as_bytes()}}));
*/
        return vote
    }

    //return voteId
    pub fn new_vote(&mut self, voter:&mut Address, cast_vote: bool, executes_if_decided: bool, time: u64) -> u64 {
        //vote_id = length++;
        let vote_length = get_param(KEY_VOTE_LENGTH);
        let length = u64::from_le_bytes(clone_into_array(&vote_length));
        let vote_id = length + 1;
        //get chain block height;
        let block_number = get_block_number();

        let addr = get_param(KEY_TOKEN);
        let address = String::from_utf8(addr).unwrap();
        let token = MiniMeToken::new(&address);
        let voting_power = token.total_supply_at(block_number);

        if voting_power <= 0 {
            panic!("error, no voting power");
        }
        let sponsor = voter.to_hex_string();
        self.sponsor = sponsor;
        self.voting_power = voting_power;
        self.set_params();   //set default vote params;
        self.start_date = time;
        self.snapshot_block = block_number;

        if cast_vote && self.can_vote(voter) {
            //cast voe
            self.vote(voter, vote_id, true, executes_if_decided);
        }
        
        //save vote
        //vore_id -> vote
        let res = self.serde();
        let id = vote_id.to_string();
        set_param(&id, &res);
        //save vote length
        set_param(KEY_VOTE_LENGTH, &(vote_id.to_le_bytes()));
        
        return vote_id
    }

    pub fn cast_vote(vote_id: u64, supports: bool, executes_if_decided: bool, voter: &mut Address) {
        
        let id = vote_id.to_string();
        let res = get_param(&id);

        //unMarshal.
        let mut vote = Vote::un_serde(res);
        if !vote.can_vote(voter) {
            panic!("cannot vote")
        }
        vote.vote(voter, vote_id, supports, executes_if_decided);

        //save vote
        //vore_id -> vote
        let res = vote.serde();
        let id = vote_id.to_string();
        set_param(&id, &res);
        
    }

    fn can_vote(&self, voter: &mut Address) -> bool {
        //
        let token_addr = get_param(KEY_TOKEN);
        let address = String::from_utf8(token_addr).unwrap();
        let token = MiniMeToken::new(&address);

        let block_number = get_block_number() - 1;

        return self.is_vote_open() && token.balance_of_at(voter, block_number) > 0;
    }

    fn can_execute(&mut self, _vote_id: u64) -> bool {
        //
        if self.executed {
            //executed already.
            return false
        }

        //// Vote ended?
        if self.is_vote_open(){
            //open yet.
            return false
        }
        let total_votes = self.yea.wrapping_add(self.nay);
        
        // Has enough support?
        if !(self.is_value_pct(self.yea, total_votes, self.support_required_pct)) {
            //
            return false
        }

        // Has min quorum?
        if !(self.is_value_pct(self.yea, self.voting_power, self.min_accept_quorum_pct)) {
            return false
        }

        return true
    }

    //pub fn get_vote() {
        //
    //}

    //fn get_vote_state() -> vote_state {
        //
        //return vote_state::Absent
    //}

    //return true if the given vote is open, false otherwise
    fn is_vote_open(&self) -> bool {
        //
        let deps = runtime::make_dependencies();
        let time_stamp = deps.api.get_timestamp();
        let time = get_param(KEY_VOTE_TIME);
        let vote_time = u64::from_le_bytes(clone_into_array(&time));
        return time_stamp < self.start_date.wrapping_add(vote_time) && !self.executed;
    }

    //Calculates whether `_value` is more than a percentage `_pct` of `_total`
    fn is_value_pct(&self, value: u64, total: u64, pct: u64) -> bool {
        //
        if total == 0 || value == 0 {
            return false
        }
        let compucted_pct = value.wrapping_mul(PCT_BASE) / total;
        return compucted_pct > pct
    }

    fn vote(&mut self, voter: &mut Address, vote_id : u64, support: bool, executes_if_decided: bool) {
        //
        let address = voter.to_hex_string();
        //let state = self.voters.get(&address).unwrap();
        let mut voter_state = Voter::new();
        let v = self.voters.get(&address);
        match v {
            None => {
                //
            }
            Some(_) => {
                //
                let state = a.unwrap();
                let s = state.clone();

                voter_state = Voter::un_serde(s);
                let per_stake = voter_state.stake;
                let per_support = voter_state.support;

                match per_support {
                    true => {
                        //
                        self.yea = self.yea.wrapping_sub(per_stake);
                    }
                    false => {
                        self.nay = self.nay.wrapping_sub(per_stake);
                    }

                    _ => {
                        //
                    }
                };
            } 
        }
        
        let addr = get_param(KEY_TOKEN);
        let token_address = String::from_utf8(addr).unwrap();
        let token = MiniMeToken::new(&token_address);
        let voter_current_stake = token.balance_of_at(voter, self.snapshot_block);
        if support {
            //
            self.yea = self.yea.wrapping_add(voter_current_stake);
            voter_state.support = true;
        }else {
            //
            self.nay = self.nay.wrapping_add(voter_current_stake);
            voter_state.support = false;
        }
        voter_state.stake = voter_current_stake;
        //
        let result = voter_state.serde();
        self.voters.insert(address, result);

        if executes_if_decided && self.can_execute(vote_id) {
            //execute
            self.unsafe_execute_vote();
        }
    }

    fn unsafe_execute_vote(&mut self) {
        //execute vote
        self.executed = true;
    }
}

//token contract address;
const KEY_TOKEN: &str = "token";
//required support percent;
const KEY_SUPPORT_REQUIRED_PCT: &str = "support_required_pct";
//min accept quorum percent;
const KEY_MIN_ACCEPT_QUORUM_PCT: &str = "min_accept_quorum_pct";
//vote open time;
const KEY_VOTE_TIME: &str = "vote_open_time";
//quantity of vote;
const KEY_VOTE_LENGTH: &str = "vote_length";
// pct_base = 10^18; 100%;
const PCT_BASE: u64 = 1000000000000000000;


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



fn get_block_number() -> u64 {
    return 2;
    //return runtime::make_dependencies().api.get_block_number();
}



fn return_contract<'a>(result: Result<Response, &'a str>) {
    runtime::make_dependencies().api.ret(result)
}

*/



extern crate c123chain_cdk as cdk;
extern crate serde_json;

use cdk::types::{Address, Response, ADDR_SIZE};
use cdk::runtime;
use cdk::codec::{Sink, Source};

use std::collections::BTreeMap;

const KEY_ALL_APPS: &str = "all_apps";
const KEY_ACL_APP: &str = "acl_app";
const KEY_TOKEN_APP: &str = "token_app";
const KEY_VOTING_APP: &str = "voting_app";
const KEY_COMMUNITY_APP: &str = "community_app";
const ANY_ENTITY: &str = "0x0000000000000000000000000000000000000001";

const ACTION_MANAGE: &str = "community.manage";

#[no_mangle]
pub fn init() {
    //
    /*
    let deps = runtime::make_dependencies();
    let input = deps.api.input();

    let acl_app_address = input.read_str().unwrap();
    let token_app_address = input.read_str().unwrap();
    let voting_app_address = input.read_str().unwrap();

    let mut map:BTreeMap<String, String> = BTreeMap::new();
    map.insert(KEY_ACL_APP.to_string(), acl_app_address.to_string());
    map.insert(KEY_TOKEN_APP.to_string(), token_app_address.to_string());
    map.insert(KEY_VOTING_APP.to_string(), voting_app_address.to_string());
    //save apps;
    store_apps(map);
    */
    return_contract(Ok(Response { data: &("init complete".to_string()).as_bytes()}));
    
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
    let res = address.clone();
    return res
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
    let app_name = KEY_TOKEN_APP.to_string();
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
    let app_name = KEY_VOTING_APP;
    let addr = get_app_address(app_name.to_string());
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

fn query_permissions() {
    //
    let app_name = KEY_ACL_APP.to_string();
    let addr = get_app_address(app_name.to_string());
    let app_address = Address::from(&*addr);
    let deps = runtime::make_dependencies();
    let mut sink = Sink::new(0);
    sink.write_str("query_permissions");
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


fn set_acl_initial_permission_and_contract_address(votingAddr: &str, tokenAddr: &str, aclAddr: &str, communityAddr: &str) {
    //
    let deps = runtime::make_dependencies();
    let voting_address = Address::from(votingAddr);
    let token_address = Address::from(tokenAddr);
    let acl_address = Address::from(aclAddr);
    //let communityAddress = Address::from(communityAddr);

    //get permission list;
    let mut voting_sink = Sink::new(0);
    voting_sink.write_str("initial_permission_list");
    let voting_input = voting_sink.into();
    match deps.api.call_contract(&voting_address, &voting_input) {
        Some(res) => {
        //
        },
        None => {
            return_contract(Err("call contract error"));
        },
    };

    let mut token_sink = Sink::new(0);
    token_sink.write_str("initial_permission_list");
    let token_input = token_sink.into();
    match deps.api.call_contract(&token_address, &token_input) {
        Some(res) => {
        //
        },
        None => {
            return_contract(Err("call contract error"));
        },
    };
    let self_map = default_permission();
    
    let mut map: BTreeMap<String, [u8; ADDR_SIZE]> = BTreeMap::new();
    map.insert(String::from("voting.new_vote"), Address::from(ANY_ENTITY).into());
    map.insert(String::from("voting.cast_vote"), voting_address.into());
    map.insert(String::from("token.mint"), token_address.into());
    for (k, v) in self_map.iter() {
        let key = k.clone();
        let value = v.clone();
        map.insert(key, value);
    }
    
    //let mut map: BTreeMap<String, [u8; ADDR_SIZE]> = BTreeMap::new();
    //let res_map = map.clone();
    //map.insert();
    let map_byte = serde_json::to_vec(&map).unwrap();
    //set acl initial permission;
    let mut acl_sink = Sink::new(0);
    acl_sink.write_str("set_initial_permission");
    acl_sink.write_bytes(&map_byte);
    let acl_input = acl_sink.into();
    match deps.api.call_contract(&acl_address, &acl_input) {
        Some(res) => {
            //
        },
        None => {
            return_contract(Err("call contract error"));
        },
    };
    //set contract address;
    let mut sink = Sink::new(0);
    sink.write_str("set_token_contract_address");
    sink.write_str(tokenAddr);
    let input = sink.into();
    let call_addr = Address::from(votingAddr);
    match deps.api.call_contract(&call_addr, &input) {
        Some(res) => {
            //
        },
        None => {
            return_contract(Err("call contract error"));
        },
    };

}


#[no_mangle]
pub fn invoke() {
    //
    let deps = runtime::make_dependencies();
    let input = deps.api.input();
    let input = Source::new(&input);
    let method = input.read_str().unwrap();
    match method {
        "initial_contract" => {
            //
            let acl_app_address = input.read_str().unwrap();
            let token_app_address = input.read_str().unwrap();
            let voting_app_address = input.read_str().unwrap();
            let community_app_address = input.read_str().unwrap();

            set_acl_initial_permission_and_contract_address(voting_app_address, token_app_address, acl_app_address, community_app_address);

            let mut map:BTreeMap<String, String> = BTreeMap::new();
            map.insert(KEY_ACL_APP.to_string(), acl_app_address.to_string());
            map.insert(KEY_TOKEN_APP.to_string(), token_app_address.to_string());
            map.insert(KEY_VOTING_APP.to_string(), voting_app_address.to_string());
            //save apps;
            store_apps(map);
            return_contract(Ok(Response { data: &("initial contract complete".to_string()).as_bytes()}));
        }

        "query_apps" => {
            //
            let apps = get_apps();
            return_contract(Ok(Response{ data: &apps}));
        }
        
        "query_votes" => {
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

        "query_permissions" => {
            query_permissions()
        }

        "add_app" => {
            //
            let app_name = input.read_str().unwrap();
            let address = input.read_str().unwrap();
            add_app(app_name.to_string(), address.to_string());
            return_contract(Ok(Response{ data: &("add app success".to_string()).as_bytes()}));
        }

        "remove_app" => {
            //
            let app_name = input.read_str().unwrap();
            remove_app(app_name.to_string());
            return_contract(Ok(Response{ data: &("remove app success".to_string()).as_bytes()}));
        }

        "default_permission" => {
            let map = default_permission();
            let res = serde_json::to_vec(&map).unwrap();
            return_contract(Ok(Response {data : &res}));
        }

        _ => {
            //
            // 返回Error
            return_contract(Err("invoke method not found"));
        }
    }
}

fn default_permission() -> BTreeMap<String, [u8;ADDR_SIZE]> {
    //
    let app_address = _contract_address();
    let mut map: BTreeMap<String, [u8;ADDR_SIZE]> = BTreeMap::new();
    map.insert(String::from(ACTION_MANAGE), app_address.into());
    return map
}

fn _contract_address() -> Address {
    let app_address = get_app_address(String::from(KEY_COMMUNITY_APP));
    let app = &(*app_address);
    let app = Address::from(app);
    return app
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
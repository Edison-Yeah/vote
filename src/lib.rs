extern crate c123chain_cdk as cdk;

use cdk::types::{Address, Response};
use cdk::runtime;
use cdk::codec::{Sink, Source};
use std::collections::BTreeMap;


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
            return_contract(Ok(Response { data: &("invoke complete".to_string()).as_bytes()}));
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
            return_contract(Ok(Response { data: &("invoke complete".to_string()).as_bytes()}));
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
    set_param(KEY_SUPPORT_REQUIRED_PCT, &_support_required_pct.to_be_bytes());
    set_param(KEY_MIN_ACCEPT_QUORUM_PCT, &(_min_accept_quorum_pct.to_be_bytes()));
    set_param(KEY_VOTE_TIME, &(_vote_time.to_be_bytes()));

    set_param(KEY_VOTE_LENGTH, &(vote_length.to_be_bytes()));
    //return_contract(Ok(Response { data: &("vote_time = ".to_string() + &_vote_time.to_string()).as_bytes()}));
/*
    let a: u64 = 5;
    let b: u64 = 4;
    let d: u64 = 5;
    let c = a.wrapping_mul(b) / d;
    return_contract(Ok(Response { data: &("result = ".to_string() + &c.to_string()).as_bytes()}));
    */
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
                    let balance = convert_nibbles_to_u64(&res);
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
                    let total_supply = convert_nibbles_to_u64(&res);
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

//state list
pub enum VoteState {
    Absent,
    Yea,
    Nay
}

//vote
pub struct Vote {
    executed:bool,    //has been executed?
    start_date:u64,   //start time
    snapshot_block:u64,   //start block height;
    support_require_pct:u64,  //required supported percent;
    min_accept_quorum_pct:u64, //min accept quorum percent;
    yea:u64,      //supports
    nay:u64,      //opposite
    voting_power:u64,   //voting_power
    voters: BTreeMap<String, VoteState>,   //voter[voter_address, vote_state]
}
//vote function
impl Vote {

    pub fn new() -> Vote {
        //default vote;
        return Vote{
            executed: false,
            start_date: 0,
            snapshot_block: 0,
            support_require_pct: 0,
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
        let support_required_pct = convert_nibbles_to_u64(&support_required_pct_u8);
        let min_accept_quorum_pct = convert_nibbles_to_u64(&min_accept_quorum_pct_u8);
        self.support_require_pct = support_required_pct;
        self.min_accept_quorum_pct = min_accept_quorum_pct;
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
        sink.write_bool(self.executed);
        sink.write_u64(self.start_date);
        sink.write_u64(self.snapshot_block);
        sink.write_u64(self.min_accept_quorum_pct);
        sink.write_u64(self.yea);
        sink.write_u64(self.nay);
        sink.write_u64(self.voting_power);
        return sink.into();
    }

    //unMarshal
    fn un_serde(data: Vec<u8>) -> Vote {
        //
        let source = Source::new(data);
        let executed = source.read_bool().unwrap();
        let start_date = source.read_u64().unwrap();
        let snapshot_block = source.read_u64().unwrap();
        let min_accept_quorum_pct = source.read_u64().unwrap();
        let yea = source.read_u64().unwrap();
        let nay = source.read_u64().unwrap();
        let voting_power = source.read_u64().unwrap();
        //
        let mut vote = Vote::new();
        vote.executed = executed;
        vote.start_date = start_date;
        vote.snapshot_block = snapshot_block;
        vote.min_accept_quorum_pct = min_accept_quorum_pct;
        vote.yea = yea;
        vote.nay = nay;
        vote.voting_power = voting_power;
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
        let length = convert_nibbles_to_u64(&vote_length);
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
        set_param(KEY_VOTE_LENGTH, &(vote_id.to_be_bytes()));

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
        //save vote length
        set_param(KEY_VOTE_LENGTH, &(vote_id.to_be_bytes()));
        
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
        if !(self.is_value_pct(self.yea, total_votes, self.support_require_pct)) {
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
        let vote_time = convert_nibbles_to_u64(&time);
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
        let voter_state = self.voters.get(&address);

        let addr = get_param(KEY_TOKEN);
        let token_address = String::from_utf8(addr).unwrap();
        let token = MiniMeToken::new(&token_address);
        let voter_stake = token.balance_of_at(voter, self.snapshot_block);

        match voter_state {
            Some(VoteState::Yea) => {
                //
                self.yea = self.yea.wrapping_sub(voter_stake);
            }
            Some(VoteState::Nay) => {
                //
                self.nay = self.nay.wrapping_sub(voter_stake);
            }
            //Some(VoteState::Absent) => {
                //
            //}
            _ => {
                //
            }
        };

        let value;
        if support {
            //
            value = VoteState::Yea;
            self.yea = self.yea.wrapping_add(voter_stake);
        }else {
            //
            value = VoteState::Nay;
            self.nay = self.nay.wrapping_add(voter_stake);
        }
        self.voters.insert(address, value);

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
const KEY_TOKEN: &str = "_token";
//required support percent;
const KEY_SUPPORT_REQUIRED_PCT: &str = "_supportRequiredPct";
//min accept quorum percent;
const KEY_MIN_ACCEPT_QUORUM_PCT: &str = "_minAcceptQuorumPct";
//vote open time;
const KEY_VOTE_TIME: &str = "_voteTime";
//quantity of vote;
const KEY_VOTE_LENGTH: &str = "_vote_length";
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

fn convert_nibbles_to_u64(values: &[u8]) -> u64 {
    let mut out = 0;
    for &i in values {
        out = out << 4 | i as u64;
    }
    out
}


fn get_block_number() -> u64 {
    //return 2;
    return runtime::make_dependencies().api.get_block_number();
}



fn return_contract<'a>(result: Result<Response, &'a str>) {
    runtime::make_dependencies().api.ret(result)
}


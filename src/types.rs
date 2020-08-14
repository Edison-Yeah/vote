/*
extern crate c123chain_cdk as cdk;

//mod store;
use crate::store;

use cdk::types::Address;
use cdk::codec::{Sink, Source};
use cdk::runtime;
use std::collections::BTreeMap;


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
        sink.write_str("token_balance");
        sink.write_str(&(voter.to_hex_string()));
        sink.write_u64(block_number);
        let input = sink.into();

        match deps.api.call_contract(&self.address, &input) {
                Some(res) => return store::convert_nibbles_to_u64(&res),
                None => return 0,//return_contract(Err("call contract error")),
            };
    }

    fn total_supply_at(&self, block_number: u64) -> u64 {
        let deps = runtime::make_dependencies();

        let mut sink = Sink::new(0);
        sink.write_str("token_total_supply");
        sink.write_u64(block_number);
        let input = sink.into();
        //let input = block_number.to_be_bytes();
        match deps.api.call_contract(&self.address, &input) {
                Some(res) => return store::convert_nibbles_to_u64(&res),
                None => return 0,//return_contract(Err("call contract error")),
            };
        //return 100000000
    }
}

//state list
pub enum VoteState {
    Absent,
    Yea,
    Nay
}

pub struct Vote {
    executed:bool,
    start_date:u64,
    snapshot_block:u64,
    support_require_pct:u64,
    min_accept_quorum_pct:u64,
    yea:u64,
    nay:u64,
    voting_power:u64,
    voters: BTreeMap<String, VoteState>,
}

impl Vote {

    pub fn new() -> Vote {
        
        return Vote{
            executed: true,
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
        let support_required_pct_u8 = store::get_param(store::KEY_SUPPORT_REQUIRED_PCT);
        let min_accept_quorum_pct_u8 = store::get_param(store::KEY_MIN_ACCEPT_QUORUM_PCT);
        //let vote_time_u8 = store::get_param(store::KEY_VOTE_TIME);
        let support_required_pct = store::convert_nibbles_to_u64(&support_required_pct_u8);
        let min_accept_quorum_pct = store::convert_nibbles_to_u64(&min_accept_quorum_pct_u8);
        //let vote_time = store::convert_nibbles_to_u64(vote_time_u8);
        self.support_require_pct = support_required_pct;
        self.min_accept_quorum_pct = min_accept_quorum_pct;
    }

    //pub fn change_support_required_pct(&mut self, support_required_pct: u64) {
        //
    //}

    //pub fn change_min_accept_quorum_pct(&mut self, min_accept_quorum_pct: u64) {
        //
   //}

    fn to_string(&self) -> Vec<u8> {
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

    fn un_marshal(data: Vec<u8>) -> Vote {
        //
        let source = Source::new(data);
        let executed = source.read_bool();
        let start_date = source.read_u64();
        let snapshot_block = source.read_u64();
        let min_accept_quorum_pct = source.read_u64();
        let yea = source.read_u64();
        let nay = source.read_u64();
        let voting_power = source.read_u64();
        //
        let mut vote = Vote::new();
        vote.executed = executed.unwrap();
        vote.start_date = start_date.unwrap();
        vote.snapshot_block = snapshot_block.unwrap();
        vote.min_accept_quorum_pct = min_accept_quorum_pct.unwrap();
        vote.yea = yea.unwrap();
        vote.nay = nay.unwrap();
        vote.voting_power = voting_power.unwrap();
        return vote
    }

    //return voteId
    pub fn new_vote(&mut self, voter:&mut Address, cast_vote: bool, executes_if_decided: bool, time: u64) -> u64 {
        //
        let vote_length = store::get_param(store::KEY_VOTE_LENGTH);
        let length = store::convert_nibbles_to_u64(&vote_length);
        let vote_id = length + 1;
        let block_number = store::get_block_number() - 1;

        let addr = store::get_param(store::KEY_TOKEN);
        let address = String::from_utf8(addr).unwrap();
        let token = MiniMeToken::new(&address);
        let voting_power = token.total_supply_at(block_number);

        if voting_power <= 0 {
            panic!("error, no voting power");
        }
        self.voting_power = voting_power;
        self.set_params();
        self.start_date = time;
        self.snapshot_block = block_number;

        if cast_vote && self.can_vote(voter) {
            //cast voe
            self.vote(voter, vote_id, true, executes_if_decided);
        }
        //save vote
        //vore_id -> vote
        let res = self.to_string();
        let id = vote_id.to_string();
        store::set_param(&id, &res);
        //save vote length
        store::set_param(store::KEY_VOTE_LENGTH, &(vote_id.to_be_bytes()));

        return vote_id
    }

    pub fn cast_vote(vote_id: u64, supports: bool, executes_if_decided: bool, voter: &mut Address) {
        
        let id = vote_id.to_string();
        let res = store::get_param(&id);

        let mut vote = Vote::un_marshal(res);
        if !vote.can_vote(voter) {
            panic!("cannot vote")
        }
        vote.vote(voter, vote_id, supports, executes_if_decided);

        //save vote
        //vore_id -> vote
        let res = vote.to_string();
        let id = vote_id.to_string();
        store::set_param(&id, &res);
        //save vote length
        store::set_param(store::KEY_VOTE_LENGTH, &(vote_id.to_be_bytes()));
        
    }

    fn can_vote(&self, voter: &mut Address) -> bool {
        //
        let token_addr = store::get_param(store::KEY_TOKEN);
        let address = String::from_utf8(token_addr).unwrap();
        let token = MiniMeToken::new(&address);

        let block_number = store::get_block_number() - 1;

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
        /*
        let deps = runtime::make_dependencies();
        let time_stamp = deps.api.get_timestamp();
        let time = store::get_param(store::KEY_VOTE_TIME);
        let vote_time = store::convert_nibbles_to_u64(&time);
        return time_stamp < self.start_date.wrapping_add(vote_time) && !self.executed
        */
        return true
    }

    //Calculates whether `_value` is more than a percentage `_pct` of `_total`
    fn is_value_pct(&self, value: u64, total: u64, pct: u64) -> bool {
        //
        if total == 0 {
            return false
        }
        let pct_base: u64 = 1000000000000000000;
        let compucted_pct = value.wrapping_mul(pct_base) / total;
        return compucted_pct > pct
    }

    fn vote(&mut self, voter: &mut Address, vote_id : u64, support: bool, executes_if_decided: bool) {
        //
        let address = voter.to_hex_string();
        let voter_state = self.voters.get(&address);
        match voter_state {
            Some(VoteState::Yea) => {
                //
            }
            Some(VoteState::Nay) => {
                //
            }
            Some(VoteState::Absent) => {
                //
            }
            _ => {
                //
            }
        }

        let mut value = VoteState::Absent;
        if support {
            //
            value = VoteState::Yea;
        }else {
            //
            value = VoteState::Nay;
        }
        self.voters.insert(address, value);

        if executes_if_decided && self.can_execute(vote_id) {
            //execute
        }
    }
}
*/
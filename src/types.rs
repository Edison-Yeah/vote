extern crate c123chain_cdk as cdk;

//mod store;
use crate::store;

use cdk::types::Address;
use cdk::codec::{Sink, Source};
use cdk::runtime;


#[derive(Clone, Default, Debug, PartialEq)]
pub struct MiniMeToken(Address);

impl MiniMeToken {

    pub fn new(s: &str) -> MiniMeToken {
        
        return MiniMeToken(Address::from(s))
    }

    pub fn balance_of_at(&self) -> u64 {
        return 255
    }

    pub fn total_supply_at(&self) -> u64 {
        return 100000000
    }
}

//state list
enum vote_state {
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
    yea:u128,
    nay:u128,
    voting_power:u64,
}

impl Vote {

    pub fn new() -> Vote {
        //
        return Vote{
            executed: true,
            start_date: 0,
            snapshot_block: 0,
            support_require_pct: 0,
            min_accept_quorum_pct: 0,
            yea: 0,
            nay: 0,
            voting_power: 0,
        };
    }

    pub fn set_params(&mut self) {
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

    pub fn change_support_required_pct(&mut self, support_required_pct: u64) {
        //
    }

    pub fn change_min_accept_quorum_pct(&mut self, min_accept_quorum_pct: u64) {
        //
    }

    pub fn to_string(&self) -> Vec<u8> {
        //
        let mut sink = Sink::new(0);
        sink.write_bool(self.executed);
        sink.write_u64(self.start_date);
        sink.write_u64(self.snapshot_block);
        sink.write_u64(self.min_accept_quorum_pct);
        sink.write_u128(self.yea);
        sink.write_u128(self.nay);
        sink.write_u64(self.voting_power);
        return sink.into();
    }

    pub fn un_marshal(data: Vec<u8>) -> Vote {
        //
        let source = Source::new(data);
        let executed = source.read_bool();
        let start_date = source.read_u64();
        let snapshot_block = source.read_u64();
        let min_accept_quorum_pct = source.read_u64();
        let yea = source.read_u128();
        let nay = source.read_u128();
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
    pub fn new_vote(&mut self, cast_vote: bool, executes_if_decided: bool, time: u64) -> u64 {
        //
        //let vote_length = store::get_param(store::KEY_VOTE_LENGTH);
        //let length = store::convert_nibbles_to_u64(&vote_length);
        let vote_id = 1;//length + 1;

        let addr = store::get_param(store::KEY_TOKEN);
        let address = String::from_utf8(addr).unwrap();
        let token = MiniMeToken::new(&address);
        let voting_power = token.total_supply_at();

        let block_number = store::get_block_number() - 1;

        if voting_power <= 0 {
            panic!("error, no voting power");
        }
        self.voting_power = voting_power;
        self.set_params();
        self.start_date = time;
        self.snapshot_block = block_number;

        //save vote
        //vore_id -> vote
        let res = self.to_string();

        //let mut sink = Sink::new(0);
       // sink.write_u64(vote_id);
        //let id = sink.into();
        //let id = vote_id.to_string();
        let id = "hh".to_string();
        store::set_param(&id, &res);

        //runtime::make_dependencies()
    
        //.storage
        //.set(&id, &res);
        runtime::make_dependencies()
        .storage
        .get(&id.as_bytes());

        return vote_id
    }

    pub fn cast_vote(vote_id: u64, supports: bool, executes_if_decided: bool, voter: Address) {
        //

        let id = vote_id.to_string();
        let res = store::get_param(&id);
        //let vote = Vote::un_marshal(res);
        //if !vote.can_vote(voter) {
         //   panic!("cannot vote")
        //}
        //vote.vote(vote_id, supports, executes_if_decided);
    }

    pub fn can_vote(&self, voter: Address) -> bool {
        //
        return self.is_vote_open() //&& token.balanceOfAt(voter, block_number)
    }

    pub fn can_execute() -> bool {
        //
        return true
    }

    pub fn get_vote() {
        //
    }

    fn get_vote_state() -> vote_state {
        //
        return vote_state::Absent
    }

    //return true if the given vote is open, false otherwise
    pub fn is_vote_open(&self) -> bool {
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
    pub fn is_value_pct(value: u128, total: u128, pct: u128) -> bool {
        //
        if total == 0 {
            return false
        }
        let pct_base: u128 = 1000000000000000000;
        let compucted_pct = value.wrapping_mul(pct_base) / total;
        return compucted_pct > pct
    }

    pub fn vote(&self, vote_id : u64, support: bool, executes_if_decided: bool) {
        //
    }
}
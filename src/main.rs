extern crate trexlib;
extern crate trex;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};

use trexlib::lib::BallotNumber;

fn main() {
    
    let expected = BallotNumber { era: 0, counter: 0, node_identifier: 0};
    let mut buf = Vec::new();
    expected.serialize(&mut Serializer::new(&mut buf)).unwrap();

    let mut de = Deserializer::new(&buf[..]);

    let round: BallotNumber = Deserialize::deserialize(&mut de).unwrap();

    println!("the round is {:?}", round);

}
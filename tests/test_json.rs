extern crate trex;
extern crate trex_ext;

use trex::BallotNumber;
use trex_ext::serde_trex::BallotNumberJSON;
use trex_ext::serde_trex::JSON;

#[test]
fn test_BallotNumberJSON() {
    let bn = BallotNumberJSON(BallotNumber { era: 0, counter: 1, node_identifier: 2});

    let json = "{\"e\":0,\"c\":1,\"n\":2}";

    println!("the expected is {:?}", bn.to_json());

    assert_eq!(json,bn.to_json());
}
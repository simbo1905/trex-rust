extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};

pub mod lib {

    use std::cmp::Ordering;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct BallotNumber {
        pub counter: u32,
        pub node_identifier: u32,
        pub era: u32
    }

    impl PartialOrd for BallotNumber {
        fn partial_cmp(&self, other: &BallotNumber) -> Option<Ordering> {

            let o: Ordering;
            
            if self.era == other.era { 
                if self.counter == other.counter {
                    o = self.node_identifier.cmp(&other.node_identifier)
                } else {
                    o = self.counter.cmp(&other.counter)
                }
            } else { 
                o = self.era.cmp(&other.era)
            }

            Some(o)
        }
    }
}

#[cfg(test)]
mod tests {
    use lib::*;

    #[test]
    fn ballot_number_eq() {
        let b1 = BallotNumber{ counter: 0, node_identifier: 0, era: 0};
        let b2 = BallotNumber{ counter: 0, node_identifier: 0, era: 0};
        assert_eq!(b1, b2);
        assert!( !(b1 > b2) );
        assert!( !(b1 < b2) );
        assert!( (b1 <= b2) );
        assert!( (b1 >= b2) );
    }

    #[test]
    fn ballot_number_cmp_counter() {
        let b1 = BallotNumber{ counter: 1, node_identifier: 0, era: 0};
        let b2 = BallotNumber{ counter: 0, node_identifier: 0, era: 0};
        assert!( b1 > b2);
        assert!( b1 >= b2);
        assert!( !(b1 < b2));
        assert!( !(b1 <= b2));
    }

    #[test]
    fn ballot_number_cmp_node_identifier() {
        let b1 = BallotNumber{ counter: 0, node_identifier: 1, era: 0};
        let b2 = BallotNumber{ counter: 0, node_identifier: 0, era: 0};
        assert!( b1 > b2);
        assert!( b1 >= b2);
        assert!( !(b1 < b2));
        assert!( !(b1 <= b2));
    }

    #[test]
    fn ballot_number_cmp_era() {
        let b1 = BallotNumber{ counter: 0, node_identifier: 0, era: 1};
        let b2 = BallotNumber{ counter: 0, node_identifier: 0, era: 0};
        assert!( b1 > b2);
        assert!( b1 >= b2);
        assert!( !(b1 < b2));
        assert!( !(b1 <= b2));
    }
}

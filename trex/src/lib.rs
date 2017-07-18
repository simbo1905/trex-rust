use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct BallotNumber {
    pub era: u32,
    pub counter: u32,
    pub node_identifier: u32
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



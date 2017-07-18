
use trex::BallotNumber;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json;

#[derive(Debug)]
pub struct BallotNumberJSON(pub BallotNumber);

impl Serialize for BallotNumberJSON {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match self.0 {
            BallotNumber { era, counter, node_identifier } => {
                // 3 is the number of fields in the struct.
                let mut state = serializer.serialize_struct("N", 3)?;
                state.serialize_field("e", &era)?;
                state.serialize_field("c", &counter)?;
                state.serialize_field("n", &node_identifier)?;
                state.end()
            }
        }
    }
}

pub trait JSON {
    fn to_json(&self) -> String
            where Self: Serialize {
        return serde_json::to_string(&self).unwrap();
    }
}

impl JSON for BallotNumberJSON {
    // add code here
}

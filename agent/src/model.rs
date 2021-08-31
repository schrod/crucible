use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Requested,
    Created,
    Tombstoned,
    Destroyed,
    Failed,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, PartialEq, Clone)]
pub struct Region {
    pub id: RegionId,
    pub volume_id: String,
    pub port_number: u16,
    pub state: State,
    pub block_size: u32,
    pub block_count: u32,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, PartialEq, Clone)]
pub struct CreateRegion {
    pub id: RegionId,
    pub volume_id: String,
    pub block_size: u32,
    pub block_count: u32,
}

impl CreateRegion {
    pub fn mismatch(&self, r: &Region) -> Option<String> {
        if self.volume_id != r.volume_id {
            Some(format!(
                "volume ID {} instead of requested {}",
                self.volume_id, r.volume_id
            ))
        } else if self.block_size != r.block_size {
            Some(format!(
                "block size {} instead of requested {}",
                self.block_size, r.block_size
            ))
        } else if self.block_count != r.block_count {
            Some(format!(
                "block count {} instead of requested {}",
                self.block_count, r.block_count
            ))
        } else {
            None
        }
    }
}

#[derive(
    Serialize, Deserialize, JsonSchema, Debug, PartialEq, Eq, Clone, Ord,
)]
pub struct RegionId(pub String);

impl PartialOrd for RegionId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let r = Region {
            id: RegionId("abc".to_string()),
            volume_id: "def".to_string(),
            port_number: 1701,
            state: State::Requested,
        };

        let s = serde_json::to_string(&r).expect("serialise");
        println!("{}", s);

        let recons: Region = serde_json::from_str(&s).expect("deserialise");

        assert_eq!(r, recons);
    }
}
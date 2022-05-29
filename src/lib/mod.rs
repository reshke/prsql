

use serde::{Deserialize, Serialize};

use bincode::{serialize};

#[derive(Serialize, Deserialize)]
pub struct FrontendMessage {
    len: i32,
}


#[derive(Serialize, Deserialize)]
pub struct StartupMsg {
    Fr: FrontendMessage,
    pub ProtocolVersionNumber: i32,
}


impl StartupMsg {
    pub fn new() -> StartupMsg {
        StartupMsg{ 
            Fr: FrontendMessage{
                len: 8,
            },
            ProtocolVersionNumber: 196608,
        }
    }

    pub fn tobytes(self: &StartupMsg) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}
use serde::{Serialize, Deserialize};
use getset::{Getters, Setters};

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(tag = "type")] // This adds a "type" field to indicate which variant is being serialized
// pub enum MessageType {
//     Handshake(Handshake),
//     HandshakeOk(HandshakeOk),
//     StartNewTable(StartNewTable),
//     StartNewTableOk(StartNewTableOk),
// }

#[derive(Debug, Getters, Serialize, Deserialize)]
pub(crate) struct Handshake { }

impl Handshake {
    pub fn new() -> Handshake{
        Handshake {
        }
    }
}

#[derive(Debug, Getters, Serialize, Deserialize)]
pub(crate) struct HandshakeOk {
    #[getset(get = "pub")]
    client_id: u128
}

impl HandshakeOk {
    pub fn new(client_id : u128) -> HandshakeOk {
        HandshakeOk {
            client_id
        }
    }
}

#[derive(Debug, Getters, Serialize, Deserialize)]
pub(crate) struct StartNewTable {
    #[getset(get = "pub")]
    client_id: u128
}

impl StartNewTable {
    pub fn new(unique_id : u128) -> StartNewTable {
        StartNewTable {
            client_id: unique_id
        }
    }
}

#[derive(Debug, Getters, Serialize, Deserialize)]
pub(crate) struct StartNewTableOk {
    #[getset(get = "pub")]
    client_id: u128,

    #[getset(get = "pub")]
    table_id: u128
}

impl StartNewTableOk {
    pub fn new(client_id: u128, table_id: u128) -> StartNewTableOk {
        StartNewTableOk {
            client_id,
            table_id
        }
    }
}

#[derive(Debug, Getters, Serialize, Deserialize)]
pub(crate) struct JoinTable {
    #[getset(get = "pub")]
    client_id: u128,

    #[getset(get = "pub")]
    table_id: u128
}

impl JoinTable {
    pub fn new(client_id: u128, table_id: u128) -> JoinTable {
        JoinTable {
            client_id,
            table_id
        }
    }
}

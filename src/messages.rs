use serde::{Deserialize, Serialize};
use getset::Getters;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")] // This adds a "type" field to indicate which variant is being serialized
pub enum MessageType {
    Handshake(Handshake),
    HandshakeOk(HandshakeOk),
    StartNewTable(StartNewTable),
    StartNewTableOk(StartNewTableOk),
    QueryTables(QueryTables),
    TablesInfo(TablesInfo),
    JoinTable(JoinTable),
    JoinTableOk(JoinTableOk)
}

impl MessageType {
    fn to_string(&self) -> String {
        match self {
            MessageType::Handshake(_) => String::from("Handshake"),
            MessageType::HandshakeOk(_) => String::from("HandshakeOk"),
            MessageType::StartNewTable(_) => String::from("StartNewTable"),
            MessageType::StartNewTableOk(_) => String::from("StartNewTableOk"),
            MessageType::QueryTables(_) => String::from("QueryTables"),
            MessageType::TablesInfo(_) => String::from("TablesInfo"),
            MessageType::JoinTable(_) => String::from("JoinTable"),
            MessageType::JoinTableOk(_) => String::from("JoinTableOk"),
        }
    }
}

#[derive(Debug, Getters, Serialize, Deserialize)]
pub(crate) struct Handshake { }

impl Handshake {
    pub fn new() -> Handshake{
        Handshake {
        }
    }
}
impl From<Handshake> for MessageType {
    fn from(msg: Handshake) -> Self {
        MessageType::Handshake(msg)
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

impl From<HandshakeOk> for MessageType {
    fn from(msg: HandshakeOk) -> Self {
        MessageType::HandshakeOk(msg)
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

impl From<StartNewTable> for MessageType {
    fn from(msg: StartNewTable) -> Self {
        MessageType::StartNewTable(msg)
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

impl From<StartNewTableOk> for MessageType {
    fn from(msg: StartNewTableOk) -> Self {
        MessageType::StartNewTableOk(msg)
    }
}

#[derive(Debug, Getters, Serialize, Deserialize)]
pub(crate) struct QueryTables {
    #[getset(get = "pub")]
    client_id: u128,
}

impl QueryTables {
    pub fn new(client_id: u128) -> QueryTables {
        QueryTables {
            client_id,
        }
    }
}

impl From<QueryTables> for MessageType {
    fn from(msg: QueryTables) -> Self {
        MessageType::QueryTables(msg)
    }
}

#[derive(Debug, Getters, Serialize, Deserialize)]
pub(crate) struct TablesInfo {
    #[getset(get = "pub")]
    tables: Vec<u128>,
}

impl TablesInfo {
    pub fn new(tables: Vec<u128>) -> TablesInfo {
        TablesInfo {
            tables
        }
    }
}

impl From<TablesInfo> for MessageType {
    fn from(msg: TablesInfo) -> Self {
        MessageType::TablesInfo(msg)
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

impl From<JoinTable> for MessageType {
    fn from(msg: JoinTable) -> Self {
        MessageType::JoinTable(msg)
    }
}

#[derive(Debug, Getters, Serialize, Deserialize)]
pub(crate) struct JoinTableOk {
    #[getset(get = "pub")]
    client_id: u128,

    #[getset(get = "pub")]
    table_id: u128
}

impl JoinTableOk {
    pub fn new(client_id: u128, table_id: u128) -> JoinTableOk {
        JoinTableOk {
            client_id,
            table_id
        }
    }
}

impl From<JoinTableOk> for MessageType {
    fn from(msg: JoinTableOk) -> Self {
        MessageType::JoinTableOk(msg)
    }
}

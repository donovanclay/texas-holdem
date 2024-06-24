use serde::Deserialize;
use serde_json::{from_str, Value};
use crate::messages::{Handshake, HandshakeOk, JoinTable, JoinTableOk, MessageType, QueryTables, StartNewTable, StartNewTableOk, TablesInfo};

pub(crate) fn deserialize<T>(text: &str) -> Result<T, &'static str>
    where
        T: Deserialize<'static> + std::fmt::Debug,
{
    // Deserialize JSON into a generic Value first
    let value: Value = match from_str(text) {
        Ok(v) => v,
        Err(_) => return Err("Failed to deserialize JSON"),
    };

    // Extract the type field from the JSON
    let message_type = match value.get("type") {
        Some(v) => match v.as_str() {
            Some(s) => s,
            None => return Err("Invalid 'type' field in JSON"),
        },
        None => return Err("Missing 'type' field in JSON"),
    };

    // Match against the specific message type and deserialize accordingly
    let message: MessageType = match message_type {
        "Handshake" => {
            let msg: Result<Handshake, _> = serde_json::from_value(value);
            match msg {
                Ok(handshake) => MessageType::Handshake(handshake),
                Err(_) => return Err("Failed to deserialize Handshake"),
            }
        }
        "HandshakeOk" => {
            let msg: Result<HandshakeOk, _> = serde_json::from_value(value);
            match msg {
                Ok(handshake_ok) => MessageType::HandshakeOk(handshake_ok),
                Err(_) => return Err("Failed to deserialize HandshakeOk"),
            }
        }
        "StartNewTable" => {
            let msg: Result<StartNewTable, _> = serde_json::from_value(value);
            match msg {
                Ok(start_new_table) => MessageType::StartNewTable(start_new_table),
                Err(_) => return Err("Failed to deserialize StartNewTable"),
            }
        }
        "StartNewTableOk" => {
            let msg: Result<StartNewTableOk, _> = serde_json::from_value(value);
            match msg {
                Ok(start_new_table_ok) => MessageType::StartNewTableOk(start_new_table_ok),
                Err(_) => return Err("Failed to deserialize StartNewTableOk"),
            }
        }
        "QueryTables" => {
            let msg: Result<QueryTables, _> = serde_json::from_value(value);
            match msg {
                Ok(query_tables) => MessageType::QueryTables(query_tables),
                Err(_) => return Err("Failed to deserialize QueryTables"),
            }
        }
        "TablesInfo" => {
            let msg: Result<TablesInfo, _> = serde_json::from_value(value);
            match msg {
                Ok(tables_info) => MessageType::TablesInfo(tables_info),
                Err(_) => return Err("Failed to deserialize TablesInfo"),
            }
        }
        "JoinTable" => {
            let msg: Result<JoinTable, _> = serde_json::from_value(value);
            match msg {
                Ok(join_table) => MessageType::JoinTable(join_table),
                Err(_) => return Err("Failed to deserialize JoinTable"),
            }
        }
        "JoinTableOk" => {
            let msg: Result<JoinTableOk, _> = serde_json::from_value(value);
            match msg {
                Ok(join_table_ok) => MessageType::JoinTableOk(join_table_ok),
                Err(_) => return Err("Failed to deserialize JoinTableOk"),
            }
        }
        _ => return Err("Unknown message type"),
    };

    // Ensure the deserialized message matches the expected type T
    match message {
        MessageType::Handshake(m) if message.type() == "Handshake" => Ok(m),
        MessageType::HandshakeOk(m) if T::type_name() == "HandshakeOk" => Ok(m),
        MessageType::StartNewTable(m) if T::type_name() == "StartNewTable" => Ok(m),
        MessageType::StartNewTableOk(m) if T::type_name() == "StartNewTableOk" => Ok(m),
        MessageType::QueryTables(m) if T::type_name() == "QueryTables" => Ok(m),
        MessageType::TablesInfo(m) if T::type_name() == "TablesInfo" => Ok(m),
        MessageType::JoinTable(m) if T::type_name() == "JoinTable" => Ok(m),
        MessageType::JoinTableOk(m) if T::type_name() == "JoinTableOk" => Ok(m),
        _ => Err("Deserialized message type does not match expected type"),
    }
}

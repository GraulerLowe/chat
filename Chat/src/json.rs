use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct ClientMessage {
    pub id: String,     // Identificador único del cliente
    pub name: String,   // Nombre del cliente
    pub message: String, // Mensaje enviado
}

pub fn to_json(message: &ClientMessage) -> Result<String, serde_json::Error> {
    serde_json::to_string(message)
}

pub fn from_json(json: &str) -> Result<ClientMessage, serde_json::Error> {
    serde_json::from_str(json)
}


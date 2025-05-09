use serde::Serialize;

#[derive(Serialize)]
pub struct Envelope {
    pub is_successful: bool,
    pub message: String,
}

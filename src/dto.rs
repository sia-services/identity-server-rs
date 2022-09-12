use serde::Serialize;

#[derive(Serialize)]
pub struct BooleanResponse {
    pub result: bool,
}

pub const TRUE_RESPONSE: BooleanResponse = BooleanResponse { result: true };
pub const FALSE_RESPONSE: BooleanResponse = BooleanResponse { result: false };

impl BooleanResponse {
    pub const fn of(result: bool) -> Self {
        if result {
            TRUE_RESPONSE
        } else {
            FALSE_RESPONSE
        }
    }
}

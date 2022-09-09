#[derive(Serialize)]
pub struct BooleanResponse {
    pub result: boolean,
}

pub const TRUE_RESPONSE: BooleanResponse = BooleanResponse::construct(true);
pub const FALSE_RESPONSE: BooleanResponse = BooleanResponse::construct(false);

impl BooleanResponse {
    pub const fn construct(result: bool) -> Self {
        Self { result }
    }
}

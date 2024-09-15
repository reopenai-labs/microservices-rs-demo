use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IdParam {
    pub id: i64,
}

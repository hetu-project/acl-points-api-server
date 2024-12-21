use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    //pub req_id: String,
    pub code: u16,
    pub result: T,
}

#[derive(Deserialize, Debug)]
pub struct OAuthCallbackParams {
    pub code: String,
    pub scope: String,
    pub authuser: String,
    pub prompt: String,
}

#[derive(Deserialize, Debug)]
pub struct OAuthParams {
    pub code: String,
    pub scope: String,
    pub authuser: String,
    pub prompt: String,
    pub state: String,
    pub redirect_uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub sub: String,
    pub name: String,
    pub email: String,
}

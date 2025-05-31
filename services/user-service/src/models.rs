use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RegisterUser { 
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}
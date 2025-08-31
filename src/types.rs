use serde::{
    Deserialize,
    Serialize
};

#[derive(Debug,Serialize, Deserialize)]

pub struct Account{
    pub id : String,
    pub email : String,
    pub username : String,
    pub balance : f64,
    pub created_at : chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct CreateAccountRequest{
    pub username : String,
    pub email : String
}

#[derive(Debug, Serialize , Deserialize)]

pub struct ApiResponse<T>{
    pub success : bool,
    pub data : Option<T>,
    pub error : Option<String>
}

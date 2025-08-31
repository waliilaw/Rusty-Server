use axum::{
    Json , 
    response::IntoResponse,
    extract::Path
};

use uuid::Uuid;
use crate::types::{Account, CreateAccountRequest , ApiResponse};
use chrono::Utc;

pub async fn account(Json(payload) : Json<CreateAccountRequest>) -> impl IntoResponse{
    let account = Account{
        id : Uuid::new_v4().to_string(),
        email : payload.email,
        username : payload.username,
        balance: 0.0,
        created_at : Utc::now()
    };
    
    let response = ApiResponse{
        success : true,
        data : Some(account),
        error : None 
    };

    Json(response)
}


pub async fn get_account( Path(id) : Path<String>) -> impl IntoResponse{
    let account = Account{
        id,
        username : "wali".to_string(),
        email : "humaidwali20@gmail.com".to_string(),
        balance : 100.0 ,
        created_at : Utc::now()
        };
     
    let response = ApiResponse{
        success : true,
        data : Some(account),
        error : None 
    };

    Json(response)
}
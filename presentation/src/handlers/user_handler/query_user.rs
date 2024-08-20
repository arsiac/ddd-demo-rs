use std::sync::Arc;

use application::services::UserAppService;
use axum::{
    extract::{Query, State},
    Json,
};
use axum_extra::extract::WithRejection;
use chrono::{DateTime, Local};
use domain::{entities::UserEntity, error::DomainError};
use infrastructure::repositories::UserRepository;
use serde::{Deserialize, Serialize};

use crate::{handlers::ErrorResponse, AppState};

pub async fn get_one(
    State(state): State<Arc<AppState>>,
    WithRejection(Query(params), _): WithRejection<Query<UserQuery>, ErrorResponse>,
) -> crate::handlers::Result<Json<UserVO>> {
    let user_repo = UserRepository::new(Arc::clone(&state.connection));
    let servcie = UserAppService::new(user_repo);
    let user;
    if let Some(id) = params.id {
        user = servcie.get_nonone_by_id(id).await;
    } else if let Some(email) = &params.email {
        user = servcie.get_nonone_by_email(email).await;
    } else {
        return Err(DomainError::Other("id or email is required".into()).into());
    };
    if let Err(err) = user {
        return Err(err.into());
    };
    Ok(Json(user.unwrap().into()))
}

#[derive(Debug, Deserialize)]
pub struct UserQuery {
    id: Option<i32>,
    email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserVO {
    pub id: i32,
    pub nickname: String,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<UserEntity> for UserVO {
    fn from(value: UserEntity) -> Self {
        UserVO {
            id: value.id,
            nickname: value.nickname,
            username: value.username,
            email: value.email.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

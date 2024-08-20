use std::sync::Arc;

use crate::{handlers::ErrorResponse, AppState};
use application::services::UserAppService;
use axum::{extract::State, Json};
use axum_extra::extract::WithRejection;
use domain::entities::UserRegisterEntity;
use infrastructure::repositories::UserRepository;
use serde::Deserialize;

pub async fn register(
    State(state): State<Arc<AppState>>,
    WithRejection(Json(user), _): WithRejection<Json<UserRegisterDTO>, ErrorResponse>,
) -> crate::handlers::Result<()> {
    let service = UserAppService::new(UserRepository::new(state.connection.clone()));
    let user = TryInto::<UserRegisterEntity>::try_into(user);
    if let Err(err) = user {
        return Err(err.into());
    };
    let user = user.unwrap();
    let result = service.register(&user).await;
    if let Err(err) = result {
        return Err(err.into());
    };
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct UserRegisterDTO {
    pub email: String,
    pub password: String,
}

impl TryFrom<UserRegisterDTO> for UserRegisterEntity {
    type Error = domain::error::DomainError;

    fn try_from(value: UserRegisterDTO) -> Result<Self, Self::Error> {
        Ok(UserRegisterEntity {
            email: value.email.parse()?,
            password: value.password,
        })
    }
}

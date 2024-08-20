use std::sync::Arc;

use crate::entities::{
    self,
    sys_user::{ActiveModel, Model},
};
use domain::{
    entities::UserEntity,
    error::{DomainError, UserError},
};
use entities::sys_user::{Column, Entity};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter,
};

pub struct UserRepository {
    connection: Arc<DatabaseConnection>,
}

impl UserRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        UserRepository { connection }
    }
}

#[async_trait::async_trait]
impl domain::repositories::UserRepository for UserRepository {
    async fn add(&self, user: &UserEntity) -> domain::error::Result<()> {
        let user_model: Model = user.clone().into();
        let mut user_active_model: ActiveModel = user_model.into();
        user_active_model.id = NotSet;
        let result = user_active_model.insert(self.connection.as_ref()).await;
        if let Err(err) = result {
            return Err(domain::error::DomainError::Other(err.to_string()));
        }
        Ok(())
    }
    async fn get_by_id(&self, id: i32) -> domain::error::Result<Option<UserEntity>> {
        let user_model = Entity::find_by_id(id).one(self.connection.as_ref()).await;
        if let Err(e) = user_model {
            return Err(DomainError::Other(e.to_string()));
        }
        let user_model = user_model.unwrap();
        if user_model.is_none() {
            return Err(UserError::NotFound(id.to_string()).into());
        }
        let user_model = user_model.unwrap();
        let user = UserEntity::try_from(user_model)?;
        Ok(Some(user))
    }

    async fn get_by_email(&self, email: &str) -> domain::error::Result<Option<UserEntity>> {
        let user = Entity::find()
            .filter(Column::Email.eq(email))
            .one(self.connection.as_ref())
            .await;
        if let Err(e) = user {
            return Err(DomainError::Other(e.to_string()));
        }
        let user = user.unwrap();
        if user.is_none() {
            return Ok(None);
        }
        Ok(Some(user.unwrap().try_into()?))
    }

    async fn get_by_username(&self, username: &str) -> domain::error::Result<Option<UserEntity>> {
        let user = Entity::find()
            .filter(Column::Username.eq(username))
            .one(self.connection.as_ref())
            .await;
        if let Err(e) = user {
            return Err(DomainError::Other(e.to_string()));
        }
        let user = user.unwrap();
        if user.is_none() {
            return Ok(None);
        }
        Ok(Some(user.unwrap().try_into()?))
    }
}

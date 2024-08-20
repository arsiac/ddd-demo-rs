//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use chrono::{DateTime, Local};
use domain::entities::UserEntity;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    pub nickname: String,
    pub password: String,
    #[sea_orm(unique)]
    pub email: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl TryFrom<Model> for UserEntity {
    type Error = domain::error::DomainError;
    fn try_from(val: Model) -> Result<Self, Self::Error> {
        Ok(UserEntity {
            id: val.id,
            username: val.username,
            nickname: val.nickname,
            password: val.password,
            email: val.email.try_into()?,
            created_at: val.created_at,
            updated_at: val.updated_at,
        })
    }
}

impl From<UserEntity> for Model {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            username: entity.username,
            nickname: entity.nickname,
            password: entity.password,
            email: entity.email.into(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

use crate::entities::UserEntity;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn add(&self, user: &UserEntity) -> crate::error::Result<()>;

    async fn get_by_id(&self, id: i32) -> crate::error::Result<Option<UserEntity>>;

    async fn get_by_email(&self, email: &str) -> crate::error::Result<Option<UserEntity>>;

    async fn get_by_username(&self, username: &str) -> crate::error::Result<Option<UserEntity>>;
}

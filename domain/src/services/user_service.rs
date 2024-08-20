use chrono::Local;

use crate::entities::UserRegisterEntity;
use crate::error::UserError;
use crate::{entities::UserEntity, repositories::UserRepository};

pub struct UserService<Repo: UserRepository> {
    user_repository: Repo,
}

impl<Repo: UserRepository> UserService<Repo> {
    pub fn new(user_repository: Repo) -> Self {
        UserService { user_repository }
    }

    pub async fn register(&self, user: &UserRegisterEntity) -> crate::error::Result<()> {
        let exists_user = self
            .user_repository
            .get_by_email(user.email.as_str())
            .await?;
        if let Some(exists_user) = exists_user {
            log::debug!(
                "Email address '{}' has been used by user '{}'",
                &exists_user.email.as_str(),
                &exists_user.username
            );
            return Err(UserError::DuplicateEmail(exists_user.email.into()).into());
        }

        let mut new_user = UserEntity {
            email: user.email.clone(),
            password: user.password.clone(),
            ..Default::default()
        };

        let email = &new_user.email.as_str();
        let (username, _) = email.split_once('@').unwrap();
        new_user.nickname = username.into();

        let mut guess_username = username.to_string();
        let mut index = 0;
        loop {
            let exists_user = self
                .user_repository
                .get_by_username(&guess_username)
                .await?;
            if let Some(exists_user) = exists_user {
                log::debug!(
                    "Username '{}' has been used by user '{}'",
                    &exists_user.username,
                    &exists_user.email
                );
                index += 1;
                guess_username = format!("{}{}", username, index);
            } else {
                new_user.username = guess_username;
                break;
            }
        }
        let datetime = Local::now();
        new_user.created_at = datetime;
        new_user.updated_at = datetime;
        log::info!(
            "Register user '{}' using the email address '{}'",
            &new_user.username,
            new_user.email.as_str()
        );
        UserRepository::add(&self.user_repository, &new_user).await
    }

    pub async fn get_by_id(&self, id: i32) -> crate::error::Result<Option<UserEntity>> {
        UserRepository::get_by_id(&self.user_repository, id).await
    }

    pub async fn get_nonone_by_id(&self, id: i32) -> crate::error::Result<UserEntity> {
        let user = UserRepository::get_by_id(&self.user_repository, id).await?;
        match user {
            Some(user) => Ok(user),
            None => Err(UserError::NotFound(id.to_string()).into()),
        }
    }

    pub async fn get_by_email(&self, email: &str) -> crate::error::Result<Option<UserEntity>> {
        UserRepository::get_by_email(&self.user_repository, email).await
    }

    pub async fn get_nonone_by_email(&self, email: &str) -> crate::error::Result<UserEntity> {
        let user = UserRepository::get_by_email(&self.user_repository, email).await?;
        match user {
            Some(user) => Ok(user),
            None => Err(UserError::NotFound(email.to_string()).into()),
        }
    }
}

use domain::entities::{UserEntity, UserRegisterEntity};
use domain::repositories::UserRepository;
use domain::services::UserService;

pub struct UserAppService<Repo: UserRepository> {
    user_service: UserService<Repo>,
}

impl<Repo: UserRepository> UserAppService<Repo> {
    pub fn new(user_repository: Repo) -> Self {
        UserAppService {
            user_service: UserService::new(user_repository),
        }
    }

    pub async fn register(&self, user: &UserRegisterEntity) -> domain::error::Result<()> {
        UserService::register(&self.user_service, user).await
    }

    pub async fn get_nonone_by_id(&self, id: i32) -> domain::error::Result<UserEntity> {
        UserService::get_nonone_by_id(&self.user_service, id).await
    }

    pub async fn get_nonone_by_email(&self, email: &str) -> domain::error::Result<UserEntity> {
        UserService::get_nonone_by_email(&self.user_service, email).await
    }
}

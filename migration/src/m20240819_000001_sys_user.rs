use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Username).string_len(30))
                    .col(string(User::Nickname).string_len(30))
                    .col(string(User::Password).string_len(32))
                    .col(string(User::Email).string_len(200))
                    .col(date_time(User::CreatedAt))
                    .col(date_time(User::UpdatedAt))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_user_username")
                    .table(User::Table)
                    .col(User::Username)
                    .unique()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_user_email")
                    .table(User::Table)
                    .col(User::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "sys_user")]
    Table,
    Id,
    Username,
    Nickname,
    Password,
    Email,
    CreatedAt,
    UpdatedAt,
}

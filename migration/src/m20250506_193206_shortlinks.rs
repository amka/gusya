use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "shortlinks",
            &[
            
            ("id", ColType::PkAuto),
            
            ("original_url", ColType::StringNull),
            ("short_code", ColType::StringUniq),
            ("custom_alias", ColType::StringNull),
            ("domain", ColType::StringNull),
            ("expires_at", ColType::DateTimeNull),
            ("password", ColType::StringNull),
            ("is_active", ColType::BooleanNull),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "shortlinks").await
    }
}

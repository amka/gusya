pub use super::_entities::shortlinks::{ActiveModel, Entity, Model};
use crate::models::_entities::shortlinks::Column;
use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::model::ModelError;
use loco_rs::Error;
use nanoid::nanoid;
use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::Deserialize;
use url::Url;

pub type Shortlinks = Entity;

#[derive(DeriveIntoActiveModel, Deserialize)]
pub struct AddParams {
    pub original_url: String,
    pub custom_alias: Option<String>,
    pub domain: Option<String>,
    pub password: Option<String>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {
    fn generate_short_code() -> String {
        nanoid!(6) // 6 символов из алфавита по умолчанию
    }

    /// Создает новый `Shortlink`
    ///
    /// # Arguments
    ///
    /// * `db` - соединение с БД
    /// * `params` - параметры создаваемого `Shortlink`
    ///
    /// # Errors
    ///
    /// * `Error::BadRequest` - если переданный URL не является валидным URL
    /// * `Error::Model(ModelError::EntityAlreadyExists)` - если кастомный алиас уже существует
    /// * `Error::CustomError(StatusCode::INTERNAL_SERVER_ERROR)` - если не удалось сгенерировать уникальный `short_code`
    ///
    /// # Examples
    ///
    ///
    pub async fn create_link(db: &DatabaseConnection, params: &AddParams) -> Result<Self, Error> {
        // Валидация URL
        let url = Url::parse(&params.original_url).map_err(|e| Error::BadRequest(e.to_string()))?;

        // Проверка на заблокированные домены
        // Self::check_blocked_domains(url.host_str().unwrap_or_default()).await?;

        let short_code = match &params.custom_alias {
            Some(alias) => {
                // Проверка уникальности кастомного алиаса
                if Self::exists_by_alias(db, alias).await? {
                    return Err(Error::Model(ModelError::EntityAlreadyExists));
                }
                alias.clone()
            }
            None => {
                // Генерация уникального short_code
                let mut attempts = 0;
                loop {
                    let code = Self::generate_short_code();
                    if !Self::exists_by_code(db, &code).await? {
                        break code;
                    }
                    attempts += 1;
                    if attempts > 5 {
                        return Err(Error::CustomError(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            ErrorDetail::with_reason("Failed to generate unique code"),
                        ));
                    }
                }
            }
        };

        // Создание ActiveModel
        let mut link = ActiveModel {
            original_url: Set(Some(params.original_url.clone())),
            short_code: Set(short_code),
            custom_alias: Set(params.custom_alias.clone()),
            domain: Set(params.domain.clone()),
            password: Set(params.password.clone()),
            is_active: Set(Some(true)),
            ..Default::default()
        };

        // Сохранение в БД
        Ok(link.insert(db).await?)
    }

    async fn exists_by_code(db: &DatabaseConnection, code: &str) -> Result<bool, Error> {
        Ok(Entity::find()
            .filter(Column::ShortCode.eq(code))
            .count(db)
            .await?
            > 0)
    }

    async fn exists_by_alias(db: &DatabaseConnection, alias: &str) -> Result<bool, Error> {
        Ok(Entity::find()
            .filter(Column::CustomAlias.eq(alias))
            .count(db)
            .await?
            > 0)
    }

    async fn check_blocked_domains(domain: &str) -> Result<(), Error> {
        // Реализация проверки через таблицу BlockedDomain
        // ...
        todo!("Add BlockedDomain validation")
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}

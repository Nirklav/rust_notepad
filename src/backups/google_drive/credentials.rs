use std::fs::File;
use std::path::PathBuf;
use chrono::{Duration, Utc};
use serde::{Serialize, Deserialize};
use crate::{AppError, exe_directory};
use crate::backups::google_drive::token::Token;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    client_id: String,
    client_secret: String,
    token: Option<Token>
}

impl Credentials {
    pub fn load() -> Result<Self, AppError> {
        let path = Self::path()?;
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn update(&mut self, token: Token) -> Result<(), AppError> {
        self.token = Some(token);
        self.save()
    }

    pub fn update_access_token(&mut self, access_token: String, expires_in: i64) -> Result<(), AppError> {
        let mut token = self.token.as_mut().ok_or(AppError::internal("Token is empty"))?;
        let now = Utc::now();

        token.access_token = access_token;
        token.access_token_expired_at = now + Duration::seconds(expires_in);

        self.save()
    }

    fn save(&self) -> Result<(), AppError> {
        let path = Self::path()?;
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    fn path() -> Result<PathBuf, AppError> {
        Ok(exe_directory()?.join("credentials.json"))
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }

    pub fn access_token(&self) -> &str {
        if let Some(token) = &self.token {
            &token.access_token
        } else {
            ""
        }
    }

    pub fn refresh_token(&self) -> &str {
        if let Some(token) = &self.token {
            &token.refresh_token
        } else {
            ""
        }
    }

    pub fn is_token_empty(&self) -> bool {
        self.token.is_none()
    }

    pub fn is_token_expired(&self) -> bool {
        if let Some(token) = &self.token {
            let now = Utc::now();
            token.access_token_expired_at < now
        } else {
            false
        }
    }
}
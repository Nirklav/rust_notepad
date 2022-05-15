use chrono::{DateTime, Duration, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
    pub access_token_expired_at: DateTime<Utc>,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawToken {
    pub token_type: Option<String>,
    pub access_token: Option<String>,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    #[serde(default, alias = "x_refresh_token_expires_in")]
    pub refresh_token_expires_in: i64,
    pub scope: Option<String>,
}

impl RawToken {
    pub fn token(self) -> Option<Token> {
        if let Some(access_token) = self.access_token {
            if let Some(refresh_token) = self.refresh_token {
                let now = Utc::now();

                return Some(Token {
                    access_token,
                    refresh_token,
                    access_token_expired_at: now + Duration::seconds(self.expires_in)
                })
            }
        }

        None
    }
}
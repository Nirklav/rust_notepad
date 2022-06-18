use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use reqwest::blocking::Client;
use reqwest::{header};
use reqwest::header::HeaderMap;
use uuid::Uuid;
use crate::AppError;
use crate::backups::google_drive::credentials::Credentials;
use crate::backups::google_drive::GoogleDrive;
use crate::backups::google_drive::token::RawToken;

pub const TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";
pub const REDIRECT_ADDR : &str = "127.0.0.1:12675";
pub const SCOPES : &str = "https://www.googleapis.com/auth/drive.file";

impl GoogleDrive {
    pub fn new() -> Result<Self, AppError> {
        let mut drive = GoogleDrive {
            credentials: Credentials::load()?,
            client: Client::new()
        };

        if drive.credentials.is_token_empty() {
            drive.authenticate()?;
        } else {
            if let Err(AppError::GoogleDriveClientError(_)) = drive.refresh_access_token() {
                drive.authenticate()?;
            }
        }

        Ok(drive)
    }

    fn authenticate(&mut self) -> Result<(), AppError> {
        let user_consent_url = self.user_consent_url();
        open::that(user_consent_url)?;
        let redirect = Redirect::receive()?;
        self.access_token(&redirect.code, &redirect.state)?;
        Ok(())
    }

    fn user_consent_url(&self) -> String {
        format!("https://accounts.google.com/o/oauth2/v2/auth?client_id={}&response_type=code&redirect_uri=http://{}&state={}&scope={}",
            self.credentials.client_id(),
            REDIRECT_ADDR,
            Uuid::new_v4(),
            SCOPES)
    }

    fn access_token(&mut self, code: &str, state: &str) -> Result<(), AppError> {
        let mut headers = HeaderMap::new();
        headers.append(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );

        let creds = &self.credentials;
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("client_id", creds.client_id()),
            ("client_secret", creds.client_secret()),
            ("redirect_uri", &format!("http://{}", REDIRECT_ADDR)),
            ("state", state),
        ];

        let resp = self.client
            .post(TOKEN_ENDPOINT)
            .headers(headers)
            .form(&params)
            .basic_auth(creds.client_id(), Some(creds.client_secret()))
            .send()?;

        Self::assert_success(&resp)?;

        let raw: RawToken = resp.json()?;
        let token = raw.token().ok_or(AppError::internal("Receive: Token in response is empty"))?;
        self.credentials.update(token)?;
        Ok(())
    }

    pub fn refresh_access_token(&mut self) -> Result<(), AppError> {
        if self.credentials.is_token_empty() {
            return Err(AppError::internal("Token must be set in credentials to refresh token"));
        }

        if !self.credentials.is_token_expired() {
            return Ok(());
        }

        let mut headers = HeaderMap::new();
        headers.append(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );

        let creds = &self.credentials;
        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", creds.refresh_token()),
            ("client_id", creds.client_id()),
            ("client_secret", creds.client_secret()),
            ("redirect_uri", &format!("http://{}", REDIRECT_ADDR)),
        ];
        let resp = self.client
            .post(TOKEN_ENDPOINT)
            .headers(headers)
            .form(&params)
            .basic_auth(creds.client_id(), Some(creds.client_secret()))
            .send()?;

        Self::assert_success(&resp)?;

        let raw: RawToken = resp.json()?;
        let access_token = raw.access_token.ok_or(AppError::internal("Access token is empty"))?;
        self.credentials.update_access_token(access_token, raw.expires_in)?;
        Ok(())
    }
}

struct Redirect {
    pub code: String,
    pub state: String
}

impl Redirect {
    pub fn receive() -> Result<Self, AppError> {
        let listener = TcpListener::bind(REDIRECT_ADDR)?;
        let (mut stream, _) = listener.accept()?;

        let request = Self::read(&mut stream)?;
        Self::write(&mut stream)?;
        Self::parse(request)
    }

    fn read(stream: &mut TcpStream) -> Result<Vec<u8>, AppError> {
        let mut request = Vec::new();
        loop {
            const BUF_SIZE : usize = 4096;
            let mut buf = [0u8; BUF_SIZE];

            match stream.read(&mut buf) {
                Ok(val) => if val > 0 {
                    request.extend_from_slice(&buf[0..val]);
                    if val < BUF_SIZE {
                        return Ok(request)
                    }
                } else {
                    return Ok(request)
                },
                Err(_) => return Err(AppError::internal("Redirect server error"))
            };
        }
    }

    fn write(stream: &mut TcpStream) -> Result<(), AppError> {
        writeln!(stream, "HTTP/1.1 200 OK")?;
        writeln!(stream, "Server: notepad (rust)")?;
        writeln!(stream, "Content-Length: 2")?;
        writeln!(stream, "Content-Type: text/html")?;
        writeln!(stream, "Connection: Closed")?;
        writeln!(stream)?;
        writeln!(stream, "OK")?;
        Ok(())
    }

    fn parse(request: Vec<u8>) -> Result<Self, AppError> {
        let request = String::from_utf8(request)?;

        let get_line = request.split("\r\n")
            .find(|l| l.starts_with("GET"))
            .ok_or(AppError::internal("GET not found"))?;

        let query = get_line.split(' ')
            .find(|l| l.starts_with("/?"))
            .ok_or(AppError::internal("Query not found"))?;

        let mut code = None;
        let mut state = None;

        for pair in query[2..].split('&') {
            let mut split = pair.split('=');

            match split.next() {
                Some("code") => code = split.next(),
                Some("state") => state = split.next(),
                _ => { }
            }

            if let Some(code) = code {
                if let Some(state) = state {
                    return Ok(Redirect {
                        code: code.to_string(),
                        state: state.to_string()
                    })
                }
            }
        }

        Err(AppError::internal("code and state not found"))
    }
}
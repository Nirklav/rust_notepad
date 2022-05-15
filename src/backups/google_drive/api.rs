use std::fs::File;
use std::io::Write;
use std::path::Path;
use reqwest::header;
use reqwest::blocking::multipart::{Form, Part};
use reqwest::blocking::Response;
use serde_json::{json, Value};
use crate::{AppError, GoogleDrive};
use crate::backups::google_drive::id::Id;

impl GoogleDrive {
    pub fn list(&mut self, name: impl AsRef<str>, parent_id: Option<&Id>, is_folder: bool) -> Result<Vec<Id>, AppError> {
        self.refresh_access_token()?;

        let mut q = format!("name = '{}' and trashed = false", name.as_ref());
        if let Some(parent_id) = parent_id {
            q.push_str(&format!("and '{}' in parents", parent_id));
        }

        if is_folder {
            q.push_str("and mimeType = 'application/vnd.google-apps.folder'");
        }

        let url = "https://www.googleapis.com/drive/v3/files";
        let query = [
            ("q", q),
        ];

        let resp = self.client
            .get(url)
            .query(&query)
            .bearer_auth(self.credentials.access_token())
            .send()?;

        Self::assert_success(&resp)?;

        let json = resp.json::<Value>()?;
        let mut ids = Vec::new();
        let files = json["files"]
            .as_array()
            .ok_or(AppError::internal("Files not found in response"))?;

        for file in files {
            if let Some(id) = file["id"].as_str() {
                ids.push(Id::new(id));
            }
        }

        Ok(ids)
    }

    pub fn create_folder(&mut self, name: impl AsRef<str>) -> Result<Id, AppError> {
        self.refresh_access_token()?;

        let url = "https://www.googleapis.com/drive/v3/files";
        let metadata = json!({
            "name": name.as_ref(),
            "mimeType": "application/vnd.google-apps.folder"
        });
        let resp = self.client
            .post(url)
            .bearer_auth(self.credentials.access_token())
            .json(&metadata)
            .send()?;

        Self::assert_success(&resp)?;

        let json = resp.json::<Value>()?;
        let id = json["id"]
            .as_str()
            .ok_or(AppError::internal("Id of folder not found in successful request"))?;
        Ok(Id::new(id))
    }

    pub fn create_file(&mut self, name: impl AsRef<str>, parent_id: Option<&Id>, file_path: &Path) -> Result<(), AppError> {
        self.refresh_access_token()?;

        let url = "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart";
        let mut metadata = json!({
            "name": name.as_ref()
        });

        if let Some(parent_id) = parent_id {
            metadata["parents"] = json!([
                parent_id.as_ref()
            ]);
        }

        let form = Form::new()
            .part("Metadata", Part::text(serde_json::to_string(&metadata)?)
                .mime_str("application/json")?)
            .part("Media", Part::file(file_path)?
                .mime_str("application/octet-stream")?);

        let resp = self.client
            .post(url)
            .bearer_auth(self.credentials.access_token())
            .multipart(form)
            .send()?;

        Self::assert_success(&resp)?;

        let _json = resp.json::<Value>()?;
        Ok(())
    }

    pub fn update_file(&mut self, file_id: &Id, file_path: &Path) -> Result<(), AppError> {
        self.refresh_access_token()?;

        let file = File::open(file_path)?;
        let url = format!("https://www.googleapis.com/upload/drive/v3/files/{}?uploadType=media", file_id);
        let resp = self.client
            .patch(url)
            .bearer_auth(self.credentials.access_token())
            .header(header::CONTENT_TYPE, "application/octet-stream")
            .body(file)
            .send()?;

        Self::assert_success(&resp)?;

        let _json = resp.json::<Value>()?;
        Ok(())
    }

    pub fn download_file(&mut self, file_id: &Id, file_path: &Path) -> Result<(), AppError> {
        self.refresh_access_token()?;

        let mut file = File::create(file_path)?;
        let url = format!("https://www.googleapis.com/drive/v3/files/{}?alt=media", file_id);
        let resp = self.client
            .get(url)
            .bearer_auth(self.credentials.access_token())
            .send()?;

        Self::assert_success(&resp)?;

        let bytes = resp.bytes()?;
        file.write(&bytes)?;
        Ok(())
    }

    fn assert_success(resp: &Response) -> Result<(), AppError> {
        if !resp.status().is_success() {
            dbg!(&resp);
            return Err(AppError::internal("Request error"))
        }

        Ok(())
    }
}
#![allow(unused)]

use chrono::{DateTime, Utc};
use std::sync::{Arc, RwLock};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DocumentError {
    #[error("Failed to acquire lock: {0}")]
    LockError(String),
}

#[derive(Clone)]
pub struct DocumentMetadata {
    creation_date: DateTime<Utc>,
    last_modified: Arc<RwLock<DateTime<Utc>>>,
}

impl DocumentMetadata {
    fn new() -> Self {
        let now = Utc::now();
        Self {
            creation_date: now,
            last_modified: Arc::new(RwLock::new(now)),
        }
    }

    pub fn creation_date(&self) -> DateTime<Utc> {
        self.creation_date
    }

    pub fn last_modified(&self) -> Result<DateTime<Utc>, DocumentError> {
        let last_modified_read_guard = self
            .last_modified
            .read()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        Ok(*last_modified_read_guard)
    }

    fn update_last_modified(&mut self) -> Result<(), DocumentError> {
        let mut last_modified_write_guard = self
            .last_modified
            .write()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        *last_modified_write_guard = Utc::now();
        Ok(())
    }
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            creation_date: now,
            last_modified: Arc::new(RwLock::new(now)),
        }
    }
}

#[derive(Clone)]
pub struct Document {
    id: Uuid,
    name: Arc<RwLock<String>>,
    content: Arc<RwLock<String>>,
    history: Arc<RwLock<Vec<String>>>,
    metadata: DocumentMetadata,
}

impl Document {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: Arc::new(RwLock::new(name.to_owned())),
            content: Arc::new(RwLock::new(content.to_owned())),
            history: Arc::new(RwLock::new(vec![content.to_owned()])),
            metadata: DocumentMetadata::new(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn metadata(&self) -> &DocumentMetadata {
        &self.metadata
    }
}

impl Default for Document {
    fn default() -> Self {
        let empty_str = String::new();

        Self {
            id: Uuid::new_v4(),
            name: Arc::new(RwLock::new(empty_str.clone())),
            content: Arc::new(RwLock::new(empty_str.clone())),
            history: Arc::new(RwLock::new(vec![empty_str])),
            metadata: DocumentMetadata::new(),
        }
    }
}

impl Document {
    pub fn set_content(&mut self, content: &str) -> Result<(), DocumentError> {
        let mut content_write_guard = self
            .content
            .write()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;
        let mut history_write_guard = self
            .history
            .write()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        history_write_guard.push(content.to_owned());
        *content_write_guard = content.to_owned();

        self.metadata.update_last_modified()
    }

    pub fn content(&self) -> Result<String, DocumentError> {
        let content_read_guard = self
            .content
            .read()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        Ok(content_read_guard.clone())
    }

    pub fn name(&self) -> Result<String, DocumentError> {
        let name_read_guard = self
            .name
            .read()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        Ok(name_read_guard.clone())
    }

    pub fn history(&self) -> Result<Vec<String>, DocumentError> {
        let history_read_guard = self
            .history
            .read()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        Ok(history_read_guard.iter().cloned().collect())
    }

    pub fn set_name(&mut self, new_name: &str) -> Result<(), DocumentError> {
        let mut name_write_guard = self
            .name
            .write()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        *name_write_guard = new_name.to_owned();
        Ok(())
    }
}

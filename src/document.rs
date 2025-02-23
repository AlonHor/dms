use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};
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
    last_modified: Arc<Mutex<DateTime<Utc>>>,
}

impl DocumentMetadata {
    fn new() -> Self {
        let now = Utc::now();
        Self {
            creation_date: now,
            last_modified: Arc::new(Mutex::new(now)),
        }
    }

    pub fn creation_date(&self) -> DateTime<Utc> {
        self.creation_date
    }

    pub fn last_modified(&self) -> Result<DateTime<Utc>, DocumentError> {
        let last_modified_guard = self
            .last_modified
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        Ok(*last_modified_guard)
    }

    fn update_last_modified(&mut self) -> Result<(), DocumentError> {
        let mut last_modified_guard = self
            .last_modified
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        *last_modified_guard = Utc::now();
        Ok(())
    }
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            creation_date: now,
            last_modified: Arc::new(Mutex::new(now)),
        }
    }
}

#[derive(Clone)]
pub struct Document {
    id: Uuid,
    name: Arc<Mutex<String>>,
    content: Arc<Mutex<String>>,
    history: Arc<Mutex<Vec<String>>>,
    metadata: DocumentMetadata,
}

impl Document {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: Arc::new(Mutex::new(name.to_owned())),
            content: Arc::new(Mutex::new(content.to_owned())),
            history: Arc::new(Mutex::new(vec![content.to_owned()])),
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
            name: Arc::new(Mutex::new(empty_str.clone())),
            content: Arc::new(Mutex::new(empty_str.clone())),
            history: Arc::new(Mutex::new(vec![empty_str])),
            metadata: DocumentMetadata::new(),
        }
    }
}

impl Document {
    pub fn set_content(&mut self, content: &str) -> Result<(), DocumentError> {
        let mut content_guard = self
            .content
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;
        let mut history_guard = self
            .history
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        history_guard.push(content.to_owned());
        *content_guard = content.to_owned();

        self.metadata.update_last_modified()
    }

    pub fn content(&self) -> Result<String, DocumentError> {
        let content_guard = self
            .content
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        Ok(content_guard.clone())
    }

    pub fn name(&self) -> Result<String, DocumentError> {
        let name_guard = self
            .name
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        Ok(name_guard.clone())
    }

    pub fn history(&self) -> Result<Vec<String>, DocumentError> {
        let history_guard = self
            .history
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        Ok(history_guard.iter().cloned().collect())
    }

    pub fn set_name(&mut self, new_name: &str) -> Result<(), DocumentError> {
        let mut name_guard = self
            .name
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        *name_guard = new_name.to_owned();
        Ok(())
    }
}

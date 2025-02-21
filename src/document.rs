use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DocumentError {
    #[error("Failed to acquire lock: {0}")]
    LockError(String),
}

pub trait DocumentTrait {
    fn set_content(&mut self, new_content: &str) -> Result<(), DocumentError>;
    fn content(&self) -> Result<Arc<str>, DocumentError>;
    fn name(&self) -> Result<Arc<str>, DocumentError>;
    fn history(&self) -> Result<Vec<Arc<str>>, DocumentError>;
    fn set_name(&mut self, new_name: &str);
}

#[derive(Clone)]
pub struct DocumentMetadata {
    creation_date: DateTime<Utc>,
    last_modified: DateTime<Utc>,
}

impl DocumentMetadata {
    fn new() -> Self {
        let now = Utc::now();
        Self {
            creation_date: now,
            last_modified: now,
        }
    }

    pub fn creation_date(&self) -> DateTime<Utc> {
        self.creation_date
    }

    pub fn last_modified(&self) -> DateTime<Utc> {
        self.last_modified
    }

    fn update_last_modified(&mut self) {
        self.last_modified = Utc::now()
    }
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            creation_date: now,
            last_modified: now,
        }
    }
}

#[derive(Clone, Default)]
pub struct Document {
    id: Uuid,
    name: Arc<Mutex<String>>,
    content: Arc<Mutex<String>>,
    history: Arc<Mutex<Vec<String>>>,
    metadata: DocumentMetadata,
}

impl Document {
    pub fn new(name: &str, content: &str) -> Self {
        let instance = Self {
            id: Uuid::new_v4(),
            name: Arc::new(Mutex::new(name.to_owned())),
            content: Arc::new(Mutex::new(content.to_owned())),
            history: Arc::new(Mutex::new(vec![content.to_owned()])),
            metadata: DocumentMetadata::new(),
        };

        instance
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn metadata(&self) -> &DocumentMetadata {
        &self.metadata
    }
}

impl DocumentTrait for Document {
    fn set_content(&mut self, content: &str) -> Result<(), DocumentError> {
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
        self.metadata.update_last_modified();

        Ok(())
    }

    fn content(&self) -> Result<Arc<str>, DocumentError> {
        let content_guard = self
            .content
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;
        Ok(Arc::<str>::from(content_guard.clone()))
    }

    fn name(&self) -> Result<Arc<str>, DocumentError> {
        let name_guard = self
            .name
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;
        Ok(Arc::<str>::from(name_guard.clone()))
    }

    fn history(&self) -> Result<Vec<Arc<str>>, DocumentError> {
        let history_guard = self
            .history
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;
        Ok(history_guard.iter().map(|v| Arc::<str>::from(v.as_str())).collect())
    }

    fn set_name(&mut self, new_name: &str) {
        let mut name_guard = self
            .name
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string())).unwrap();
        *name_guard = new_name.to_owned();
    }
}

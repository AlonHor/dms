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
    fn set_name(&mut self, new_name: &str) -> Result<(), DocumentError>;
}

type SharedString = Arc<Mutex<Arc<str>>>;
type SharedHistory = Arc<Mutex<Vec<Arc<str>>>>;

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

#[derive(Clone)]
pub struct Document {
    id: Uuid,
    name: SharedString,
    content: SharedString,
    history: SharedHistory,
    metadata: DocumentMetadata,
}

impl Document {
    pub fn new(name: &str, content: &str) -> Self {
        let content_arc = Arc::from(content);

        let instance = Self {
            id: Uuid::new_v4(),
            name: Arc::new(Mutex::new(Arc::from(name))),
            content: Arc::new(Mutex::new(Arc::clone(&content_arc))),
            history: Arc::new(Mutex::new(vec![content_arc])),
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
    fn set_content(&mut self, new_content: &str) -> Result<(), DocumentError> {
        let content_arc = Arc::from(new_content);

        let mut content_guard = self
            .content
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;
        let mut history_guard = self
            .history
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;

        history_guard.push(Arc::clone(&content_arc));
        *content_guard = content_arc;
        self.metadata.update_last_modified();

        Ok(())
    }

    fn content(&self) -> Result<Arc<str>, DocumentError> {
        let content_guard = self
            .content
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;
        Ok(Arc::clone(&content_guard))
    }

    fn name(&self) -> Result<Arc<str>, DocumentError> {
        let name_guard = self
            .name
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;
        Ok(Arc::clone(&name_guard))
    }

    fn history(&self) -> Result<Vec<Arc<str>>, DocumentError> {
        let history_guard = self
            .history
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;
        Ok(history_guard.clone())
    }

    fn set_name(&mut self, new_name: &str) -> Result<(), DocumentError> {
        let mut name_guard = self
            .name
            .lock()
            .map_err(|e| DocumentError::LockError(e.to_string()))?;
        *name_guard = Arc::from(new_name);
        Ok(())
    }
}

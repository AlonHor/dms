use std::sync::{Arc, Mutex};

pub trait DocumentTrait {
    fn set_new_content<'a>(&mut self, new_content: &'a str) -> Result<&'a str, &'static str>;
    fn read_content(&self) -> Arc<str>;
    fn read_name(&self) -> Arc<str>;
    fn read_history(&self) -> Vec<Arc<str>>;
    fn set_name(&mut self, new_name: &str);
}

pub struct Document {
    pub id: uuid::Uuid,
    pub creation_date: chrono::NaiveDateTime,
    name: Arc<Mutex<Arc<str>>>,
    content: Arc<Mutex<Arc<str>>>,
    history: Arc<Mutex<Vec<Arc<str>>>>,
    last_modified: chrono::NaiveDateTime,
}

impl Document {
    pub fn new(name: &str, content: &str) -> Self {
        let instance = Self {
            id: uuid::Uuid::new_v4(),
            name: Arc::new(Mutex::new(Arc::from(name))),
            content: Arc::new(Mutex::new(Arc::from(content))),
            history: Arc::new(Mutex::new(Vec::new())),
            creation_date: chrono::Utc::now().naive_utc(),
            last_modified: chrono::Utc::now().naive_utc(),
        };

        let history_clone = Arc::clone(&instance.history);
        let mut history_lock = history_clone.lock().expect("Failed to acquire lock.");
        history_lock.push(Arc::from(content));

        instance
    }
}

impl DocumentTrait for Document {
    fn set_new_content<'a>(&mut self, new_content: &'a str) -> Result<&'a str, &'static str> {
        match self.content.clone().try_lock() {
            Err(_) => Err("Failed to acquire lock."),
            Ok(mut content_lock) => {
                let history_clone = Arc::clone(&self.history);

                let mut history_lock = history_clone.lock().expect("Failed to acquire lock.");
                history_lock.push(Arc::from(new_content));

                self.last_modified = chrono::Utc::now().naive_utc();

                *content_lock = Arc::from(new_content);
                Ok(new_content)
            }
        }
    }

    fn read_content(&self) -> Arc<str> {
        let content_lock = self.content.lock().expect("Failed to acquire lock.");
        let content = Arc::clone(&content_lock);

        content
    }

    fn read_name(&self) -> Arc<str> {
        let name_lock = self.name.lock().expect("Failed to acquire lock.");
        let name = name_lock.clone();

        name
    }

    fn read_history(&self) -> Vec<Arc<str>> {
        let mut history = Vec::new();

        let history_lock = self.history.lock().expect("Failed to acquire lock.");
        let history_vec = history_lock.clone();

        for version in history_vec {
            history.push(version)
        }
        
        history
    }

    fn set_name(&mut self, new_name: &str) {
        let name_clone = Arc::clone(&self.name);
        let mut name_lock = name_clone.lock().expect("Failed to acquire lock.");
        *name_lock = Arc::from(new_name);
    }
}

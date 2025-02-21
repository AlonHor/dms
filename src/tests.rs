#[cfg(test)]
mod tests {
    use crate::document::Document;
    use crate::document::DocumentTrait;

    #[test]
    fn test_document_creation() {
        let doc = Document::new("TestDoc", "TestContent");
        assert_eq!(doc.name().unwrap().as_ref(), "TestDoc");
        assert_eq!(doc.content().unwrap().as_ref(), "TestContent");
        assert_eq!(doc.history().unwrap().len(), 1);
    }

    #[test]
    fn test_set_content() {
        let mut doc = Document::new("TestDoc", "TestContent");
        doc.set_content("NewContent").unwrap();
        assert_eq!(doc.content().unwrap().as_ref(), "NewContent");
        assert_eq!(doc.history().unwrap().len(), 2);
    }

    #[test]
    fn test_set_name() {
        let mut doc = Document::new("Doc", "Content");
        doc.set_name("NewName");
        assert_eq!(doc.name().unwrap().as_ref(), "NewName");
    }
    
    #[test]
    fn test_metadata_no_modification() {
        let doc = Document::new("TestDoc", "Initial");
        assert_eq!(doc.metadata().creation_date(), doc.metadata().last_modified());
    }
    
    #[test]
    fn test_metadata_after_modification() {
        let mut doc = Document::new("TestDoc", "Initial");
        let creation = doc.metadata().creation_date();
        std::thread::sleep(std::time::Duration::from_millis(1));
        doc.set_content("Changed").unwrap();
        assert!(doc.metadata().last_modified() > creation);
    }
}

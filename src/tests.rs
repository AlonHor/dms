#[cfg(test)]
mod document_tests {
    use crate::document::Document;

    #[test]
    fn test_document_creation() {
        let doc = Document::new("TestDoc", "TestContent");
        assert_eq!(doc.name().expect("Failed to acquire lock"), "TestDoc");
        assert_eq!(
            doc.content().expect("Failed to acquire lock"),
            "TestContent"
        );
        assert_eq!(doc.history().expect("Failed to acquire lock").len(), 1);
    }

    #[test]
    fn test_set_content() {
        let mut doc = Document::new("TestDoc", "TestContent");
        doc.set_content("NewContent")
            .expect("Failed to acquire lock");
        assert_eq!(doc.content().expect("Failed to acquire lock"), "NewContent");
        assert_eq!(doc.history().expect("Failed to acquire lock").len(), 2);
    }

    #[test]
    fn test_set_name() {
        let mut doc = Document::new("Doc", "Content");
        doc.set_name("NewName").expect("Failed to acquire lock");
        assert_eq!(doc.name().expect("Failed to acquire lock"), "NewName");
    }

    #[test]
    fn test_metadata_no_modification() {
        let doc = Document::new("TestDoc", "Initial");
        assert_eq!(
            doc.metadata().creation_date(),
            doc.metadata()
                .last_modified()
                .expect("Failed to acquire lock")
        );
    }

    #[test]
    fn test_metadata_after_modification() {
        let mut doc = Document::new("TestDoc", "Initial");
        let creation = doc.metadata().creation_date();
        std::thread::sleep(std::time::Duration::from_millis(1));
        doc.set_content("Changed").expect("Failed to acquire lock");
        assert!(
            doc.metadata()
                .last_modified()
                .expect("Failed to acquire lock")
                > creation
        );
    }

    #[test]
    fn test_document_default() {
        let doc = Document::default();
        assert_eq!(doc.name().expect("Failed to acquire lock"), "");
        assert_eq!(doc.content().expect("Failed to acquire lock"), "");
        assert_eq!(doc.history().expect("Failed to acquire lock").len(), 1);
    }

    #[test]
    fn test_document_clone() {
        let original = Document::new("CloneMe", "Content");
        let cloned = original.clone();
        assert_eq!(
            original.name().expect("Failed to acquire lock"),
            cloned.name().expect("Failed to acquire lock")
        );
        assert_eq!(
            original.content().expect("Failed to acquire lock"),
            cloned.content().expect("Failed to acquire lock")
        );
    }

    #[test]
    fn test_document_metadata_default() {
        let meta = crate::document::DocumentMetadata::default();
        assert_eq!(
            meta.creation_date(),
            meta.last_modified().expect("Failed to acquire lock")
        );
    }
}

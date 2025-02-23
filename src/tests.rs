#[cfg(test)]
mod document_tests {
    use crate::document::Document;

    #[test]
    fn test_document_creation() {
        let doc = Document::new("TestDoc", "TestContent");
        assert_eq!(doc.name().expect("Failed to lock name"), "TestDoc");
        assert_eq!(
            doc.content().expect("Failed to lock content"),
            "TestContent"
        );
        assert_eq!(doc.history().expect("Failed to lock history").len(), 1);
    }

    #[test]
    fn test_set_content() {
        let mut doc = Document::new("TestDoc", "TestContent");
        doc.set_content("NewContent")
            .expect("Failed to lock content");
        assert_eq!(doc.content().expect("Failed to lock content"), "NewContent");
        assert_eq!(doc.history().expect("Failed to lock history").len(), 2);
    }

    #[test]
    fn test_set_name() {
        let mut doc = Document::new("Doc", "Content");
        doc.set_name("NewName").expect("Failed to lock name");
        assert_eq!(doc.name().expect("Failed to lock name"), "NewName");
    }

    #[test]
    fn test_metadata_no_modification() {
        let doc = Document::new("TestDoc", "Initial");
        assert_eq!(
            doc.metadata().creation_date(),
            doc.metadata()
                .last_modified()
                .expect("Failed to lock last_modified")
        );
    }

    #[test]
    fn test_metadata_after_modification() {
        let mut doc = Document::new("TestDoc", "Initial");
        let creation = doc.metadata().creation_date();
        std::thread::sleep(std::time::Duration::from_millis(1));
        doc.set_content("Changed").expect("Failed to lock content");
        assert!(
            doc.metadata()
                .last_modified()
                .expect("Failed to lock last_modified")
                > creation
        );
    }

    #[test]
    fn test_document_default() {
        let doc = Document::default();
        assert_eq!(doc.name().expect("Failed to lock name"), "");
        assert_eq!(doc.content().expect("Failed to lock content"), "");
        assert_eq!(doc.history().expect("Failed to lock history").len(), 1);
    }

    #[test]
    fn test_document_clone() {
        let original = Document::new("CloneMe", "Content");
        let cloned = original.clone();
        assert_eq!(
            original.name().expect("Failed to lock name"),
            cloned.name().expect("Failed to lock name")
        );
        assert_eq!(
            original.content().expect("Failed to lock content"),
            cloned.content().expect("Failed to lock content")
        );
    }

    #[test]
    fn test_document_metadata_default() {
        let meta = crate::document::DocumentMetadata::default();
        assert_eq!(
            meta.creation_date(),
            meta.last_modified().expect("Failed to lock last_modified")
        );
    }
}

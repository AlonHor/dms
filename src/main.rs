use crate::document::{Document, DocumentTrait};
use actix_web::{get, post, web, App, HttpServer, Responder};
use std::collections::HashMap;
use std::sync::{Mutex};
use uuid::Uuid;

mod document;
mod tests;

struct DocumentDb {
    docs: Mutex<HashMap<Uuid, Document>>,
}

impl DocumentDb {
    fn new() -> Self {
        Self {
            docs: HashMap::new().into(),
        }
    }

    async fn find_doc(&self, uuid_str: &str) -> Result<Document, &'static str> {
        let parsed = Uuid::parse_str(uuid_str).map_err(|_| "Invalid UUID")?;
        let docs = self.docs.lock().map_err(|_| "Couldn't lock documents")?;
        docs.get(&parsed).cloned().ok_or("Not found")
    }

    async fn add_doc(&self, document: Document) -> Result<(), &'static str> {
        let mut docs = self.docs.lock().map_err(|_| "Couldn't lock documents")?;
        docs.insert(document.id(), document);
        Ok(())
    }
}

#[get("/doc/{uuid}")]
async fn get_doc(server: web::Data<DocumentDb>, uuid: web::Path<String>) -> impl Responder {
    match server.find_doc(&uuid).await {
        Ok(doc) => match doc.content() {
            Ok(content) => {
                format!("Document content: {}", content)
            }
            Err(e) => format!("Error: {}", e),
        },
        Err(e) => format!("Error: {}", e),
    }
}

#[derive(serde::Deserialize)]
struct CreateDocumentRequest {
    name: String,
    content: String,
}

#[post("/doc")]
async fn create_doc(
    server: web::Data<DocumentDb>,
    body: web::Json<CreateDocumentRequest>,
) -> impl Responder {
    let doc = Document::new(&body.name, &body.content);
    let doc_id = doc.id();
    match server.add_doc(doc).await {
        Ok(_) => format!("Document created with ID: {}", doc_id),
        Err(e) => format!("Error: {}", e),
    }
}

#[derive(serde::Deserialize)]
struct EditDocumentRequest {
    content: String,
}

#[post("/doc/{uuid}")]
async fn edit_doc(
    server: web::Data<DocumentDb>,
    uuid: web::Path<String>,
    body: web::Json<EditDocumentRequest>,
) -> impl Responder {
    match server.find_doc(&uuid).await {
        Ok(mut doc) => match doc.set_content(body.content.as_ref()) {
            Ok(_) => "Content changed".to_owned(),
            Err(e) => format!("Error: {}", e),
        },
        Err(e) => format!("Error: {}", e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(DocumentDb::new());
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(get_doc)
            .service(create_doc)
            .service(edit_doc)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

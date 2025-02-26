use crate::document::Document;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;
use thiserror::Error;

mod document;
mod tests;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Invalid UUID")]
    InvalidUuid,
    #[error("Couldn't lock documents")]
    LockError,
    #[error("Not found")]
    NotFound,
}

struct DocumentDb {
    docs: RwLock<HashMap<Uuid, Document>>,
}

impl DocumentDb {
    fn new() -> Self {
        Self {
            docs: HashMap::new().into(),
        }
    }

    async fn find_doc(&self, uuid_str: &str) -> Result<Document, ServerError> {
        let parsed = Uuid::parse_str(uuid_str).map_err(|_| ServerError::InvalidUuid)?;
        let docs = self.docs.read().map_err(|_| ServerError::LockError)?;
        docs.get(&parsed).cloned().ok_or(ServerError::NotFound)
    }

    async fn add_doc(&self, document: Document) -> Result<(), ServerError> {
        let mut docs = self.docs.write().map_err(|_| ServerError::LockError)?;
        docs.insert(document.id(), document);
        Ok(())
    }
}

fn get_error_json(error: String) -> serde_json::Value {
    serde_json::json!({ "error": error })
}

#[get("/doc/{uuid}")]
async fn get_doc(server: web::Data<DocumentDb>, uuid: web::Path<String>) -> HttpResponse {
    match server.find_doc(&uuid).await {
        Ok(doc) => {
            let content = match doc.content() {
                Ok(c) => c,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(get_error_json(e.to_string()))
                }
            };

            let name = match doc.name() {
                Ok(n) => n,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(get_error_json(e.to_string()))
                }
            };

            HttpResponse::Ok().json(serde_json::json!({
                "name": name,
                "content": content,
            }))
        }
        Err(e) => HttpResponse::BadRequest().json(get_error_json(e.to_string())),
    }
}

#[derive(serde::Deserialize)]
struct CreateDocumentRequest {
    name: Option<String>,
    content: Option<String>,
}

#[post("/doc")]
async fn create_doc(
    server: web::Data<DocumentDb>,
    body: web::Json<CreateDocumentRequest>,
) -> HttpResponse {
    let content = body.content.as_deref().unwrap_or("");
    let name = body.name.as_deref().unwrap_or("Untitled");

    let doc = Document::new(name, content);
    let doc_id = doc.id();
    match server.add_doc(doc).await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({ "id": doc_id.to_string() })),
        Err(e) => HttpResponse::InternalServerError().json(get_error_json(e.to_string())),
    }
}

#[derive(serde::Deserialize)]
struct EditDocumentRequest {
    content: Option<String>,
    name: Option<String>,
}

#[post("/doc/{uuid}")]
async fn edit_doc(
    server: web::Data<DocumentDb>,
    uuid: web::Path<String>,
    body: web::Json<EditDocumentRequest>,
) -> HttpResponse {
    match server.find_doc(&uuid).await {
        Ok(mut doc) => {
            if let Some(c) = &body.content {
                if let Err(e) = doc.set_content(c) {
                    return HttpResponse::InternalServerError().json(get_error_json(e.to_string()));
                }
            }

            if let Some(n) = &body.name {
                if let Err(e) = doc.set_name(n) {
                    return HttpResponse::InternalServerError().json(get_error_json(e.to_string()));
                }
            }

            let name = match doc.name() {
                Ok(n) => n,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(get_error_json(e.to_string()));
                }
            };

            let content = match doc.content() {
                Ok(c) => c,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(get_error_json(e.to_string()));
                }
            };

            HttpResponse::Ok().json(serde_json::json!({ "name": name, "content": content }))
        }
        Err(e) => HttpResponse::BadRequest().json(get_error_json(e.to_string())),
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

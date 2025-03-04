use crate::document::Document;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;
use thiserror::Error;
use uuid::Uuid;

mod document;
mod tests;

lazy_static! {
    static ref lazy_docs: RwLock<HashMap<Uuid, Document>> = HashMap::new().into();
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Invalid UUID")]
    InvalidUuid,
    #[error("Couldn't lock documents")]
    LockError,
    #[error("Not found")]
    NotFound,
}

async fn find_doc(uuid_str: &str) -> Result<Document, ServerError> {
    let parsed = Uuid::parse_str(uuid_str).map_err(|_| ServerError::InvalidUuid)?;
    let docs = lazy_docs.read().map_err(|_| ServerError::LockError)?;
    docs.get(&parsed).cloned().ok_or(ServerError::NotFound)
}

async fn add_doc(document: Document) -> Result<(), ServerError> {
    let mut docs = lazy_docs.write().map_err(|_| ServerError::LockError)?;
    docs.insert(document.id(), document);
    Ok(())
}

fn get_error_json(error: String) -> serde_json::Value {
    serde_json::json!({ "error": error })
}

#[get("/doc/{uuid}")]
async fn get_doc(uuid: web::Path<String>) -> HttpResponse {
    match find_doc(&uuid).await {
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
    body: web::Json<CreateDocumentRequest>,
) -> HttpResponse {
    let content = body.content.as_deref().unwrap_or("");
    let name = body.name.as_deref().unwrap_or("Untitled");

    let doc = Document::new(name, content);
    let doc_id = doc.id();
    match add_doc(doc).await {
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
    uuid: web::Path<String>,
    body: web::Json<EditDocumentRequest>,
) -> HttpResponse {
    match find_doc(&uuid).await {
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
    HttpServer::new(move || {
        App::new()
            .service(get_doc)
            .service(create_doc)
            .service(edit_doc)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

use crate::backend::{Compose, DatabaseEntity, ComposeDb};

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::Route;

pub(crate) fn routes() -> Vec<Route> {
    routes![index, get, compose]
}

#[get("/?<limit>&<page>&<all>")]
async fn index(
    page: Option<usize>,
    limit: Option<usize>,
    all: Option<bool>,
) -> Result<Json<Vec<Compose>>, Status> {
    let composes = if all.unwrap_or(false) {
        Compose::list_all()
            .await
            .map_err(|_| Status::InternalServerError)
            .unwrap()
    } else {
        Compose::list(limit.unwrap_or(100), page.unwrap_or(0))
            .await
            .map_err(|_| Status::InternalServerError)
            .unwrap()
    };
    Ok(Json(composes))
}

#[get("/<id>")]
async fn get(id: Uuid) -> Option<Json<Compose>> {
    Compose::get(id).await.map(Json).ok()
}


#[post("/<id>")]
async fn compose (id: Uuid) -> Result<Json<Compose>, Status> {
    let compose = Compose::compose(id).await.map(Json).ok().unwrap();
    Ok(compose)
}
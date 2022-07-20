#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_dyn_templates;
use rocket::fs::FileServer;
use rocket::http::ContentType;
use rocket::response::status;
use rocket_dyn_templates::Template;
use sea_orm_rocket::Database;
mod api;
mod artifacts;
mod auth;
mod db;
mod db_object;
mod entity;
mod pkgs;
mod repos;
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm_rocket::Database;
mod artifacts;
mod backend;
mod entity;

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            foo: 123,
        },
    )
}

#[get("/favicon.png")]
fn favicon() -> (ContentType, &'static Vec<u8>) {
    (ContentType::PNG, include_bytes!("favicon.png"))
}

#[launch]
async fn rocket() -> _ {
    match db::setup_db().await {
        Ok(db) => db,
        Err(e) => panic!("{}", e),
    };

    rocket::build()
        .attach(db::Db::init())
        .mount("/", routes![index])
        .mount(
            "/static",
            FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../static")),
        )
        .mount(
            "/assets",
            FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets")),
        )
        .attach(Template::fairing())
        .mount("/builds", api::builds_routes())
        .mount("/artifacts", api::artifacts_routes())
}

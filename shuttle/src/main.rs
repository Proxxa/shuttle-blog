use rocket::{fs::{FileServer, Options, relative, NamedFile}, catch, Request, catchers, routes};
mod api;

#[catch(404)]
async fn not_found(_req: &Request<'_>) -> NamedFile {
    NamedFile::open(relative!("static/index.html")).await.ok().unwrap()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", FileServer::new(relative!("static"), Options::Index).rank(-999))
        .mount("/api", routes![api::hello])
        .register("/", catchers![not_found]);

    Ok(rocket.into())
}

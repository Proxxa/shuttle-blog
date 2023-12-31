use std::collections::HashMap;

use rocket::{
    catch, catchers,
    fs::{relative, FileServer, NamedFile, Options},
    futures::lock::Mutex,
    get, routes,
    time::{Duration, Instant},
    Request,
};
mod api;

#[catch(404)]
async fn not_found(_req: &Request<'_>) -> NamedFile {
    NamedFile::open(relative!("static/index.html"))
        .await
        .ok()
        .unwrap()
}

#[get("/")]
async fn home() -> NamedFile {
    NamedFile::open(relative!("static/index.html"))
        .await
        .ok()
        .unwrap()
}

#[get("/blog/<_blogid>")]
async fn home_blog_override(_blogid: Option<usize>) -> NamedFile {
    home().await
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![home, home_blog_override])
        .mount(
            "/assets",
            FileServer::new(relative!("static/assets"), Options::Index),
        )
        .mount(
            "/public",
            FileServer::new(relative!("static"), Options::Index),
        )
        .mount(
            "/api",
            routes![api::hello, api::blogs, api::blog_content, api::blog_data],
        )
        .manage(api::BlogPosts {
            last: Mutex::new(Instant::now() - Duration::DAY),
            list: Mutex::new(HashMap::new()),
        })
        .register("/", catchers![not_found]);

    Ok(rocket.into())
}

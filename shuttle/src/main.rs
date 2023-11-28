use rocket::{fs::{FileServer, Options, relative, NamedFile}, catch, Request, catchers, routes, get};
mod api;

#[catch(404)]
async fn not_found(_req: &Request<'_>) -> NamedFile {
    NamedFile::open(relative!("static/index.html")).await.ok().unwrap()
}

#[get("/")]
async fn home() -> NamedFile {
    NamedFile::open(relative!("static/index.html")).await.ok().unwrap()
}

#[get("/blog/<_blogid>")]
async fn home_blog_override(_blogid: Option<usize>) -> NamedFile {
    home().await
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![home, home_blog_override])
        .mount("/blog", routes![home])
        .mount("/", FileServer::new(relative!("static"), Options::Index).rank(-999))
        .mount("/api", routes![api::hello])
        .register("/", catchers![not_found]);

    Ok(rocket.into())
}

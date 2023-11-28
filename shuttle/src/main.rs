use rocket::fs::{FileServer, Options, relative};

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", FileServer::new(relative!("static"), Options::Index).rank(-999));

    Ok(rocket.into())
}

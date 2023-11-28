use rocket::get;


#[get("/hello")]
pub fn hello() -> String {
    "Hello, World!".to_owned()
}
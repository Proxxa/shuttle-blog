use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use rocket::{
    fs::NamedFile,
    futures::lock::Mutex,
    get,
    http::Status,
    serde::json::Json,
    time::{ext::NumericalDuration, Instant},
    State,
};

use serde::{Deserialize, Serialize};
use tracing::warn;

#[get("/hello")]
pub fn hello() -> String {
    "Hello, World!".to_owned()
}

pub struct BlogPosts {
    pub last: Mutex<Instant>,
    pub list: Mutex<HashMap<String, BlogPost>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BlogMeta {
    pub id: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub image: Option<String>,
    pub ordering: usize,
}

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct BlogPost {
    pub filepath: PathBuf,
    pub meta: BlogMeta,
}

const NOT_FOUND_STATUS: Status = Status { code: 404 };
const INTERNAL_ERROR_STATUS: Status = Status { code: 500 };
const CARGO_MANIFEST: &str = env!("CARGO_MANIFEST_DIR");

/// Return a list of all blog posts with metadata.
///
#[get("/blogs")]
pub async fn blogs(
    post_state: &State<BlogPosts>,
) -> Result<Json<HashMap<String, BlogPost>>, Status> {
    if post_state.last.lock().await.elapsed() < 5.minutes() {
        Ok(Json(post_state.list.lock().await.clone()))
    } else {
        let mut post_list_lock = post_state.list.lock().await;
        let mut post_last_lock = post_state.last.lock().await;

        let posts = std::fs::read_dir(rocket::fs::relative!("static/blogs"))
            .map(|rd| {
                let mut posts: HashMap<String, BlogPost> = HashMap::new();
                for ed in rd {
                    let Some(ed) = ed.ok() else { continue };
                    if !ed.path().is_dir() { continue }
                    let mut buffer = String::new();
                    let Ok(mut file) = File::open(ed.path().join("meta.json")) else { continue };
                    let Ok(_) = file.read_to_string(&mut buffer) else {
                        warn!("Cannot read blog meta file {}", ed.path().join("meta.json").display());
                        continue
                    };

                    let Ok(meta) = serde_json::from_str::<BlogMeta>(&buffer) else {
                        warn!("Cannot parse blog meta file {}", ed.path().join("meta.json").display());
                        continue
                    };

                    let filepath = ed.path();
                    let Ok(filepath) = filepath.strip_prefix(CARGO_MANIFEST) else {
                        warn!("Cannot remove {CARGO_MANIFEST} from {}", ed.path().display());
                        continue
                    };
                    posts.insert(meta.id.clone(), BlogPost { filepath: filepath.join("post.md"), meta });
                }
                posts
            })
            .expect("Failed to get blogs folder. Does it exist?");

        *post_list_lock = posts.clone();
        *post_last_lock = Instant::now();
        Ok(Json(posts))
    }
}

#[get("/blog/<id>")]
pub async fn blog_content(id: &str, post_state: &State<BlogPosts>) -> Result<NamedFile, Status> {
    let blogs = blogs(post_state)
        .await
        .expect("Failed to get blogs. Does the blogs folder exist?");
    let post = blogs.get(id).ok_or(NOT_FOUND_STATUS)?;
    NamedFile::open(post.filepath.clone()).await.map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            warn!("Could not find `{}`", post.filepath.display());
            NOT_FOUND_STATUS
        } else {
            INTERNAL_ERROR_STATUS
        }
    })
}

#[get("/blogdata/<id>")]
pub async fn blog_data(id: &str, post_state: &State<BlogPosts>) -> Result<Json<BlogMeta>, Status> {
    let blogs = blogs(post_state)
        .await
        .expect("Failed to get blogs. Does the blogs folder exist?");
    let post = blogs.get(id).ok_or_else(|| {
        warn!("No such blog `{id}`");
        NOT_FOUND_STATUS
    })?;
    Ok(Json(post.meta.clone()))
}

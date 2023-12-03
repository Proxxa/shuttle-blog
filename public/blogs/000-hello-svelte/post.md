#### Table of Contents

- [Start](#start)
- [Configuration](#configuration)
- [Routing](#routing)
- [Blogs](#blogs)
  - [Layout](#layout)
  - [API](#api)
  - [API Routing](#api-routing)
    - [API Testing](#api-testing)
  - [Blog Frontend](#blog-frontend)
- [Extra Pages](#extra-pages)
- [Final Testing](#final-testing)
- [Reflection](#reflection)

### Introduction

Hello, Shuttle! This is Yori. Let's talk about how I made this website. It's not a super interesting story, but it's a story.
If you want to just see the source code, head over to [GitHub][this@github]. If you want to know a little more about it, look
no further than the [homepage][home]. Obviously, **this blog post will assume understanding of [Rust][rustlang] and [Shuttle][shuttlers].**
If you are not familiar with either, a lot of this will go over your head.

Alright? Alright. Let's go.

##### Updates as of Nov. 30, 2023

This has been updated for links that are `inline code` to be bolded like **`this`**.

<h2 class="shuttletext" id="the-inspiration">
The Inspiration
</h2>

Yeah, the shuttle logo gradient doesn't look good on text. I've wanted to use [Svelte][svelte] as a framework for a while, but I
haven't had a reason to. Come November 2023 and Shuttle is running a [Christmas Code Hunt][cch] akin to [Advent of Code][aoc].
What's more, Jeff Mitchell posts [a dedicated page for the hunt][jeffmitchelldiary] and makes the main page a diary about the event,
and the project is apparently using Hyper and Tower. Goodness, Jeff, I do not want to be in your shoes. Push comes to shove, and
my desire to finally use Svelte wins, so I decide to make an entire site dedicated to shuttle blogging. Who knows, maybe someone
would want to read.

## Start

Unlike most [Shuttle][shuttlers] projects, this one doesn't start with `cargo shuttle init`. That's because we need [Vite][vite]/[Svelte][svelte]
to actually build, and there's no guarantee with Shuttle that our server will have [`npm`][npm] installed. Even then, there's no guarantee
that we'll be able to write to disk, though if we can't, I personally wouldn't use that server vendor for long. Because there's no
guarantee that we can run `npm install` in our `build.rs` script, we instead need our Shuttle project to be a subfolder of our Svelte
project.

First, let's run `npm create vite@latest` and create a Svelte/JS project. TypeScript would work as well, but do not use SvelteKit.
To my current knowledge, SvelteKit builds a server itself, so unless we want to write an API to access an internal SvelteKit server,
we shouldn't use SvelteKit. I'll try it sometime, but not right now. If I try it, and it works, I might write a blog on it. Next,
let's run `cargo shuttle init`. We'll place it at `project/shuttle`, next to the `project/src` created by `npm create`. I'll use
Rocket, since that's what I'm used to. If all goes accordingly, then the file tree should look something like this:

![A VSCode File Explorer](/public/blogs/000-hello-svelte/image.png)

## Configuration

We have to configure our build steps some more. Let's take a look at `project/vite.config.js`. It's very bare right now, so we'll
make it have JSDoc types for autocompletion and a `build` configuration key.

```js
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
/**
 * @type {import('vite').UserConfig}
 */
export default defineConfig({
  plugins: [svelte()],
  build: {
    outDir: "shuttle/static",
    copyPublicDir: true,
    chunkSizeWarningLimit: 900,
  },
  optimizeDeps: {
    exclude: ["svelte-navigator"]
  },
})
```

While we're at it, let's give our `package.json` some helpful development scripts.

```json
{
  "name": "shuttle-x-svelte",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {
    "run": "vite build && cargo shuttle run --wd shuttle",
    "live": "vite build --watch & cargo watch -C shuttle -x 'shuttle run' ; kill %1 ; kill %2",
    "build": "vite build",
    "preview": "vite preview",
    "deploy": "vite build && cargo shuttle deploy --ad --wd shuttle"
  },
  "devDependencies": {
    "@sveltejs/vite-plugin-svelte": "^3.0.0",
    "svelte": "^4.2.3",
    "vite": "^5.0.0"
  }
}
```

Awesome. Now we can have an *almost* liveserver. Note that I say almost, because that requires more work
inside Rust. Now, our `project/shuttle/Cargo.toml` needs some dependencies.

```toml
[package]
name = "shuttle-x-svelte"
version = "0.1.0"
edition = "2021"

[dependencies]
rocket = "0.5.0"
shuttle-rocket = "0.34.0"
shuttle-runtime = "0.34.0"
tokio = "1.26.0"
```

Perfect. This will do for now, but we'll probably need much more in the future.

## Routing

Svelte is fantastic, but it's no SvelteKit. SvelteKit would automatically create routing for me, but I'm using
Svelte. I need packages. There's no way I'm manully editing `project/package.json` again, so I run 
`npm install --save-dev --force svelte-navigator`. That installs [`svelte-navigator`][svelte-navigator]. For some reason, I need `--force` to install it, but it'll
do us wonders. Let's look at `project/src/App.svelte`. I know I'm going to need a navigation bar, a `/blog` route,
a `/blog/<id>` route, and probably an `/api` route just to be nice. Oh, and I need a home page. Can't forget a
homepage.

```svelte
<script>
    // @ts-ignore
    import { Router, Link, Route } from "svelte-navigator";
	import Home from "./routes/Home.svelte";
    import NotFound from "./routes/404.svelte";
    import GithubLink from "./lib/GithubLink.svelte";
    import favicon from "./assets/favicon.png";
    import ApiLandingPage from "./routes/ApiLandingPage.svelte";
    import Blog from "./routes/Blog.svelte";

    /**
     * @type {HTMLElement}
     */
    let navbar;
    let scrollPos = window.scrollY;
    function onScroll() {
        let newScrollPos = window.scrollY
        if (scrollPos > newScrollPos)
            navbar.style.top = "0";
        else
            navbar.style.top = `${-navbar.clientHeight}px`;
        scrollPos = newScrollPos
        console.log(scrollPos > newScrollPos)
    }
</script>

<svelte:window on:scroll={onScroll}/>

<head>
    <link rel="icon" type="image/png" href={favicon} />
</head>

<Router>
    <nav bind:this={navbar}>
        <Link to="/">Home</Link>
        <Link to="blog">Blog</Link>
        <span class="navSpacer"></span>
        <GithubLink/>    
    </nav>

    <Route path="blog/:blogId" component="{Blog}"/>
    <Route path="blog" component="{Blog}"/>
    <Route path="/api" component="{ApiLandingPage}"/>
    <Route path="/" component="{Home}"/>
    <Route path="" component="{NotFound}"/>
</Router>
```

The `<Link>` tags are great, but every time I try to close them, it autofills `</nav>` because my IDE wants
to be helpful. Alright, I need CSS. Luckily, `project/src/app.css` is preloaded for me because Svelte's app
creator automatically includes it. So, what does that `nav` bar look like?

```css
#app {
    padding-top: 1em;
}

#app > nav {
    display: flex;
    position: fixed;
    top: 0;
    left: 0;
    padding: 0;
    width: 100%;
    max-height: max-content;
    background-color: var(--nav-background);
    --border-bottom-radius: 0.5em;
    border-bottom-left-radius: var(--border-bottom-radius);
    border-bottom-right-radius: var(--border-bottom-radius);
    transition: top 0.3s;

}

/* Navbar links. */
#app > nav a {
    margin: 0.35em;
    margin-bottom: 0.5em;
    padding: 0.25em;
    text-decoration-line: none;
    color: var(--link-color);
    font-weight: 600;
    border-radius: 0.5em;

    transition: background-color 0.25s;
    background-color: rgba(255, 255, 255, 0);

    transform: scale(1, 1);
}

/* For accessibility features */
#app > nav a:focus {
    transition: transform 0.1s;
    transform: scale(1.2, 1.2);
    text-decoration-line: underline;
    text-decoration-thickness: 10%;
    outline: none;
}

/* Neat little background hover. Fades in slower than it fades out. */
#app > nav a:hover {
    transition: background-color 0.5s;
    background-color: rgba(255, 255, 255, 0.3);
    transition: transform 0.1s;
    transform: scale(1.2, 1.2);
}
```

Wow, these codeblocks are big. To give ourselves something to look at, let's go ahead and create each of those `.svelte` files
and write a home page.

```svelte
<script>
    import GithubLink from "../lib/GithubLink.svelte";
    import ShuttleLink from "../lib/ShuttleLink.svelte";
</script>

<main>
    <div class="blog">
        
        <h1>
            Proxxa's Shuttle Blog
        </h1>
        <p>
            Hi, I'm Yori. This is my blog on things I do with <ShuttleLink />.
            This is probably not going to be updated much, but I might write a bit with the <ShuttleLink path="cch" text="Christmas Code Hunt"/>.
            You can expect this to change a bit if I start posting... a lot. This site is made with <a href="https://svelte.dev/">Svelte</a>, and
            this site is also an excuse to learn Svelte. You can see the source code on <GithubLink/>. 
        </p>
    </div>
</main>
```

```svelte
<!-- ShuttleLink.svelte -->
<script>
    export let path = "";
    export let text = "Shuttle";
</script>

<a href="https://shuttle.rs/{path}">{text}</a>
```

```svelte
<!-- GithubLink.svelte -->
<script>
    export let owner = "proxxa";
    export let name = "shuttle-blog";
    export let path = "";
    export let text = "GitHub";
</script>

<a href="https://github.com/{owner}/{name}/{path}">{text}</a>
```

![An image of the blog's homepage](/public/blogs/000-hello-svelte/image-1.png)


Now that's beautiful. What's next? Oh. Blogs.

## Blogs

### Layout

Alright, I need an API to give me a list of blog posts and another to just return the content of the blog posts.
I think I'll set up a `project/public/blogs` folder that will get copied into `project/static/blogs` for me every
time I build Svelte. I'll also have folders inside this one—the name doesn't matter as long as I can recognize it.
I want two files at least: `project/public/blogs/postname/post.md` and `project/public/blogs/postname/meta.json`.
I can render the markdown in real time, but I'd prefer to have parsed access to `meta.json` so I can call the API
function for that information if I ever need it. Hm... I'd love to create a search functionality, but that can
wait until I have a good number of blog posts. Here's what I'll have, then:

* `GET /api/blogs` to return a mapping of blog post IDs to content file paths and metadata.
* `GET /api/blog/<id>` to return the content file of a blog post.
* `GET /api/blogdata/<id>` to return the metadata of a blog post.
* A `Blog.svelte` file that handles every part of the blog.
* A `LinkList.svelte` file just to make lists of links easier.

### API

Let's make those endpoints in a module named `api`. First, it looks like I'll need `rocket`'s `json` feature,
and I'll need [`serde`][serde], [`serde_json`][serde_json], and [`tracing`][tracing-rs]. I run `cargo add rocket -F json` to add the json feature,
and then I run `cargo add serde serde_json tracing` to get my other dependencies.

```rs
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
}

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct BlogPost {
    pub filepath: PathBuf,
    pub meta: BlogMeta,
}
```

Some data to hold information. `BlogPosts` is a cache that Rocket will manage for me. `BlogPost` and `BlogMeta`
are structs that will hold the really important info.

```rs
/// Return a list of all blog posts with metadata.
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
```

Oh my. I knew this would be big, but this is absurd.
The only reason this is so big is because `/api/blogs` has to get information amount multiple files *per subfolder*.
Here's the short version: if it's been more than 5 minutes since I've cached the blogs, recache them. This is for
live servers and really should only cache once rather than after an amount of time since the last cache. If it has
been less than 5 minutes, simply use the cache. When I cache the blogs, I look at everything in the `project/shuttle/static/blogs`
folder, which is just a copy of `project/public/blogs`. If anything in the folder is a regular file, continue onwards,
but if it's a folder, read the `meta.json` inside the folder and use [`serde_json`][serde_json] to parse it. Add the metadata and
path to `post.md` to a `HashMap<String, BlogPost>` with the metadata ID as the key, put that into the cache, and
return a copy of the map wrapped in `rocket::serde::json::Json` for serialization. **Simple, right?** I'll confess,
this took more from me than it should have.

```rs
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
```

Much better. Both of these endpoints reuse `/api/blogs` and I am so glad that they do. `/api/blog/<id>` returns
the `post.md` file itself, not its contents. `/api/blogdata/<id>` returns the metadata that `/api/blogs` gets.
All of the constants and `use` statements can be found on the [GitHub][this@github].

### API Routing

Now comes the easy part, right? All I have to do is route the server to `index.html` and the API endpoints! Wrong.
I actually need to set up two others: one to use `index.html` when there's a 404 error, and one to use `index.html`
when trying to connect to the blog page.

```rs
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
```

There's probably much more elegant solutions, but I can't be bothered. Now, I just need to mount
all of this.

```rs
#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![home, home_blog_override])
        .mount("/public", FileServer::new(relative!("static"), Options::Index))
        .mount("/assets", FileServer::new(relative!("static/assets"), Options::Index))
        .mount("/api", routes![api::hello, api::blogs, api::blog_content, api::blog_data])
        .register("/", catchers![not_found]);
        .manage(api::BlogPosts {
            last: Mutex::new(Instant::now() - Duration::DAY),
            list: Mutex::new(HashMap::new()),
        })

    Ok(rocket.into())
}

```

Look at those mounts! This is technically formatted incorrectly, but I like the single-line mounts too much.
The `Instant::now() - Duration::DAY` statement is just to make an `Instant` that is far enough in the past
that `/api/blogs` will always regenerate the cache on the first request. 

#### API Testing

There's only one thing left to do now.
I make a folder `project/public/blogs/000-hello-svelte` with placeholder `post.md` and `meta.json`.


```sh
$ npm run run &> /dev/null & # Run silently in the background
$ curl -I 0.0.0.0:8000/api/blogs
HTTP/1.1 200 OK
...
$ curl 0.0.0.0:8000/api/blogs
{"000-hello-svelte":{"filepath":"static/blogs/000-hello-svelte/post.md","meta":{"id":"000-hello-svelte","title":"Hello, Svelte!","author":"Yori","description":"A first blog post, detailing the site","image":null,"ordering":0}}}
```

Awesome. It's exactly what I expected. The others work too, which is an added bonus. Now to set up the frontend.

### Blog Frontend

This isn't going to be pretty. It's time to make a new svelte file. First, let's look at the `<script>` tag.

```ts
// Markdown renderer
import SvelteMarkdown from "svelte-markdown";
// List of links, for /blog
import LinkList from "../lib/LinkList.svelte";
// Dynamic renderer
import { ComponentRenderConfig, Render, createRender } from "svelte-render";
// API helper
import { import_blog, list_blogs } from "../blogs/blog_getter";
// Reactivity
import { writable, type Unsubscriber } from "svelte/store";
// Markdown additions
import MarkdownCodeSpan from "../lib/markdown/MarkdownCodeSpan.svelte";
import MarkdownCodeBlock from "../lib/markdown/MarkdownCodeBlock.svelte";

// Code highlighting
import 'highlight.js/styles/github-dark.min.css'
import hljs from 'highlight.js/lib/core';
// Languages
import javascript from 'highlight.js/lib/languages/javascript';
import json from 'highlight.js/lib/languages/json';
import css from 'highlight.js/lib/languages/css';
import toml from 'highlight.js/lib/languages/ini';
// The Svelte plugin doesn't work
import svelte from 'highlight.js/lib/languages/xml';
import rust from "highlight.js/lib/languages/rust";
import sh from "highlight.js/lib/languages/shell"
hljs.registerLanguage("js", javascript)
hljs.registerLanguage("ts", javascript)
hljs.registerLanguage("json", json)
hljs.registerLanguage("css", css)
hljs.registerLanguage("toml", toml)
hljs.registerLanguage("svelte", svelte)
hljs.registerLanguage("rs", rust)
hljs.registerLanguage("sh", sh)
```

This is ALL preparation for the rest of the script. There're [Svelte][svelte] and my own utilities.
I need to install [`svelte-render`][svelte-render] and [`highlight.js`][highlight.js]. 
`npm i -D -f svelte-render highlight.js`.

```ts
// Helpful enum. Typescript.
enum PageType {
    Post,
    List,
    NotFound
}

// blogId is automatically set thanks to <Route path="blog/:blogId" component="{Blog}"/>
export let blogId: string = null;
let pageType = blogId ? PageType.Post : PageType.List;
const divClass = () => pageType == PageType.NotFound ? "" : "blog";
// For feeding into Promise.then (async functions)
const dummy = () => null;

// A "Property Store," for Svelte reactivity
let markdownSource = writable({ source: "# Loading Blog Article...", renderers: { codespan: MarkdownCodeSpan, code: MarkdownCodeBlock } });
let unsubs: Unsubscriber[] = [];

// A configuration for a dynamically rendered component
let render: ComponentRenderConfig<any> = createRender(SvelteMarkdown, markdownSource);

// When markdownSource changes, highlight everything.
unsubs.push(markdownSource.subscribe(
    _ => setTimeout(() => hljs.highlightAll(), 0),
    _ => {},
))

// Run functions
if (pageType == PageType.List)
    default_page().then(dummy)
else if (pageType == PageType.Post)
    blog_page().then(dummy)
```

That's not too bad. I hope the functions are similar.

```ts
async function default_page() {
    try {

        let blogIter = Object.values(await list_blogs());
        let blogs = [];

        for (const blog of blogIter) blogs.push(blog.meta);

        blogs = blogs.sort((a,b)=> a.ordering - b.ordering).map(b => [b.title, `/blog/${b.id}`, `by ${b.author}. ${b.description}`])

        for (const unsub of unsubs) unsub();
        
        markdownSource.update(store => ({ source: "", renderers: store.renderers }));
        render = createRender(LinkList, { styles:"blog", header: "Blogs", links: blogs });
    } catch (err) {

        reportError(err);
        let source = "# Encountered an error.\n\nIt seems we've encountered an error:\n<pre>\n<code>\n${err.toString()}\n</code>\n</pre>\n";

        markdownSource.update(store => ({ source, renderers: store.renderers }));
    }
}
```

It's definitely not pretty, especially with that hand-written markdown text, but it's much better than what it could be.
Also, I'm using the blog list! That way, I can create a list of blogs. I create an array of the metadata and sort by the 
custom sorting property. CSS might actually have been easier if it weren't for my desire to use `LinkList`, my own creation
that makes link lists easy as well.

```svelte
<script lang="ts">
    import { Router, Link } from "svelte-navigator";
    export let header: string = "Links";
    export let links: ([string, string] | [string, string, string])[] = [];
    export let styles: string = "";

    console.log(links);
</script>

<div class={styles}>
    <h1>
        {header}
    </h1>
    <Router>
        <nav>
            <ul>
                {#each links as link, i}
                    <li>
                        <Link to={link[1]}>{link[0]}</Link>
                        {link[2] ?? ""}
                    </li>
                {/each}
            </ul>
        </nav>
    </Router>
</div>
```

I have a header, some link data, and some styles. I apply the styles to a div that holds a level-1
header with my desired text. Below that, I have a `Router` that will link throughout `/blog/*`. Then,
using Svelte's handy `#each` syntax, I create a bulleted list of `<Link>`s with the desired destination,
text, and accompanying text if desired. Now, let's look at `blog_page()`.

```ts
async function blog_page() {
    if (blogId == null) return default_page();

    let { text, meta } = await import_blog(blogId);
    let source: string;
    if (meta.id == blogId)
        source = `# ${meta.title}\n## Written by ${meta.author}\n\n${text}`;
    else
        source = "# 404.\n\nThat page doesn't exist.";

    markdownSource.update(store => ({ source, renderers: store.renderers }));
}
```

Oh, that's very simple. If I incorrectly call `blog_page`, I correct myself to `default_page`.
Otherwise, I import blog information, account for erroneous data (an error provides an empty `meta.id`)
and update the `markdownSource` object. Hah! I'm almost done! Wait, *almost?*

## Extra Pages

I can't forget a 404 page and an API landing page. First, the 404 page:

```svelte
<main>
    <h1>
        404.
    </h1>
    <p>
        Looks like we don't know what this page is.
    </p>
</main>
```

It couldn't get simpler. How about the API page?

```svelte
<script>
    import { Link } from "svelte-navigator";
    let output = "pending...";
    let duration;
    const start = Date.now();
    fetch("/api/blogs")
        .then(res => res.text())
        .then(text => {
            output = text
            duration = (Date.now() - start) + "ms"
        });
</script>

<main>
    <div class="blog">
        <h1>
            API
        </h1>
        <p>
            Hi! You shouldn't be here. It might be a good idea to <Link to="/">go home</Link>.
            The API is nothing right now. Come back later and I <i>might</i> have information on it.

            However, if you want to be sure that the routing for the API is still correct, here's a
            call to it: <br/>

            <b>{output}</b> {duration?`(${duration})`:""}
        </p>
    </div>
</main>
```

It's a little more complex, but it's still simple. I put up some placeholder variables and write a promise to
fill them with data, the result of an API call. I then have some svelte-coated HTML to display that information
and a link to go to the homepage. Neat!

## Final Testing

I almost forgot—I have a blog list to check! Let's see here...

![A picture of the blogs list](/public/blogs/000-hello-svelte/image-2.png)

**Beautiful! How about that blog page?**

![A picture of the blogs page](/public/blogs/000-hello-svelte/image-3.png)

**It's splendid! Why, I think I'm done!**

## Reflection

Has it been worth it? Yes. Did everything really go that smoothly? No. I'm not an expert on Svelte, so I had a
few mishaps on the way.

<!-- # Links Used
## In alphabetical order

[Advent of Code][aoc]

[Blog Github][this@github]

[Homepage][home]

[Jeff Mitchell's CCH Diary][jeffmitchelldiary]

[Rust Programming Language][rustlang]

[Shuttle Christmas Code Hunt][cch]

[Svelte][svelte]

[Vite][vite] -->



[aoc]: https://adventofcode.com/ "Advent of Code"
[cch]: https://shuttle.rs/cch "Shuttle's Christmas Code Hunt"
[highlight.js]: https://github.com/bryanmylee/svelte-render/ "Highlight.JS"
[home]: / "Homepage"
[jeffmitchelldiary]: https://sentinel1909-shuttle-cch.shuttleapp.rs/ "Jeff Mitchell's CCH Blog"
[npm]: https://npmjs.com/ "npm"
[rustlang]: https://rust-lang.org/ "Rust"
[serde]: https://serde.rs/ "Serde"
[serde_json]: https://docs.rs/serde_json/ "serde_json"
[shuttlers]: https://shuttle.rs/ "Shuttle"
[svelte]: https://svelte.dev/ "Svelte"
[svelte-navigator]: https://github.com/mefechoel/svelte-navigator/ "svelte-navigator"
[svelte-render]: https://github.com/bryanmylee/svelte-render/ "svelte-render"
[this@github]: https://github.com/proxxa/shuttle-blog/ "Github"
[tracing-rs]: https://docs.rs/tracing/ "tracing"
[vite]: https://vitejs.dev/ "Vite"
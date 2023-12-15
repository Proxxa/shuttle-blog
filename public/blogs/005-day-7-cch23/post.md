#### Table of Contents

- [Introduction](#introduction)
- [Task 1](#task-1)
  - [Prompt](#prompt)
  - [Solution](#solution)
- [Tasks 2/3](#tasks-23)
  - [Bonus Prompts](#bonus-prompts)
  - [Bonus Solutions](#bonus-solutions)
- [Conclusion](#conclusion)

## Introduction

Once again, hello fellow Shuttlers. It looks like this time, we need to once again look into documentation for this one.
Expect a link list similar to the size of the day 1 link list. If you haven't read day 1, you can find it [here][day1].
Now, on to the actual challenge.

## Task 1

### Prompt

> Santa's secret cookie recipe is hidden in an encoded message, and he's looking to you for help. He's sending a GET request to your server with a `Cookie` header.
>
> What you need to do is parse the Cookie header, decode the value in the recipe field, and return it.
>
> Make an endpoint `/7/decode` that extracts the `Cookie` HTTP header. The header in the request will look something like this:
>
> ``` Cookie: recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ== ```
>
> After decoding the recipe value to bytes, convert it to a string and return it as the response to the GET request.

Okay. It looks like we're going to be dealing with *actual browser cookies*, but instead they're arbitrarily set because that's "cool."
What do we need?

### Solution

We need a way to get the header AND decrypt the base64—or as the [Shuttle][shuttle] team puts it, the Based encoding, 64th edition.
I wish I was kidding.

![A screenshot of the Christmas Code Hunt challenge page saying "Based encoding, 64th edition."](/public/blogs/005-day-7-cch23/image.png)

Luckily, we don't even need to parse the decrypted cookie as JSON; we just need to decrpyt it and return that. What we need is the
[`base64`][base64-rs] crate. Let's add it: `cargo add base64`. Looking at the docs, they seem to have traits for making your own engine,
which is just an empty struct and an `impl Engine for ...` to make a config-returning function. That sounds like a lot of boilterplate,
and they agreed, so they apparently have a preconfigured engine: [`base64::prelude::BASE64_STANDARD`][base64-rs-preconfig]. This will use
the standard base64 encoding that this cookie uses. Let's decode using this, and turn the resulting `Vec<u8>` into a string with `String::from_utf8`.
To get the cookie in the first place, [Rocket][rocket-rs] has us covered. The `CookieJar` is made exclusively for this header, and it's
perfect.

```rs
// A utility function for mapping errors to 422 Unprocessable Entity
fn map_err422<O, E>(r: Result<O, E>) -> Result<O, Status> {
    r.map_err(|_| Status { code: 422 })
}

#[get("/decode")]
pub fn b64_decode(jar: &CookieJar) -> Result<String, Status> {
    jar.get_pending("recipe")
        // If no cookie, return 400 for malformed "input"
        .ok_or(Status { code: 400 })
        .map(|a| a.value().to_string())
        .map(|s| base64::prelude::BASE64_STANDARD.decode(&s))
        // If decoding failed, return 422 with the utility function
        .and_then(map_err422)
        // Convert to a string
        .map(String::from_utf8)
        // Again, map the error.
        .and_then(map_err422)
}
```

Unfortunately, with this monadic style of coding, I can't show the process as I think of it. Instead, I have to show off how
we solve the problem at the end. However, testing verifies that this works.

```sh
curl http://localhost:8000/7/decode \
  -H 'Cookie: recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ=='

{"flour":100,"chocolate chips":20}
```

## Tasks 2/3

### Bonus Prompts

It turns out that tasks 2 and 3 are basically the same. Here's task 2's prompt:

> Now that the recipe is decoded, Santa can get back to baking cookies! Santa is not sure, however, if he has enough of each ingredient to bake a cookie for every elf.
> 
> The same type of request as in Task 1 will be sent to a new endpoint, `/7/bake`, but this time Santa needs your help to calculate the amount of cookies he can bake with the ingredients he has in the pantry.
> 
> After decoding, parse the bytes as a JSON object (shape and keys can be seen in the example below) and use that to calculate how many cookies can be baked with the provided recipe and ingredients. Additionally, return the amount of ingredients that would remain in the pantry after the cookies have been baked.

Task 3 just says to make the same endpoint a bit tougher by ensuring that nothing bad happens if the recipe contains items
that aren't available in the pantry. Essentially, make a `5XX` error code impossible.

Here's some example input that [Shuttle][shuttle] provides:

```sh
curl http://localhost:8000/7/bake \
  -H 'Cookie: recipe=eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319'
```

That recipe cookie's value is equivalent to this JSON:

```json
{
  "recipe": {
    "flour": 95,
    "sugar": 50,
    "butter": 30,
    "baking powder": 10,
    "chocolate chips": 50
  },
  "pantry": {
    "flour": 385,
    "sugar": 507,
    "butter": 2122,
    "baking powder": 865,
    "chocolate chips": 457
  }
}
```

And here's the expected output:

```json
{
  "cookies": 4,
  "pantry": {
    "flour": 5,
    "sugar": 307,
    "butter": 2002,
    "baking powder": 825,
    "chocolate chips": 257
  }
}
```

### Bonus Solutions

It's "Bonus Solution**s**" because we're just going to do this correctly for both tasks the first time.

First, we want to decode just like we did before, so we'll copy the function and rename it. Then, we'll
parse it into the recipe and pantry data. Let's use [`serde_json`][serde_json], a companion crate of
[`serde`][serde]. First, let's write a structure for the output.

```rs
#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct RecipePantryData {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}
```

I can confidently use a `HashMap<String, usize>` because **1.** I'm from the future and I discovered why using &str was as bad as I thought
it would be before I even used it and **2.** I'm not certain that the keys of the pantry/recipe will always be the same, so I can gladly
use the flexible `HashMap`. 

```rs
jar.get_pending("recipe")
    .ok_or(Status { code: 400 })
    .map(|a| a.value().to_string())
    .map(|s| base64::prelude::BASE64_STANDARD.decode(&s))
    .and_then(map_err422)
    .map(String::from_utf8)
    .and_then(map_err422)
    .map(|s| serde_json::from_str::<RecipePantryData>(&s))
    .and_then(map_err422)
```

I'm glad I wrote that utility function. Now, we need to find how many cookies we can make. `HashMap::<_,_>::iter` will
iterate over all of the key-pair values so that I can get the amount needed for the recipe *and* the amount in the pantry
using the key. Since both are integers, dividing `pantry_amount / needed_by_cookie` will round down to the number of cookies
we can make given this situation. Also, since `HashMap::<_,V>::get` returns an `Option<V>`, we can use `Option::<V>::unwrap_or` 
to return `0` (or `&0`, to match the return type of `get`) if the pantry doesn't contain that ingredient. We can also use
`Iterator::fold` to make a `min_by_key` ourselves, since anything else will just return the name and recipe value of the
limiting ingredient, making us calculate the number of cookies required all over again.

```rs
.map(|d| {
    d.recipe
        .iter()
        .fold(usize::MAX, |a, (k, v)| {
            min(a, d.pantry.get(k).unwrap_or(&0) / v)
        }),
})
```

Showing off each map like this is confusing, but I'll show the whole function later just to clear things up.
The problem with this is that we've now mapped into a `Result<usize, Status>` when we still need to return the ingredients
that will be left in the pantry. We could either have a `let n = d.recipe...`, or we can use a closure to keep ourselves
"stateless." I like trying to become purely functional as most as we can, so I'll try the closure. What we need to do with the
pantry is map over the `HashMap` and create a new one where the proper number of ingredients have been removed. If the key in
the pantry does not exist in the recipe, simply multiply the number of cookies by `0` when deciding how much of the ingredient
to remove from the pantry. Since `Iterator::Map` returns another iterator, I'll simply use `Iterator::collect` directly into
a struct that accepts `HashMap<String, usize>` for a field. That struct, which will be our successful return type, looks like
this:

```rs
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CookiesPantryData {
    cookies: usize,
    pantry: HashMap<String, usize>,
}
```

The structs for these tasks are named for the data they contain, and it works well. The closure that builds this
struct looks like this:

```rs
(|n| {
    Json(CookiesPantryData {
        cookies: n,
        pantry: d
            .pantry
            .iter()
            .map(|(k, &v)| (k.to_owned(), v - (d.recipe.get(k).unwrap_or(&0) * n)))
            .collect(),
    })
})
```

So, finally, here's the entire endpoint in all of its monadic glory:

```rs
#[get("/bake")]
pub fn bake_cookies(jar: &CookieJar) -> Result<Json<CookiesPantryData>, Status> {
    jar.get_pending("recipe")
        .ok_or(Status { code: 400 })
        .map(|a| a.value().to_string())
        .map(|s| base64::prelude::BASE64_STANDARD.decode(&s))
        .and_then(map_err422)
        .map(String::from_utf8)
        .and_then(map_err422)
        .map(|s| serde_json::from_str::<RecipePantryData>(&s))
        .and_then(map_err422)
        .map(|d| {
            (|n| {
                Json(CookiesPantryData {
                    cookies: n,
                    pantry: d
                        .pantry
                        .iter()
                        .map(|(k, &v)| (k.to_owned(), v - (d.recipe.get(k).unwrap_or(&0) * n)))
                        .collect(),
                })
            })(
                d.recipe
                    .iter()
                    .fold(usize::MAX, |a, (k, v)| {
                        min(
                          a,
                          if v != &0 {
                              d.pantry.get(k).unwrap_or(&0) / v
                          } else {
                              usize::MAX
                          }
                    }),
            )
        })
}
```

> EDIT: This used to assume v was not 0, but tests released later revealed a test that had a key in the recipe
> with a 0 value. Devilish!

It's big and scary, but part of that is exactly what monads can do. They're big and scary, but they're elegant.
I'm handling errors without having to write any `if` statements, and the only "side effects" are the memory allocations
necessary for `String`, the [`base64`][base64-rs] crate, and [`serde_json`][serde_json] to work.\

Testing the example from Task 2 gives us exactly what we want, which is a good sign. Now, let's try the task 3 example.

```sh
curl 0.0.0.0:8000/7/bake \
  -H 'Cookie: recipe=eyJyZWNpcGUiOnsic2xpbWUiOjl9LCJwYW50cnkiOnsiY29iYmxlc3RvbmUiOjY0LCJzdGljayI6IDR9fQ=='

{
  "cookies": 0,
  "pantry": {
    "cobblestone": 64,
    "stick": 4
  }
}
```

That was simple enough. By the way, that cookie data comes out to have the recipe `{ "slime": 9 }`, so it's likely
that it's the minecraft reference that we want it to be. Sorry, elves, but we can't make slime block cookies today.
Stone tools and furnaces will have to do.

## Conclusion

I like this! The first wasn't too hard, but figuring out how to make the second endpoint perfectly monadic was a task
I wanted to handle and love to see that I accomplished. I can't wait to see the next day!

[base64-rs]: https://docs.rs/base64 "base64 Rust crate"
[base64-rs-preconfig]: https://docs.rs/base64/latest/base64/engine/general_purpose/constant.STANDARD.html "base64 Rust preconfigured engine"
[day1]: /blog "Blog list"
[rocket-rs]: https://rocket.rs "Rocket"
[serde]: https://serde.rs "Serde"
[serde_json]: https://docs.rs/serde_json "serde_json crate"
[shuttle]: https://shuttle.rs "Shuttle"

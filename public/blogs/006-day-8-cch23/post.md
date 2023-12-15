#### Table of Contents

- [Introduction](#introduction)
- [Task 1](#task-1)
  - [Prompt](#prompt)
  - [Solution](#solution)
- [Task 2](#task-2)
  - [Bonus Prompt](#bonus-prompt)
  - [Bonus Solution](#bonus-solution)
- [Improvement](#improvement)
- [Conclusion](#conclusion)

## Introduction

For far-future readers, I'm writing this almost a week late. Why? Because I was too
busy code hunting! Also, I moved home from uni for December break. This challenge will
need some documentation-reading, but not much for actual Rust code!

## Task 1

### Prompt

> Your quest is to add a GET endpoint `/8/weight/<pokedex_number>` that, given a pokédex number, responds with the corresponding Pokémon's weight in kilograms as a number in plain text.

### Solution

Wow, that's simple. Well, simple enough. We could make our own table of weights, or we could use the [PokéAPI][pokeapi].
I'll opt for the API, but if you want to make a massive `fn(u32)->f32`, then be my guest. The bit about plain text
really just means that the weight should be *all* we're returning, so we can just return the number. The PokéAPI says
that the `https://pokeapi.co/api/v2/pokemon/<id>` endpoint returns information about the Pokémon with the provided ID
*in hectograms* according to its [documentation][pokeapi-endpoint]. The prompt requires *kilograms*, so we'll convert by dividing by 10. Since this could return a decimal
weight, I want to return a floating point number. I'll be overkill and use `f64` because I like to use my memory to its
fullest extent even if that extent is wasted. Let's use 

I'm going to use `reqwest::get` from the [`reqwest`][reqwest] crate as suggested by [Shuttle][shuttlers]. 
This function will create a new HTTP client every time it's called, which is slow, but we can use [Rocket][rocketrs]
managed state to store a shared HTTP client later.

```rs
const ENDPOINT_BASE: &str = "https://pokeapi.co/api/v2/pokemon/";
#[get("/weight/<id>")]
pub async fn weight(id: usize) -> Result<Json<f64>, Status> {
    reqwest::get(format!("{ENDPOINT_BASE}{id}"))
        .await
        .map_err(|e| Status {
            code: e.status().map_or(500, |s| s.as_u16()),
        })?
        .json::<HashMap<String, Value>>()
        .await
        .map_err(|_| Status { code: 500 })?
        .get("weight")
        .and_then(|v| v.as_f64())
        .map(|f| Json(f / 10f64))
        .ok_or(Status { code: 500 })
}
```

After the first `await`, I want to return the error as best I can. If I can't get the error code, I'll return 
`500 Internal Server Error` because I can't be too sure it's the user's fault. Once I map the response to a glorified
JSON object, returning 500 for any parsing errors, I get the weight as an f64, divide it by 10, and wrap it in the `Json`
data guard provided by [Rocket][rocketrs]. 

Bravo! Now then, with the magic of having tested this myself, I can say that it works! On to the bonus task, then.

## Task 2

### Bonus Prompt

> Once the Pokémon's weight is retrieved, Santa needs you to calculate the momentum it would have at the time of impact with the floor if dropped from a 10-meter high chimney *(so that he knows if he needs to climb down or if he can just drop it)*.
>
> Keep in mind that the gravity of Earth that Santa has in his physics book was measured close to the North Pole. This could explain why his calculations are a bit off sometimes, but he still wants you to use it.
>
> The momentum, measured in Newton-seconds, signifies the force the Pokémon would exert upon meeting the floor beneath the 10-meter high chimney.
>
> The GET endpoint `/8/drop/<pokedex_number>` shall respond with a plain text floating point number.
>
> *Use gravitational acceleration `g = 9.825 m/s²`. Ignore air resistance.*

### Bonus Solution

> Ignore air resistance.

Glad to know we're theoretical physicists now. Joking aside, this is nothing more than a bit of multiplication.
Seriously, momentum can be calculated using `mass * velocity`. In our case, "mass" is just our weight. Technically,
`/8/weight` is really returning *mass* since *weight* is an entirely different thing, but for now, let's treat them
as the same. I can send apologies to my physics professor later.

```rs
// const GRAVITY: f64 = 9.825;
// v_f^2 - v_i^2 = 2 * dx * a
// v_i = 0
// sqrt(2 * 10 * 9.825) = sqrt(196.5) = ~14.0178457689
// Thanks, calculator!
const VELOCITY_AFTER_10M: f64 = 14.0178457689;
```

Here's a constant for the velocity of *anything* after 10 meters of free fall beginning from no velocity.
There's absolutely no way I'm making this website contain a LaTeX parser yet, so here's a picture of the equations:

![Equations detailing the approximation of the velocity of an object after falling for 10 meters from rest](/public/blogs/006-day-8-cch23/image.png)

Now all I have to do is multiply the "weight" (mass) by this constant. Really, it's that simple.

```rs
#[get("/drop/<id>")]
pub async fn drop(id: usize) -> Result<Json<f64>, Status> {
    weight(id).await.map(|Json(f)| Json(f * VELOCITY_AFTER_10M))
}
```

Sweet! The only fancy thing I've done is some parameter deconstruction. Again, I've tested these examples
before writing this, so I know for certain that everything here works. Now, let's get to making that managed
state HTTP client.

## Improvement

It's really simple. Here's what the source code for `reqwest::get` looks like:

```rs
pub async fn get<T: IntoUrl>(url: T) -> crate::Result<Response> {
    Client::builder().build()?.get(url).send().await
}
```

To make a client, all we need to do is run `Client::builder().build()` and unwrap it. I'm completely fine with
unwrapping in `main` because this only fails if something is very wrong. Now, let's manage this in main:

```rs
let rocket = rocket::build()
    .manage(Client::builder().build().unwrap())
    /* Routes... */
```

Thankfully, I don't need to put this into a mutex because `Client::get` only takes a `&self`, which is exactly
what rocket gives us. Now, let's modify our functions.

```rs
#[get("/weight/<id>")]
pub async fn weight(id: usize, client: &State<Client>) -> Result<Json<f64>, Status> {
    client
        .get(format!("{ENDPOINT_BASE}{id}"))
        .send()
        .await
        /* The same exact code after the first await */
}

#[get("/drop/<id>")]
pub async fn drop(id: usize, client: &State<Client>) -> Result<Json<f64>, Status> {
    weight(id, client)
        .await
        .map(|Json(f)| Json(f * VELOCITY_AFTER_10M))
}
```

Now, testing in real-time, everything works beautifully.

## Conclusion

Do I really need this? No. Instead, I'll "ramble" about the difference between mass and weight. Mass is a measure
of matter. Describing what mass really is can be tricky, so I'll simply lay out its difference from weight. **Weight
is the force of gravity on an object**. Here's an example in language: have you ever felt the "weight" of something?
Really, what you felt was the force of gravity on that object, or some equivalent "normal" (repelling) force.
However, have you ever felt the mass of something? No, you haven't. Nobody says that they've felt the mass of something
unless they're using the verb "to feel out" in place of the verb "to guess."

So, what is the difference? **Mass is a measure of how much matter something has. Weight is a measure of the force of**
**gravity on that mass.** If you want some concrete answers, go to Wikipedia or something.

[pokeapi]: https://pokeapi.co/ "PokéAPI"
[pokeapi-endpoint]: https://pokeapi.co/docs/v2#pokemon "PokéAPI Endpoint Documentation"
[reqwest]: https://docs.rs/reqwest/ "reqwest"
[rocketrs]: https://rocket.rs/ "Rocket"
[shuttlers]: https://shuttle.rs/ "Shuttle"

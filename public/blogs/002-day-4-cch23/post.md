#### Table of Contents

- [Introduction](#introduction)
  - [What's with the day numbering?](#whats-with-the-day-numbering)
- [Task 1](#task-1)
  - [Prompt](#prompt)
  - [Solution](#solution)
- [Bonus Task](#bonus-task)
  - [Bonus Prompt](#bonus-prompt)
    - [Example Input for Bonus Prompt](#example-input-for-bonus-prompt)
    - [Example Output for Bonus Prompt](#example-output-for-bonus-prompt)
  - [Bonus Prompt Solution.](#bonus-prompt-solution)
- [Thoughts](#thoughts)


## Introduction

Hello again, fellow Shuttlers! We may be unable to submit solutions, but it's still a great time to
work out these solutions. Today, the tasks are on *different* endpoints with *different* purposes.
Different endpoints having different purposes is a big surprise, I know.

### What's with the day numbering?

I'm numbering the days in accordance to the advent calendar for the [Shuttle][shuttle] [Christmas Code Hunt][cch].
This means numbering the days by the day of december that the challenges are revealed. Expect numbers to jump every
so often! That's every 5 days, to be exact.

![A picture of the Shuttle Christmas Code Hunt challenge calendar.](/public/blogs/002-day-4-cch23/image.png)

## Task 1

### Prompt

> The task is to create a POST endpoint `/4/strength` that calculates the combined strength of a group of reindeer, so that Santa knows if they can pull his sled through the skies.
>
> The input to the endpoint is a JSON array containing information about each reindeer. Each reindeer is represented as an object with two attributes: `"name"` (string) and `"strength"` (integer). Collect the strength of each reindeer and respond with the sum.

Neat. Basically, we'll receive some POST data at `/4/strength` in the form of an array of  `{ "name": string, "strength": integer }`. What we need to do is *ignore the name attribute* and sum the strengths. I guess it's time to write.

### Solution

This is actually very simple. Since we'll have an array of data, we can just iterate over it. Let's get to work. First, let's model the data.
We're going to need to use [`serde`][serde]. We can also install [`serde_json`][serde_json] while we're at it, but what we really need is
[`rocket`][rocket]'s form of `serde_json`, so let's update our installation to account for this. Let's run `cargo add rocket -F json` and
`cargo add serde`. I suppose `rocket` is all we need since rocket has its own custom built-in `serde` crate, but it's nicer to just
`use serde::{Serialize, Deserialize};`.


```rs
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct StrengthData {
    #[allow(unused)]
    name: String,
    strength: usize,
}
```

Nice. Since we're not using the `name` field, we could probably make it a `()` (unit) type and skip deserializing it, but I don't mind taking up
the space. Now, let's form an endpoint. The basics are first: function signature and annotations.

```rs
#[post("/strength", data = "<data>")]
pub fn strength(data: Json<Vec<StrengthData>>) -> Json<usize> {}
```

This is how POST looks in `rocket`. I don't know why data has to be specified like this, but it works and I don't mind typing it each time.
Since we're going to have JSON input, we want to use `rocket::serde::json::Json` to wrap our post data in. Don't worry, it's just a simple
wrapper that automatically dereferences to `&Vec<_>` when necessary. We'll return `usize` because who knows how large this'll get, but it'll
definitely never go negative, because negative strength is absurd. We also want to wrap that in `Json` to make sure we can send it as a response.
Let's write the inside.

```rs
#[post("/strength", data = "<data>")]
pub fn strength(data: Json<Vec<StrengthData>>) -> Json<usize> {
    Json(data.iter().fold(0, |a, b| a + b.strength))
}
```

We take the iterator over the vector and add all the strength values using an `Iterator::fold` and initial value of `0`. Now, if we run this...

```sh
curl -X POST 0.0.0.0:8000/4/strength \
  -H 'Content-Type: application/json' \
  -d '[
    { "name": "Dasher", "strength": 5 },
    { "name": "Dancer", "strength": 6 },
    { "name": "Prancer", "strength": 4 },
    { "name": "Vixen", "strength": 7 }
  ]'

22
```

`5 + 6 + 4 + 7` is in fact `22`. Good job, `0.0.0.0:8080/4/strength`!

## Bonus Task

### Bonus Prompt

> This time, there is some more data for each reindeer (see example). The endpoint is called `/4/contest`, because the reindeer are now going to compare the attributes of the reindeer and present a summary of the winners.
>
> There is at least one reindeer participating in the contest, and there is never a tie for first place.
>
> For some reason, one of the field names in the input seems to still be a bit corrupted from the incident. Guess we just have to deal with it anyways.
>
> The output should be a JSON object containing a summary of the winners (see example).

This one is weird. It's best to look at the example input/output. We'll be given a LOT of data on each reindeer, compare some of the values, and use the
others in showcases.

#### Example Input for Bonus Prompt

```sh
curl -X POST http://localhost:8000/4/contest \
  -H 'Content-Type: application/json' \
  -d '[
    {
      "name": "Dasher",
      "strength": 5,
      "speed": 50.4,
      "height": 80,
      "antler_width": 36,
      "snow_magic_power": 9001,
      "favorite_food": "hay",
      "cAnD13s_3ATeN-yesT3rdAy": 2
    },
    {
      "name": "Dancer",
      "strength": 6,
      "speed": 48.2,
      "height": 65,
      "antler_width": 37,
      "snow_magic_power": 4004,
      "favorite_food": "grass",
      "cAnD13s_3ATeN-yesT3rdAy": 5
    }
  ]'
```

#### Example Output for Bonus Prompt

```json
{
  "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
  "tallest": "Dasher is standing tall with his 36 cm wide antlers",
  "magician": "Dasher could blast you away with a snow magic power of 9001",
  "consumer": "Dancer ate lots of candies, but also some grass"
}
```

### Bonus Prompt Solution.

This barely makes sense, so I'll try to just implement something that runs the example properly. First, I need to find the reindeer
with the highest speed and make a string, "Speeding past the finish line with a strength of `<strength>` is `<name>`". I also
need to do something similar with height/antler width, magic/magic, and candies eaten/favorite food. Shouldn't be too hard. First,
some structures.

```rs
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ContestData {
    name: String,
    strength: usize,
    speed: f32,
    height: usize,
    antler_width: usize,
    snow_magic_power: usize,
    favorite_food: String,
    #[serde(alias = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: usize,
}

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct ContestOutput {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}
```

There's not much of a better way to do the output without just making one large `format!` macro and returning it, which could have unintended side effects.
You'll notice that we alias `ContestData.candies_eaten_yesterday`. This is to account for the corruption shown in the example, which I sincerely hope is
the only corruption there is. Now, let's brainstorm a way to find the data with the heighest of a given field.

```rs
let fastest = data
    .iter()
    .max_by(|a, b| a.speed.total_cmp(&b.speed))
    .unwrap();
```

Actually, that's not bad. Shuttle specifies that shere should always be a winner, and `total_cmp` almost guarantees a winner. Actually, this is unique
since speed is the only `f32`. The others are `usize`, so let's look at the `tallest`.

```rs
let tallest = data.iter().max_by_key(|r| r.height).unwrap();
```

Oh, it's one line. Again, Shuttle specifies that there's always a winner, so we can unwrap safely. That does give me the idea to simply return status
codes on error, so let's try that.

```rs
let tallest = data.iter().max_by_key(|r| r.height).ok_or(Status { code: 400 })?;
```

Great. `?` will propogate return the `Err` and unwrap the `Ok` for us. It's like syntactic sugar, but really, really good. Let's make these for everything
and write the full endpoint.

```rs
#[post("/contest", data = "<data>")]
pub fn contest(data: Json<Vec<ContestData>>) -> Result<Json<ContestOutput>, Status> {
    let fastest = data
        .iter()
        .max_by(|a, b| a.speed.total_cmp(&b.speed))
        .ok_or(Status { code: 400 })?;
    let fastest = format!(
        "Speeding past the finish line with a strength of {} is {}",
        fastest.strength, &fastest.name
    );

    let tallest = data.iter().max_by_key(|r| r.height).ok_or(Status { code: 400 })?;
    let tallest = format!(
        "{} is standing tall with his {} cm wide antlers",
        &tallest.name, tallest.antler_width
    );

    let magician = data.iter().max_by_key(|r| r.snow_magic_power).ok_or(Status { code: 400 })?;
    let magician = format!(
        "{} could blast you away with a snow magic power of {}",
        &magician.name, magician.snow_magic_power
    );

    let consumer = data
        .iter()
        .max_by_key(|r| r.candies_eaten_yesterday)
        .ok_or(Status { code: 400 })?;
    let consumer = format!(
        "{} ate lots of candies, but also some {}",
        &consumer.name, &consumer.favorite_food
    );

    Ok(Json(ContestOutput {
        fastest,
        tallest,
        magician,
        consumer,
    }))
}
```

Wow. How about some testing? 

Actually, you've already seen it. Yes, it works. Neat!

## Thoughts

This is actually a VERY easy challenge, though day 1 would be easy for someone very familiar with their framework, and this would be very hard
for someone who has never touched [Serde][serde]. Really, Serde is daunting. Overall, I enjoy the challenge! I just wish I could make the second challenge more monadic.

[rocket]: https://rocket.rs (Rocket)
[serde]: https://serde.rs (Serde)
[serde_json]: https://docs.rs/serde_json (serde_json)
[shuttle]: https://shuttle.rs (Shuttle)
[cch]: https://shuttle.rs/cch (Christmas Code Hunt)
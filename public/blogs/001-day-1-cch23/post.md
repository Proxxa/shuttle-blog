#### Table of Contents

- [Introduction](#introduction)
- [Task 1](#task-1)
  - [Prompt](#prompt)
  - [Solution](#solution)
- [Task 2](#task-2)
  - [Bonus Prompt](#bonus-prompt)
  - [Bonus Solution](#bonus-solution)


## Introduction

Greetings, fellow shuttlers! It's December 1! That means it's time for ~~[Advent of Code][aoc]~~ [Shuttle][shuttlers]'s [Christmas Code Hunt][cch]!
Today, it's all about `^`! No, not exponents; BitXOR! Let's take a look at the Tasks.

## Task 1

### Prompt

Alright, let's see here... "Implement a GET endpoint `/1/<num1>/<num2>` that takes 2 integers in the path, `num1` and `num2`, and returns the result of `(num1 XOR num2) POW 3`, where *XOR* is the exclusive OR operation, and *POW* is exponentiation." Okay, I lied, there is some exponentiation, but the XOR is the weird one.

### Solution

Alright, if you haven't done the task yet, go do it before reading further because I'm about to show the solution in [`Rocket`][rocketrs]. Is everyone who needs to leave gone yet?
Well, there's no way for me to know. Here's my solution:

```rs
// .mount("/1", routes![...])
#[get("/<a>/<b>")]
pub fn xor_pow(a: isize, b: isize) -> String {
    (a ^ b).pow(3).to_string()
}
```

I'm not kidding. It's that simple. Most people use `i32` in their solutions, but I opt for `isize` in order to make things architecture-agnostic. Well,
I doubt any 16-bit processor is running this code, but I'm not taking any risks. Rocket does the heavy lifting of parsing things into numbers;
I simply tell it to `a ^ b` and raise the result to the power of 3. Let's mount this and test it.

```sh
$ curl 0.0.0.0:8000/1/1/2
27
```

It works! `1 ^ 2` or `0b01 ^ 0b10` is `3` or `0b11`, and 3 cubed is 27.

## Task 2

### Bonus Prompt

Task 2 is a bonus task. This time, we're going sledding. "The formula is very similar: All packet IDs (integers) are *XOR*'ed with each other, and then the result is (again) raised to the power of 3. The catch is that there can be between 1 and 20 packets in a sled!" This is different. We need to change our endpoint so that it
accepts a variable number of... numbers. 


### Bonus Solution

Luckily, Shuttle links us to [Rocket documenation about routing][rocket-tip]. The [Multiple Segments][rocket-multiseg]
section is just what we need. [`FromSegments`][FromSegments] is a trait that will let us create a struct from URL segments.
Let's implement that for a `Vec<isize>`.

```rs
pub struct PathNums(pub Vec<isize>)
impl<'r> FromSegments<'r> for PathNums {
    type Error: ParseIntError;

    // Required method
    fn from_segments(segments: Segments<'r, Path>) -> Result<Self, Self::Error> {
        let vec: Vec<isize> = vec![];
        for segment in segments {
            vec.push(segment.parse()?);
        }

        Ok(vec)
    }
}
```

Nice! Just one problem. We'll need to once again iterate over this, *and* we're allocating on the stack for a one-time-use struct. Let's specialize a bit.

```rs
pub struct BitXorSegments(pub isize)
impl<'r> FromSegments<'r> for BitXorSegments {
    type Error: ParseIntError;

    // Required method
    fn from_segments(segments: Segments<'r, Path>) -> Result<Self, Self::Error> {
        let mut value = 0isize;
        for segment in segments {
            value ^= segment.parse::<isize>();
        }

        Ok(value)
    }
}
```

No more need to iterate again! I can start `value` at zero because `0 ^ x == x`. Now let's modify that endpoint.

```rs
// .mount("/1", routes![...])
#[get("/<parsed..>")]
pub fn xor_pow(parsed: BitXorSegments) -> String {
    parsed.pow(3).to_string()
}
```

This is delightfully easy! Now, let's test it. `5 ^ 4 ^ 7 == 6`. 6 cubed is 216. Let's check our endpoint...

```sh
$ curl 0.0.0.0:8000/1/5/4/7
216
```

It works beautifully! Now all we need to do is upload this. `cargo shuttle deploy --ad` will deploy
the project to Shuttle without any need to manage git.


[aoc]: https://adventofcode.com/ "Advent of Code"
[cch]: https://shuttle.rs/cch/ "Shuttle's Christmas Code Hunt"
[FromSegments]: https://api.rocket.rs/v0.5/rocket/request/trait.FromSegments.html/ "Rocket FromSegments Trait"
[rocketrs]: https://docs.rs/rocket/ "Rocket"
[rocket-tip]: https://rocket.rs/v0.5/guide/requests/#dynamic-paths/ "Shuttle's Bonus Tip"
[rocket-multiseg]: https://rocket.rs/v0.5/guide/requests/#multiple-segments/ "Multiple Segments Section"
[shuttlers]: https://shuttle.rs/ "Shuttle"
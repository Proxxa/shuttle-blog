#### Table of Contents

- [Introduction](#introduction)
- [Task 1](#task-1)
  - [Prompt](#prompt)
  - [Solution](#solution)
- [Task 2](#task-2)
  - [Bonus Prompt](#bonus-prompt)
  - [Bonus Solution](#bonus-solution)
- [Conclusion](#conclusion)

## Introduction

Hello, Shuttlers! Day 6 is available and it is all about elves! specifically, we need to find the word `elf` in strings. Let's go!

## Task 1

### Prompt

> Elves are notorious for their hide-and-seek skills, and now they've hidden themselves in strings of text!
>
> Create an endpoint `/6` that takes a POST request with a raw string as input and count how many times the substring `"elf"` appears.
>
> The output should be a JSON object containing the count of the string `"elf"`.

### Solution

I have a feeling that we'll be reusing this `/6` enpoint for a bonus prompt. Quite simply, we can count the number of occurences
by creating an iterator for each occurence. We don't even need a struct for this!

```rs
#[post("/", data = "<data>")]
pub fn endpoint(data: &str) -> Json<usize> {
    Json(data.matches("elf").count())
}
```

Wonderful! I'd like to test this, but I have a nagging suspicion that I'll want to take a look at the next prompt before testing anything.

## Task 2

### Bonus Prompt

> Add two fields to the response that counts:
>
> - The number of occurrences of the string `"elf on a shelf"` in a field with the same name.
> - The number of shelves that don't have an elf on it. That is, the number of strings `"shelf"` that are not preceded by the string `"elf on a "`. Put this count in the field `"shelf with no elf on it"`.

Egads! Who would have seen this coming? I did because I'm writing in the future. It looks like we might want to do something other than `str::matches` for this.

### Bonus Solution

First, let's create the new data structure that we need for a response.

```rs
[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DaySixResp {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    lonely_shelves: usize,
}
```

That'll do the trick. We need spaces in the key names, so we'll have to use annotation-like macros (proc-macros) to modify the behavior of serde.
Since we're doing that, we might as well name that dubiously-long final key something more fun like `lonely_shelves`. All good shelves deserve
an elf visit or two. I can think of a way for us to implement this completely monadically, but it's quite messy and I'm not too fond of it. Instead,
let's just split at every occurrence of `elf` and run an `Iterator::for_each`.

```rs
#[post("/", data = "<data>")]
pub fn endpoint(data: &str) -> Json<DaySixResp> {
    let mut elves = 0usize;
    let mut shelves = 0usize;
    let mut elves_on_shelves = 0usize;

    data.split("elf").for_each(|s| {
        elves += 1;
        if s.ends_with("sh") {
            shelves += 1;
        }
        if s == " on a sh" {
            elves_on_shelves += 1;
        }
    });

    Json(DaySixResp {
        elf: elves - 1, // Counts one extra.
        elf_on_a_shelf: elves_on_shelves,
        lonely_shelves: shelves - elves_on_shelves,
    })
}
```

What we've done here is we've calculated the number of elves with the tiniest bit of error that is always the same. Then,
we check if any of the `&str`s that we get in the closure ends with `"sh"`. If it does, then unsplit, the text would read
`"shelf"`. We could try to see if it doesn't say `"elf on a"` before this, but I have a better plan. We'll also check if a
`&str` reads `" on a sh"`. If so, then it says `"elf on a shelf"` without the splits. Since we know how many shelves there
are *and* how many elves are on shelves, we can calculate how many shelves must NOT have any elves on them. `shelves - elves_on_shelves`.
It does perfectly. And how about testing?

```sh
curl -X POST 0.0.0.0:8000/6 \
  -H 'Content-Type: text/plain' \
  -d 'there is an elf on a shelf on an elf.
      there is also another shelf in Belfast.'

{"elf":5,"elf on a shelf":1,"shelf with no elf on it":1}
```

Bravo! Do note, I'm writing this in the future when I know it's already successful, so these results are quite literally copied from
the example page, but I know for a fact that it works exactly the same.

## Conclusion

This is by far the easiest yet, but I have seen some odd solutions. I believe someone even used RegEx, and many people just used three
different `str::matches` statements. Whatever works best, I suppose, but I'm here to *challenge* myself, not for some quick hack.

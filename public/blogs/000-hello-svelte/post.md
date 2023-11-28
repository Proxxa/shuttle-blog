
#### Table of Contents

1. [Table of Contents](#table-of-contents)
2. [Introduction](#introduction)
3. [Inspiration](#the-inspiration)
4. [Links](#links-used)

### Introduction

Hello, Shuttle! This is Yori. Let's talk about how I made this website. It's not a super interesting story, but it's a story.
If you want to just see the source code, head over to [GitHub][this@github]. If you want to know a little more about it, look
no further than the [homepage][home]. Obviously, **this blog post will assume understanding of [Rust][rustlang] and [Shuttle][shuttlers].**
If you are not familiar with either, a lot of this will go over your head.

Alright? alright. Let's go.

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
to actually build, and there's no guarantee with Shuttle that our server will have `npm` installed. Even then, there's no guarantee
that we'll be able to write to disk, though if we can't, I personally wouldn't use that server vendor for long. Because there's no
guarantee that we can run `npm install` in our `build.rs` script, we instead need our Shuttle project to be a subfolder of our Svelte
project.

First, let's run `npm create vite@latest` and create a Svelte/JS project. TypeScript would work as well, but do not use SvelteKit.
To my current knowledge, SvelteKit builds a server itself, so unless we want to write an API to access an internal SvelteKit server,
we shouldn't use SvelteKit. I'll try it sometime, but not right now. If I try it, and it works, I might write a blog on it.

# This blog post isn't finished.

Check back some other time and I might have it done.

# Links Used
## In alphabetical order

[Advent of Code][aoc]

[Blog Github][this@github]

[Homepage][home]

[Jeff Mitchell's CCH Diary][jeffmitchelldiary]

[Rust Programming Language][rustlang]

[Shuttle Christmas Code Hunt][cch]

[Svelte][svelte]

[Vite][vite]



[aoc]: https://adventofcode.com/
[cch]: https://shuttle.rs/cch
[home]: /
[jeffmitchelldiary]: https://sentinel1909-shuttle-cch.shuttleapp.rs/
[rustlang]: https://rust-lang.org/
[shuttlers]: https://shuttle.rs/
[svelte]: https://svelte.dev/ 
[this@github]: https://github.com/proxxa/shuttle-blog/
[vite]: https://vitejs.dev/
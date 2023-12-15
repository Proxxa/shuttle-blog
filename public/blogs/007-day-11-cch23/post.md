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

We've got ourselves a grand time with this one today, my "fellow Shuttlers." Even if you don't
use [Shuttle][shuttlers], I hope you'll find this one interesting. Today, we'll be serving
images and reading pixel data.

## Task 1

### Prompt

> ![An image of various ball-shaped Christmas ornaments](/public/blogs/007-day-11-cch23/image.png)
>
> Download the image above and serve it as a static file so that a GET request to `/11/assets/decoration.png` responds with the image file and correct headers for MIME type (`Content-Type: image/png`) and response length (`Content-Length: ...`).

### Solution

The thing is, this one is already solved for us. [Rocket][rocketrs] already has a static file
server built in. First, let's Save the image in a folder named `assets` next to our `src` folder.
Next, let's mount a [Rocket file server][rocket-fileserver] at `/11/assets` that statically serves
all of the files in our `assets` folder.

```rs
let rocket = rocket::build()
    .mount("/11/assets", FileServer::new("assets", Options::None))
```

When calling `FileServer::new`, we need to give two things: a path relative to `Cargo.toml` and some options.
[`Options::None`][rocket-fs-options] will return any requested files and nothing else. With that, we can use the
brand new [CCH Validator][cch-validator] to validate locally! 

```
⋆｡°✩ ⋆⁺｡˚⋆˙‧₊✩₊‧˙⋆˚｡⁺⋆ ✩°｡⋆°✩ ⋆⁺｡˚⋆˙‧₊✩₊‧˙⋆˚｡⁺⋆ ✩°｡⋆
.・゜゜・・゜゜・．                .・゜゜・・゜゜・．
｡･ﾟﾟ･          SHUTTLE CCH23 VALIDATOR          ･ﾟﾟ･｡
.・゜゜・・゜゜・．                .・゜゜・・゜゜・．
⋆｡°✩ ⋆⁺｡˚⋆˙‧₊✩₊‧˙⋆˚｡⁺⋆ ✩°｡⋆°✩ ⋆⁺｡˚⋆˙‧₊✩₊‧˙⋆˚｡⁺⋆ ✩°｡⋆


Validating Challenge 11...

Task 1: completed 🎉
Core tasks completed ✅
```

## Task 2

### Bonus Prompt

> Add a POST endpoint `/11/red_pixels`, that takes in a PNG image in the `image` field of a multipart POST request, and returns the number of pixels in the image that are perceived as "magical red" when viewed with Santa's night vision goggles. The goggles considers a pixel "magical red" if the color values of the pixel fulfill the formula `red > blue + green`.

### Bonus Solution

This might take a while longer. First, I need to be able to get an image from a `POST` request.
Unfortunately, there's no data guard made by [Rocket][rocketrs] for this, so I'll want to get
a `Vec<u8>` of the data so that we can pass it on to an image reader/decoder. Yikes, this is
scary. First, let's make a struct that we can use to get our data.

```rs
pub struct MyFormField(Vec<u8>);
```

That's it. Unfortunately, I don't trust Rocket's implementation of `Form<Vec<u8>>` to only store the data
from the form field, so we'll be implementing the [`FromFormField`][rocket-fff] trait on our struct.

```rs
#[rocket::async_trait]
impl<'v> FromFormField<'v> for MyFormField {
    async fn from_data(field: DataField<'v, '_>) -> form::Result<'v, Self> {
        field
            .data
            .open(rocket::data::ToByteUnit::bytes(usize::MAX))
            .into_bytes()
            .await
            .map(|a| MyFormField(a.into_inner()))
            .map_err(|e| {
                let mut es = Errors::new();
                es.push(e.into());
                es
            })
    }
}
```

This is a lot of jargon just to say "get the raw data from this field." You can see that it's an async trait,
and it has `usize::MAX` because Rocket *would* cap the amount of data, but we want as much as we can get.
Once that's done, we need to pass this data into an image reader. Luckily, the [`image`][image-rs] crate has
just what we need. First, `image::io::reader::ImageReader` requires something with the `Read` trait. `Vec<u8>`
doesn't have this, but an `std::io::cursor::Cursor<Vec<u8>>` does! Then, we can guess the image format and
decode it. Let's write what we have so far.

```rs
#[post("/red_pixels", format = "multipart/form-data", data = "<data>")]
pub async fn bull_mode(data: Form<MyFormField>) -> Result<Json<usize>, Status> {
    ImageReader::new(Cursor::new(data.into_inner().0))
        .with_guessed_format()
        .map_err(|_| Status { code: 422 })
        .and_then(ImageReader::decode)
        .and_then(|res| res.map_err(|_| Status { code: 422 }))
        // ...
}
```

Nice. The prompt will most likely provide PNG files, but I want to be sure, so that's why we're guessing the image format.
Next, let's turn this into a bunch of RGB values. I'll use u16 just to be "fancy" and account for HDR or whatever it's for.
Next, let's iterate over the image in 3-number chunks for red, green, and blue, then we'll use `fold` to iterate over the
pixels and calculate how many pixels satisfy the predicate `red > blue + green`.

```rs
//      ...
        .map(|im| im.into_rgb16())
        .map(|buf| {
            buf.chunks_exact(3).fold(0usize, |a, p| {
                a + (p[0] as u32 > p[2] as u32 + p[1] as u32) as usize
            })
        })
        .map(|n| Json(n))
}
```

Now let's look at all of the code together.

```rs
pub struct MyFormField(Vec<u8>);

#[rocket::async_trait]
impl<'v> FromFormField<'v> for MyFormField {
    async fn from_data(field: DataField<'v, '_>) -> form::Result<'v, Self> {
        field
            .data
            .open(rocket::data::ToByteUnit::bytes(usize::MAX))
            .into_bytes()
            .await
            .map(|a| MyFormField(a.into_inner()))
            .map_err(|e| {
                let mut es = Errors::new();
                es.push(e.into());
                es
            })
    }
}

#[post("/red_pixels", format = "multipart/form-data", data = "<data>")]
pub async fn bull_mode(data: Form<MyFormField>) -> Result<Json<usize>, Status> {
    ImageReader::new(Cursor::new(data.into_inner().0))
        .with_guessed_format()
        .map_err(|_| Status { code: 422 })
        .map(ImageReader::decode)
        .and_then(|res| res.map_err(|_| Status { code: 422 }))
        .map(|im| im.into_rgb16())
        .map(|buf| {
            buf.chunks_exact(3).fold(0usize, |a, p| {
                a + (p[0] as u32 > p[2] as u32 + p[1] as u32) as usize
            })
        })
        .map(|n| Json(n))
}
```

This is a lot, and I'm not even too sure it'll work. However, it compiles! Let's check using the
validator again.

```
⋆｡°✩ ⋆⁺｡˚⋆˙‧₊✩₊‧˙⋆˚｡⁺⋆ ✩°｡⋆°✩ ⋆⁺｡˚⋆˙‧₊✩₊‧˙⋆˚｡⁺⋆ ✩°｡⋆
.・゜゜・・゜゜・．                .・゜゜・・゜゜・．
｡･ﾟﾟ･          SHUTTLE CCH23 VALIDATOR          ･ﾟﾟ･｡
.・゜゜・・゜゜・．                .・゜゜・・゜゜・．
⋆｡°✩ ⋆⁺｡˚⋆˙‧₊✩₊‧˙⋆˚｡⁺⋆ ✩°｡⋆°✩ ⋆⁺｡˚⋆˙‧₊✩₊‧˙⋆˚｡⁺⋆ ✩°｡⋆


Validating Challenge 11...

Task 1: completed 🎉
Core tasks completed ✅
Task 2: completed 🎉
Bonus points: 200 ✨
```

Awesome!

## Conclusion

So, how was this? The first task was simple enough. The Shuttle team says that the challenges should
increase in complexity, but really, the work that Rocket has done for me has only made these challenges
vary in difficulty. I will say that figuring out the second task today was difficult, but I'm sure
they saw the ease of the first task coming. The first task is quite complex, though.

[cch-validator]: https://crates.io/crates/cch23-validator "Christmas Code Hunt Validator"
[image-rs]: https://crates.io/crates/image "Image crate" 
[rocketrs]: https://rocket.rs/ "Rocket"
[rocket-fff]: https://api.rocket.rs/v0.5/rocket/form/trait.FromFormField.html "Rocket FromFormField"
[rocket-fileserver]: https://api.rocket.rs/v0.5/rocket/fs/struct.FileServer.html "Rocket File Server"
[rocket-fs-options]: https://api.rocket.rs/v0.5/rocket/fs/struct.Options.html "Rocket File Server Options"
[shuttlers]: https://shuttle.rs/ "Shuttle"
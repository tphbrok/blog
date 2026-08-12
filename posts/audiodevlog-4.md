---
category: Audio software
date: 2026-08-12
---

# audiodevlog 004: Creating an example and getting an audio backend to work

Having written a few generators, it was time to put them to practice, since unit tests can't prevent something from sounding bad.

I created an example `rising&lowbar;sine` which creates a `Sine` generator, fills an `f32` vector of 44100 samples with that generator, and increases the frequency by 100 Hz over those 44100 samples. The sampling frequency has thoughtfully been set to 44100 samples per second.

```rust
// examples/rising_sine.rs

use disperse::{
    backends,
    generators::{generator::Generator, sine::Sine},
};

/**
 * Plays a sine for 1 second, while linearly increasing its frequency from 440 to 540
 */
fn main() {
    let mut frequency = 440.0;
    let mut sine = Sine::new(frequency, 44100);

    let samples = vec![0.0; 44100]
        .iter()
        .map(|_| {
            sine.set_frequency(frequency);

            // Make it rise 100 Hz over the span of 44100 samples
            frequency = frequency + 100.0 / 44100.0;

            sine.get_next_value()
        })
        .collect();

    backends::cpal::play_samples(samples);
}
```

Running this example with `cargo run --example rising&lowbar;sine` satsifyingly plays exactly what I expected.

I did run into an issue with `cpal` though, where it simply didn't call the provided `data&lowbar;callback`, which should write the samples into an audio output buffer.

```rust
// src/backends/cpal.rs (partial)

let stream = device
    .build_output_stream(
        config.into(),
        move |data: &mut [f32], _| {
            for frame in data.chunks_mut(channels) {
                let value = next_value();

                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        |err| eprintln!("{err}"),
        None,
    );

// Nothing happens here!
stream.unwrap().play().unwrap();
```

The solution was moving an `unwrap()` call from the line that tries to `play()` the stream to the line that creates the stream. The reason is not unwrapping a `build&lowbar;output&lowbar;stream` call immediately drops the handle to the `cpal::Stream`, which results in `stream.play()` not doing anything nor erroring.

While I have my default search engine set to DuckDuckGo, it was Google that showed me [this 3 year old GitHub issue](https://github.com/RustAudio/cpal/issues/790) as its top search result, whereas DuckDuckGo failed to give me the results I needed to fix this (with the same prompt used for both searches). As an EU citizen, I'm all for using software that isn't part of _Big Tech&trade;_, but every now and then stuff like this happens, making me feel funny on the inside `¯\&lowbar;(ツ)&lowbar;/¯`.

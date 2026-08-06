---
category: Audio software
date: 2026-08-06
---

# audiodevlog 003: Writing generators for Disperse

I started writing this post mere seconds after pushing the latest commit of the day to the [Disperse](https://github.com/tphbrok/disperse) repository, so that all things I have learned are still fresh in memory.

The first piece of code I committed was a white noise generator function, which simply returned a random `f32` value between -1.0 and 1.0. I then made it into a stateful `struct`, so it could reuse the thread-local generator for improved performance in returning random values. Its function `get&lowbar;next&lowbar;value` returns the next random value from the already initialized random number generator.

```rust
// white_noise.rs

use rand::{RngExt, rng, rngs::ThreadRng};

use crate::generators::generator::Generator;

pub struct WhiteNoise {
    rng: ThreadRng,
}

impl WhiteNoise {
    pub fn new() -> Self {
        WhiteNoise { rng: rng() }
    }
}

impl Generator for WhiteNoise {
    fn get_next_value(&mut self) -> f32 {
        self.rng.random()
    }
}
```

Then, I built a `Sine` generator, which 'ticked' its phase to the next value (dependent on its frequency), so the next call to `get&lowbar;next&lowbar;value` neatly returns the next value in line. A more functional approach would be to make the phase part of the caller, but the entire point of theses stateful generator structs is to simplify writing and using generators. Moreover, a typical synthesizer has stateful oscillators, where an incoming MIDI note may or may not 'retrigger' the oscillator back to phase 0.

That's when I started noticing a pattern among the two generators (the `get&lowbar;next&lowbar;value` function) and extracted it into a `Generator` trait, which should be implemented by any generator.

```rust
// sine.rs (partial)

impl Generator for Sine {
    fn get_next_value(&mut self) -> f32 {
        let phase_increment = self.frequency / self.sample_rate;
        let new_phase = (self.phase + phase_increment) % 1.0;

        let next_value = (self.phase * 2.0 * PI).sin();
        self.phase = new_phase;

        next_value
    }
}
```

After that, I followed up with a `Triangle` and a `Sawtooth` generator. All generators are covered by unit tests, except for the white noise one, because I can't think of a way to reliably test random data.

### Mathematics become much simpler and more correct when minimizing usage of &#960; in calculations

The number &#960; is a pretty complex number, and performing `float` calculations mostly leads to inaccuracies. I doubt that anyone would hear the difference between an audio output level of 0.25 or 0.2499999, but if there's some calculation ordering or simplifications I can do to make it more accurate (without sacrificing performance), I went with it.

Also, I started out calculating the current phase of a generator over a range of [0, 2&#960;), but that meant I had to use calculations with &#960; in conditional expressions (e.g. for a triangle generator, where the waveform increases to 1 over [0, &#960;/2), decreases to -1 over [&#960;/2, 3&#960;/2) and returns to 0 towards 2&#960;). That meant the value at exactly &#960;/2 was sometimes exactly equal to 1, and sometimes it was not.

The resulting implementation of the `Generator` trait was much more satisfying than before:

```rust
// triangle.rs (partial)

impl Generator for Triangle {
    fn get_next_value(&mut self) -> f32 {
        let phase_increment = self.frequency / self.sample_rate;
        let new_phase = (self.phase + phase_increment) % 1.0;

        let next_value = if self.phase >= 0.0 && self.phase < 0.25 {
            self.phase
        } else if self.phase >= 0.75 && self.phase < 1.0 {
            self.phase - 1.0
        } else {
            0.5 - self.phase
        } * 4.0;
        self.phase = new_phase;

        next_value
    }
}
```

The unit tests for the triangle generator, that I copied from the sine generator, also became much better. Before, I had to use a certain _tolerance_ for output inaccuracies, because the values could deviate by 0.000001.

```rust
// Before

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_correct_values() {
        let mut triangle = Triangle::new(44100.0 / 8.0, 44100);

        assert!((0.0 - triangle.get_next_value()).abs() < 1e-6);
        assert!((0.5 - triangle.get_next_value()).abs() < 1e-6);
        assert!((1.0 - triangle.get_next_value()).abs() < 1e-6);
    }
}

// After

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_correct_values() {
        let mut triangle = Triangle::new(44100.0 / 8.0, 44100);

        assert_eq!(0.0, triangle.get_next_value());
        assert_eq!(0.5, triangle.get_next_value());
        assert_eq!(1.0, triangle.get_next_value());
    }
}

```

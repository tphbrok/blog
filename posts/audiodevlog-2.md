---
category: Audio software
date: 2026-07-30
---

# audiodevlog 002: Project refinements

Whenever I want to figure something out, I purposefully _do not_ actively try to figure it out. Just gathering some information and requirements, and then letting it marinate in my mind works best for me. This method has worked miracles for me for music production, where I sometimes managed to think through an entire musical piece before even creating the project file.

I followed the same process here, and I came to the following first project:

**Write a Rust DSP library from scratch, to learn about DSP and to only have to write it _once_ (because I'd want to use this library in other audio software I'll create in the future)**

It'll contain:

- generator functions, e.g. sine, cosine, noise, step, envelope
- stateful oscillator structs
- filter functions
- general DSP utility functions
- ... and much more

If I start with this project (which I probably will not finish before starting a new project), I can skip the non-audio part of audio software of integrating with other existing software (like creating VST plug-ins), while keeping it lightweight on the hardware side (because I can write a DSP library without knowing who will use it and how they'll use it to control their speakers).

And with this incentive, a new repository was born: [tphbrok/disperse](https://github.com/tphbrok/disperse). I like non-descriptive names that actually came from playing with letters of descriptive names (DSP with Rust &rarr; dsp + rs &rarr; dsprs &rarr; disperse).

---
category: Audio software
date: 2026-07-27
---

# audiodevlog 001: Introduction

Before and next to my current job writing enterprise software, I work as a professional DJ and producer of electronic dance music. Unsurprisingly, balancing the edge between these two career paths has crossed my mind: writing audio software. This varies from actual 'naked' software which directly instructs my speakers to move back and forth, or software which integrates seamlessly with a host (a _Digital Audio Workstation_, or a DAW) used to arrange and compose music.

Why not write about the journey while I'm at it!?

I've had some attempts before, I'll be honest about that. I've dabbled with Steinberg's VST SDK using C++, I've tried to write a bare-bones CLAP plug-in in C and Zig, and I've attempted to use a Rust audio plug-in framework called 'nih-plug' to create an audio plug-in. Creating a full DAW is another thing I tried, and failed at.

I'm not sure what the reason is for these failed attempts. Using LLM's to assist me in writing audio software isn't the solution, at least. I did manage to create a couple audio plug-ins that actually ran fine, but then I ended up trying to create another one that made my powerful MacBook Pro come to a screeching halt, suffocating CPU and memory resources, despite being a fairly simple plug-in. The classic AI-backed knowledge debt was the reason, I was simply unable to fix the performance issues because I didn't really know what I was doing.

I want this audiodevlog entry to mark the beginning of my final attempt in writing audio software. I don't have a specific goal in mind, other than simply doing this for entertainment and learning purposes.

To set some constraints for now:

- I'll be using Rust, because that's the systems programming language I'm most comfortable with right now. This might change throughout the process, but I'll probably have a good reason for it by then. If I would have went with C or C++, I'd have to refresh my knowledge on those, on top of dealing with memory safety
- Creating an audio-plugin involves dealing with SDKs or application APIs, which might not be the best fit to start the learning process with. Then again, I have some experience writing plug-ins from previous failed attempts, so the learning curve might not be too steep for me
- Develop in public, treat published source code and repositories as if people actually read code and use the software.

(once the next audiodevlog is available, a link will magically appear here)

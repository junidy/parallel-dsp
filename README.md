# Parallelized Real-Time Musical DSP
Proof of concept of a parallelized, multi-threaded strategy to real-time digital signal processing for predominantly musical applications. Implemented in Rust. Low-level, sample-based approach undergirded by _cpal_ and _fundsp_ crates. Command-line interface to modify signal chain. Audio sources include microphone input, samples cued by MIDI input, and long-form audio files.

## Milestone progress and next objectives
Progress at the time of the milestone includes the establishing of basic dynamically generated output audio, lock-free communication between DSP manager thread and output thread, and groundbreaking on CLI. Next objectives include enabling live microphone input and implentation of basic DSP algorithms within modular "audio nodes". Implementation of the parallelization strategy proper will occur upon the completion of supporting code, for which our target deadline is by the start of spring break.

## Link to supporting documentation
[Report draft, work in progress](https://docs.google.com/document/d/1yueU7AsnhksBSHPak2sx3SDHFSOYYCh8bnGxBNqH4V8/edit?usp=sharing).
This doc is where all the information and resources relevant to our project is centralized. It includes:
- Project abstract and motivation
- Overview of architecture and runtime model
- Explanation of project from first principles, from the physics of sound to the basics of DSP
- Aggregation of external sources to reference in report

## Usage
Ensure [Rust is installed](https://www.rust-lang.org/tools/install) on your machine. Navigate to the project directory and run the command `cargo run`.

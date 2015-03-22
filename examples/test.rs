#![feature(collections)]

extern crate bit_crusher;
extern crate dsp;

use dsp::{Dsp, Event, Settings, SoundStream};
use bit_crusher::BitCrusher;

const SAMPLE_HZ: u32 = 44_100;
const FRAMES: u16 = 512;
const CHANNELS: u16 = 2;
const BIT_DEPTH: u8 = 16;
const AMOUNT: f32 = 0.5;

fn main() {
    
    let mut sound_stream = SoundStream::<f32, f32>::new()
        .settings(Settings { sample_hz: SAMPLE_HZ, frames: FRAMES, channels: CHANNELS })
        .run()
        .unwrap();

    let mut bit_crusher = BitCrusher::new(BIT_DEPTH, AMOUNT, CHANNELS as usize);

    // We'll use this to process the input buffer and pass the result to the output.
    let mut buffer = Vec::new();

    // We'll use this to modulate the bit_crush amount.
    let mut time = 0.0;

    for event in sound_stream.by_ref() {
        match event {
            Event::In(input) => { ::std::mem::replace(&mut buffer, input); },
            Event::Out(output, settings) => {
                bit_crusher.audio_requested(&mut buffer[..], settings);
                output.clone_from_slice(&buffer[..]);
            },
            Event::Update(dt) => {
                use std::num::Float;
                time += dt;
                let new_amount = 0.5 + (time as f32 * 0.5).sin() * 0.5;
                bit_crusher.set_amount(new_amount);
                println!("BitCrusher amount: {}", new_amount);
            },
        }
    }

}


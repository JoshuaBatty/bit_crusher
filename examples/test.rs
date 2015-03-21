extern crate bit_crusher;
extern crate dsp;

use dsp::{SoundStream, Event, Dsp};
use bit_crusher::BitCrusher;

fn main() {
    
    let mut sound_stream = SoundStream::<f32, f32>::new().run().unwrap();

    let mut bit_crusher = BitCrusher::new(16, 0.5);

    let mut buffer = Vec::new();

    let mut time = 0.0;

    for event in sound_stream.by_ref() {
        match event {
            Event::In(samples) => { 
                buffer = samples;
            },
            
            Event::Out(output, settings) => {
                bit_crusher.audio_requested(&mut buffer[..], settings);
                output.clone_from_slice(&buffer[..]);
            },
            
            Event::Update(dt) => {
                use std::num::Float;
                time += dt;
                bit_crusher.amount = 0.5+(time as f32 * 0.5).sin()*0.5;
                
                println!("{}", bit_crusher.amount); 
            }
        }
    }
}


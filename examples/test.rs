
extern crate bit_crusher;
extern crate dsp;
extern crate num;

use dsp::{CallbackFlags, CallbackResult, Node, Settings, SoundStream, StreamParams};
use bit_crusher::BitCrusher;

const SAMPLE_HZ: f64 = 44_100.0;
const FRAMES: u16 = 512;
const CHANNELS: i32 = 2;
const BIT_DEPTH: u8 = 16;
const AMOUNT: f32 = 0.5;

fn main() {

    // Construct a new BitCrusher.
    let mut bit_crusher = BitCrusher::new(BIT_DEPTH, AMOUNT, CHANNELS as usize);

    // We'll use this to modulate the bit_crush amount.
    let mut time = 0.0;

    // Callback used to construct the duplex sound stream.
    let callback = Box::new(move |input: &[f32], _in_settings: Settings,
                                  output: &mut[f32], out_settings: Settings,
                                  dt: f64,
                                  _: CallbackFlags| {
        use num::Float;

        // Write the input samples to our output buffer.
        for (out_sample, in_sample) in output.iter_mut().zip(input.iter()) {
            *out_sample = *in_sample;
        }

        time += dt;
        let new_amount = 0.5 + (time as f32 * 0.5).sin() * 0.5;
        bit_crusher.set_amount(new_amount);

        // Process our output buffer.
        bit_crusher.audio_requested(output, out_settings);

        CallbackResult::Continue
    });

    // Build the params for our stream.
    let stream_params = StreamParams::new().channels(CHANNELS);

    // Construct the stream with default parameters.
    let stream = SoundStream::new()
        .sample_hz(SAMPLE_HZ)
        .frames_per_buffer(FRAMES)
        .duplex(stream_params, stream_params)
        .run_callback(callback)
        .unwrap();

    // Wait for our stream to finish.
    while let Ok(true) = stream.is_active() {
        ::std::thread::sleep_ms(16);
    }

}



use dsp::{self, Sample};
use dsp::Settings as DspSettings;

/// A type for bit crushing a given number of channels.
#[derive(Clone, Debug)]
pub struct BitCrusher {
    pub channels: Vec<Channel>,
}

/// A type for forcing audio samples to a fixed, low resolution.
#[derive(Clone, Debug)]
pub struct Channel {
    pub bit_depth: u8, 
    pub amount: f32,
    pub cnt: f32,
    pub y: f32,
}

impl BitCrusher {

    /// Constructor for a new BitCrusher.
    pub fn new(bit_depth: u8, amount: f32, num_channels: usize) -> BitCrusher {
        BitCrusher {
            channels: vec![Channel::new(bit_depth, amount); num_channels],
        }
    }

    /// A method for setting the amount of bit-crushedness on all channels.
    pub fn set_amount(&mut self, amount: f32) {
        for channel in self.channels.iter_mut() {
            channel.amount = amount;
        }
    }

    /// A method for setting the bit_depth for all channels.
    pub fn set_bit_depth(&mut self, bit_depth: u8) {
        for channel in self.channels.iter_mut() {
            channel.bit_depth = bit_depth;
        }
    }

}

impl Channel {

    /// Constructor for a Channel given a bit depth and amount.
    pub fn new(bit_depth: u8, amount: f32) -> Channel {
        Channel {
            bit_depth: bit_depth,
            amount: amount,
            y: 0.0,
            cnt: 0.0,
        }
    }

    /// A method for processing a single sample of audio.
    #[inline]
    fn decimate<S>(&mut self, sample: S) -> S where S: Sample {
        let sample: f32 = sample.to_wave();
        let m: i64 = 1 << (self.bit_depth - 1);
        self.cnt += self.amount;
        if self.cnt >= 1.0 {
            self.cnt -= 1.0;
            self.y =  ((sample * m as f32) as i64 / m) as f32;
        }
        Sample::from_wave(self.y)
    }

}

impl<S> dsp::Node<S> for Channel where S: Sample {
    fn audio_requested(&mut self, output: &mut [S], _settings: DspSettings) {
        for sample in output.iter_mut() {
            *sample = self.decimate(*sample);
        }
    }
}

impl<S> dsp::Node<S> for BitCrusher where S: Sample {
    fn audio_requested(&mut self, output: &mut [S], settings: DspSettings) {
        // For every frame in the output buffer.
        for frame in output.chunks_mut(settings.channels as usize) {
            // For each channel in the frame.
            for (channel, bit_crusher_channel) in frame.iter_mut().zip(self.channels.iter_mut()) {
                *channel = bit_crusher_channel.decimate(*channel);
            }
        }
    }
}


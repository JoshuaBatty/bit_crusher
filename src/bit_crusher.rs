
use dsp::{Dsp};
use dsp::Settings as DspSettings;

#[derive(Clone)]
pub struct BitCrusher{
    pub bit_depth: u8, 
    pub amount: f32,
    pub cnt: f32,
    pub y: f32,
}


impl BitCrusher{

    pub fn new(bit_depth: u8, amount: f32) -> BitCrusher{
        BitCrusher {
            bit_depth: bit_depth,
            amount: amount,
            y: 0.0,
            cnt: 0.0,
        }
    }

    fn decimate(&mut self, sample: f32) -> f32 {
        
        let m: i64 = 1<<(self.bit_depth-1);
        
        self.cnt += self.amount;
        if self.cnt >= 1.0 {

            self.cnt -= 1.0;
            self.y =  ((sample * m as f32) as i64 / m) as f32;
        }
        self.y
    }
}

impl Dsp<f32> for BitCrusher {

    fn audio_requested(&mut self, output: &mut [f32], _settings: DspSettings) {
        for sample in output.iter_mut() {
            *sample = self.decimate(*sample);
        }
    }
}


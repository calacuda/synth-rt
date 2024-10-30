use crate::SAMPLE_RATE;

pub struct Chorus {
    pub size: usize,
    pub buff: [f32; SAMPLE_RATE as usize],
    pub i: usize,
    pub step: usize,
    pub volume: f32,
    pub speed: f32,
}

impl Chorus {
    pub fn new() -> Self {
        Self {
            size: SAMPLE_RATE as usize,
            buff: [0.0; SAMPLE_RATE as usize],
            i: 0,
            step: 1,
            volume: 0.75,
            speed: 0.0,
        }
    }

    pub fn get_sample(&mut self, input_sample: f32) -> f32 {
        let chorus = ((self.buff[self.i] * self.volume) + input_sample).tanh();
        // self.buff[self.i ] = echo;
        self.buff[(self.i + self.step) % self.size] = chorus;
        // self.buff[self.i] = 0.0;
        // self.buff[(self.i as i64 - self.step as i64).abs() as usize % self.size] = echo;
        self.i = (self.i + 1) % self.size;
        // if echo == input_sample && input_sample != 0.0 {
        //     error!("[error] {}", self.i);
        // }
        chorus
    }

    /// sets speed, takes speed in seconds
    pub fn set_speed(&mut self, speed: f32) {
        // info!("speed: {}", speed);
        self.speed = speed;
        self.step = (SAMPLE_RATE as f32 * (speed * 0.05)) as usize;
        // info!("step:  {}", self.step);
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }
}

// pub struct Chorus {
//     buff: Buff,
// }
//
// impl Chorus {
//     pub fn new() -> Self {
//         const BUFF_SIZE: usize = SAMPLE_RATE as usize;
//
//         let mut buff = Buff {
//             size: BUFF_SIZE,
//             buff: [0.0; BUFF_SIZE],
//             i: 0,
//             step: 0,
//             volume: 0.75,
//         };
//         let audio_in = 0.0;
//
//         // buff.set_speed(0.0);
//         buff.set_speed(0.075);
//
//         Self { buff }
//     }
//
//     fn get_samples(&mut self, sample: f32) -> Vec<(u8, f32)> {
//         // info!("chorus");
//         vec![(0, self.buff.get_sample(sample))]
//     }
// }

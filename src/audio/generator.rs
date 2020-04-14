extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use pulse::stream::Direction;
use pulse::sample;
use std::f64::consts::PI;

pub const SAMPLE_RATE : u32 = 44100;
pub const BIT_DEPTH : u32 = 16;

pub struct AudioHandle {
    pub pa : psimple::Simple
}

pub fn init_audio() -> AudioHandle {
    println!("Initializing audio!");
    let spec = sample::Spec { format: sample::SAMPLE_S16NE,
        channels: 2,
        rate: SAMPLE_RATE,
    };
    assert!(spec.is_valid());

    let s = psimple::Simple::new(
        None,                   // Default server
        "AudioGame",            // Application name
        Direction::Playback,
        None,                   // Default device
        "Music",                // Stream description
        &spec,
        None,                   // Default channel map
        None                    // Default buffering attributes
    ).unwrap();
    
    AudioHandle {
        pa: s
    }
}

pub fn wavetable_to_data(left : &Vec<i16>, right : &Vec<i16>) -> Vec<u8> {
    // Left and right channel must contain the same number of samples
    assert_eq!(left.len(), right.len());

    const BYTES_PER_SAMPLE : usize = 2;
    const CHANNELS : usize = 2;

    let num_bytes : usize = left.len() * CHANNELS * BYTES_PER_SAMPLE;

    let mut data : Vec<u8> = vec![0; num_bytes];

    for (sample_idx, byte_idx) in (0..num_bytes).step_by(4).enumerate() {
        // Left channel, little endian
        data[byte_idx + 0] = (left[sample_idx] & 0xff) as u8;
        data[byte_idx + 1] = ((left[sample_idx] >> 8) & 0xff) as u8;

        // Right channel, little endian
        data[byte_idx + 2] = (right[sample_idx] & 0xff) as u8;
        data[byte_idx + 3] = ((right[sample_idx] >> 8) & 0xff) as u8;
    }

    data
}

/**
 * Takes a function and executes it for the given number of samples
 */
pub fn func_to_wavetable(num_samples : usize, f : impl Fn(usize) -> i16) -> Vec<i16> {
    let mut wavetable : Vec<i16> = vec![0; num_samples];
    for x in 0..num_samples {
       wavetable[x] = f(x);
    }

    wavetable
}

pub fn sine_wavetable(freq : u32, phase : f64, amplitude : f64) -> Vec<i16> {
    let samples_per_repetition : usize = (SAMPLE_RATE / freq) as usize;

    return func_to_wavetable(samples_per_repetition, |t|  {
        ((f64::from(t as i32) / f64::from(samples_per_repetition as i32) * 2.0 * PI + phase).sin()
            * 2.0f64.powi(BIT_DEPTH as i32 - 1) * amplitude) as i16
    });
}

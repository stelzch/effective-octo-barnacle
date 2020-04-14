extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use pulse::stream::Direction;
use pulse::sample;

pub const SAMPLE_RATE : u32 = 44100;

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
pub fn func_to_wavetable(num_samples : usize, f : fn(usize) -> i16) -> Vec<i16> {
    let mut wavetable : Vec<i16> = vec![0; num_samples];
    for x in 0..num_samples {
       wavetable[x] = f(x);
    }

    wavetable
}

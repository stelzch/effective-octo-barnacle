extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use psimple::Simple;
use pulse::stream::Direction;
use pulse::sample;
use std::f64::consts::PI;

fn main() {
    const SAMPLERATE : u32 = 44100;

    let spec = sample::Spec { format: sample::SAMPLE_S16NE,
        channels: 2,
        rate: SAMPLERATE,
    };
    assert!(spec.is_valid());

    let s = Simple::new(
        None,                   // Default server
        "AudioGame",            // Application name
        Direction::Playback,
        None,                   // Default device
        "Music",                // Stream description
        &spec,
        None,                   // Default channel map
        None                    // Default buffering attributes
    ).unwrap();

    // Construct a wavetable
    const FREQ : u32 = 440;
    const SAMPLES_PER_REPETITION : u32 = SAMPLERATE / FREQ;

    // 2 channels with 2 bytes per sample
    const WAVETABLE_LENGTH : u32 = (SAMPLES_PER_REPETITION * 2 * 2) as u32;

    let mut wavetable : Vec<u8> = vec![0; WAVETABLE_LENGTH as usize];
    let volume : f64 = 0.6;
    const MAX_SAMPLE_VALUE : i16 = 1 << 15;

    for x in (0..WAVETABLE_LENGTH).step_by(4) {
        let idx = x as usize;
        let val : f64 = (f64::from(x) / f64::from(SAMPLES_PER_REPETITION) * 2.0* PI).sin() * volume * f64::from(MAX_SAMPLE_VALUE);
        let val : i16 = val as i16;
        let val : u16 = val as u16;
        //println!("{}", val);

        let upper_byte : u8 = ((val >> 8) ) as u8;
        let lower_byte : u8 = (val & 0xff) as u8;

        println!("{}", u64::from(upper_byte));

        // Left channel
        wavetable[idx] = lower_byte;
        wavetable[idx + 1] = upper_byte;

        // Right channel
        wavetable[idx + 2] = lower_byte;
        wavetable[idx + 3] = upper_byte;
    }

    loop {
        s.write(&wavetable).expect("Failed to write");
    }
}

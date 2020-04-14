extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use std::f64::consts::PI;

mod audio;
mod graphics;

use audio::generator::AudioHandle;

fn main() {
    let audio : AudioHandle = audio::generator::init_audio();

    // Construct a sine wavetable
    const FREQ : u32 = 440;
    const SAMPLES_PER_REPETITION : usize = (audio::generator::SAMPLE_RATE / FREQ) as usize;
    const vol : f64 = 0.6;

    let wavetable = audio::generator::func_to_wavetable(SAMPLES_PER_REPETITION,
        |t|  {
            ((f64::from(t as i32) / f64::from(SAMPLES_PER_REPETITION as i32) * 2.0 * PI).sin()
                * 2.0f64.powi(15) * vol) as i16
        });

    let audio_data = audio::generator::wavetable_to_data(&wavetable, &wavetable);

    loop {
        audio.pa.write(&audio_data).expect("Failed to write");
    }
}

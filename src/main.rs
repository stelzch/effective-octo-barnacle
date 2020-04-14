extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use std::f64::consts::PI;

mod audio;
mod graphics;

use audio::generator::AudioHandle;

fn main() {
    let audio : AudioHandle = audio::generator::init_audio();

    let left_wavetable = audio::generator::sine_wavetable(444, 0.0, 0.2);
    let right_wavetable = audio::generator::sine_wavetable(444, 0.0, 0.7);

    let audio_data = audio::generator::wavetable_to_data(&left_wavetable, &right_wavetable);

    loop {
        audio.pa.write(&audio_data).expect("Failed to write");
    }
}

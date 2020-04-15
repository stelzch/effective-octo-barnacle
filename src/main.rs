extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use std::f64::consts::PI;

mod audio;
mod graphics;

use audio::generator::AudioHandle;

use graphics::Point2D;
use graphics::Polygon;

fn main() {
    let audio : AudioHandle = audio::generator::init_audio();

    let left_wavetable = audio::generator::sine_wavetable(444, 0.0, 0.2);
    let right_wavetable = audio::generator::sine_wavetable(444, 0.25 * PI, 0.7);

    let audio_data = audio::generator::wavetable_to_data(&left_wavetable, &right_wavetable);

    let mut triangle = Polygon {
        points : vec![
            Point2D {
                x: -0.5,
                y: -0.5
            },
            Point2D {
                x: 0.0,
                y: 0.5
            },
            Point2D {
                x: 0.5,
                y: -0.5
            },
            Point2D {
                x: 0.0,
                y: -0.25
            }
        ],
        x: 0.0,
        y: 0.0

    };

    let mut bounding_rect = graphics::rectangle(-1.0, -1.0, 2.0, 2.0);
    let mut left_paddle = graphics::rectangle(-0.80, -0.1, 0.2, 0.4);
    let mut t = 0.0;

    loop {
        audio.pa.write(&audio::generator::polygon(200, &left_paddle)).expect("Failed to write");
        audio.pa.write(&audio::generator::polygon(200, &bounding_rect));

        
        t += 0.1;
    }
}

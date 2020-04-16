extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;
extern crate rand;

use std::f64::consts::PI;

mod audio;
mod graphics;
mod engine;

use audio::generator::AudioHandle;

use graphics::{ Point2D, Polygon, Vector2D, Rectangle };
use engine::{ Scene, Entity };
use rand::prelude::*;


fn main() {
    const PADDLE_WIDTH : f64 = 0.1;
    const PADDLE_HEIGHT : f64 = 0.45;
    const BALL_RADIUS : f64 = 0.07;
    let audio : AudioHandle = audio::generator::init_audio();

    let paddle = graphics::Rectangle {
        pos: Point2D {
            x: 0.0,
            y: -0.5 * PADDLE_HEIGHT
        },
        width: PADDLE_WIDTH,
        height: PADDLE_HEIGHT
    };

    let bounding_box = graphics::Rectangle {
        pos: Point2D {
            x: -1.0,
            y: -1.0,
        },
        width: 2.0,
        height: 2.0
    };

    let left_paddle = Entity {
        geometry: Polygon::new(&paddle),
        bounding_box: paddle.clone(),
        pos: Point2D {
            x: -0.75,
            y: 0.0
        },
        direction: None
    };

    let right_paddle = Entity {
        geometry: Polygon::new(&paddle),
        bounding_box: paddle.clone(),
        pos: Point2D {
            x: 0.75,
            y: 0.0
        },
        direction: Some(Vector2D {
            x: 0.0,
            y: 0.0
        })
    };

    let field_border = Entity {
        geometry: Polygon::new(&bounding_box),
        bounding_box: bounding_box.clone(),
        pos: Point2D {
            x: 0.0,
            y: 0.0
        },
        direction: Some(Vector2D {
            x: 0.0,
            y: 0.0
        })
    };

    let ball = Entity {
        geometry: Polygon::new_circle(40, 0.05, Point2D { x: 0.0, y: 0.0 }),
        bounding_box: Rectangle {
            pos: Point2D {x: -0.1, y: -0.1 },
            width: 0.2,
            height: 0.2
        },
        pos: Point2D { x: 0.0, y: 0.0 },
        direction: Some(Vector2D {
            x: random(),
            y: random()
        })
    };

    let scene = Scene {
        num_samples: 800,    // 60 fps!!
        entities: vec![
            left_paddle,
            right_paddle,
            field_border,
            ball
        ]
    };

    loop {
        scene.render(&audio);
    }
}

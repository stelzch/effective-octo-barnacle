use crate::graphics::Point2D;
use crate::graphics::Vector2D;
use crate::graphics::Polygon;
use crate::graphics::Rectangle;
use crate::audio::generator::AudioHandle;
use crate::audio::generator::entity_to_wavetable;
use std::cmp::min;


pub struct Entity {
   pub geometry: Polygon,
   pub bounding_box: Rectangle,
   pub pos : Point2D,
   pub direction : Option<Vector2D>
}

impl Entity {
    pub fn render(&self, audio : &AudioHandle, samples : usize) {
        audio.pa.write(&entity_to_wavetable(samples, &self));
    }

    pub fn perimeter(&self) -> f64 {
        self.geometry.perimeter()
    }
}


pub struct Scene {
    pub entities : Vec<Entity>,
    pub num_samples : usize,
}

impl Scene {
    pub fn render(&self, audio : &AudioHandle) {
        let total_perimeter : f64 = self.entities.iter().map(|e| e.perimeter()).sum();

        for entity in self.entities.iter() {
            let entity_samples : usize = (entity.perimeter() / total_perimeter * f64::from(self.num_samples as i32)) as usize;
            entity.render(audio, 200);
        }
    }
}

pub struct Point2D {
    pub x: f64,
    pub y: f64
}

type Matrix2D = [[f64; 2]; 2];

fn rotation_matrix(angle : f64) -> Matrix2D {
    let mut m : Matrix2D = [[0.0; 2]; 2];
    m[0][0] = f64::cos(angle); 
    m[0][1] = -1.0 * f64::sin(angle);
    m[1][0] = f64::sin(angle);
    m[1][1] = f64::cos(angle);

    m
}

impl Point2D {
    pub fn distance_to(&self, other : &Point2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    // matrix is in row-major form
    pub fn transform(&mut self, matrix: &Matrix2D) {
        let new_x = self.x * matrix[0][0] + self.y * matrix[0][1];
        let new_y = self.x * matrix[1][0] + self.y * matrix[1][1];

        self.x = new_x;
        self.y = new_y;
    }

    pub fn rotate(&mut self, angle : f64) {
        self.transform(&rotation_matrix(angle));
    }

    pub fn translate(&mut self, dx : f64, dy : f64) {
        self.x += dx;
        self.y += dy;
    }

    pub fn scale(&mut self, sx : f64, sy : f64) {
        self.x *= sx;
        self.y *= sy;

    }
}

pub struct Polygon {
    pub points : Vec<Point2D>,
    pub x : f64,
    pub y : f64
}

impl Polygon {
    pub fn iterate_lines_mut(&self, mut f : impl FnMut(&Point2D, &Point2D)) {
        if self.points.len() < 2 {
            panic!("Can't iterate over lines with a polygon that has less than 2 points");
        }

        let mut last_idx : usize = 0;
        let n = self.points.len();
        loop { 
            f(&self.points[last_idx % n], &self.points[(last_idx + 1) % n]);

            last_idx = (last_idx + 1) % n;
            if last_idx == 0 {
                break;
            }
        }
    }

    pub fn perimeter(&self) -> f64 {
        let mut perimeter = 0.0;
        self.iterate_lines_mut(|a, b| {
            perimeter += a.distance_to(b);
        });

        perimeter
    }

    pub fn rotate(&mut self, angle : f64) {
        for point in self.points.iter_mut() {
            point.rotate(angle);
        }
    }

    pub fn scale(&mut self, sx : f64, sy : f64) {
        for point in self.points.iter_mut() {
            point.scale(sx, sy);
        }
    }
}

pub fn interpolate_points(a: &Point2D, b: &Point2D, t: f64) -> Point2D {
    let diff_x : f64 = b.x - a.x;
    let diff_y : f64 = b.y - a.y;

    Point2D {
        x: a.x + (t * diff_x),
        y: a.y + (t * diff_y)
    }
}

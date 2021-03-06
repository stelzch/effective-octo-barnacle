use std::f64::consts::PI;

#[derive(Clone)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

pub type Matrix2D = [[f64; 2]; 2];
pub type Vector2D = Point2D;

fn rotation_matrix(angle: f64) -> Matrix2D {
    let mut m: Matrix2D = [[0.0; 2]; 2];
    m[0][0] = f64::cos(angle);
    m[0][1] = -1.0 * f64::sin(angle);
    m[1][0] = f64::sin(angle);
    m[1][1] = f64::cos(angle);

    m
}

impl Point2D {
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    // matrix is in row-major form
    pub fn transform(&mut self, matrix: &Matrix2D) {
        let new_x = self.x * matrix[0][0] + self.y * matrix[0][1];
        let new_y = self.x * matrix[1][0] + self.y * matrix[1][1];

        self.x = new_x;
        self.y = new_y;
    }

    pub fn rotate(&mut self, angle: f64) {
        self.transform(&rotation_matrix(angle));
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    pub fn scale(&mut self, sx: f64, sy: f64) {
        self.x *= sx;
        self.y *= sy;
    }
}

#[derive(Clone)]
pub struct Polygon {
    pub points: Vec<Point2D>,
    pub x: f64,
    pub y: f64,
}

impl Polygon {
    pub fn new(r : &Rectangle) -> Polygon {
        Polygon {
            points: vec![
                Point2D {
                    x: r.pos.x,
                    y: r.pos.y
                },
                Point2D {
                    x: r.pos.x,
                    y: r.pos.y + r.height
                },
                Point2D {
                    x: r.pos.x + r.width,
                    y: r.pos.y + r.height
                },
                Point2D {
                    x: r.pos.x + r.width,
                    y: r.pos.y
                }
            ],
            x: r.pos.x,
            y: r.pos.y
        }
    }

    pub fn new_circle(num_points : i32, r : f64, pos : Point2D,) -> Polygon {
        let mut points : Vec<Point2D> = vec![];

        for p in 0..num_points {
            let a : f64 = f64::from(p) / f64::from(num_points) * 2.0 * PI;

            points.push(Point2D {
                x: f64::sin(a) * r,
                y: f64::cos(a) * r
            });
        }

        Polygon {
            points,
            x: pos.x,
            y: pos.y
        }
    }

    pub fn iterate_lines_mut(&self, mut f: impl FnMut(&Point2D, &Point2D)) {
        if self.points.len() < 2 {
            panic!("Can't iterate over lines with a polygon that has less than 2 points");
        }

        let mut last_idx: usize = 0;
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

    pub fn rotate(&mut self, angle: f64) {
        for point in self.points.iter_mut() {
            point.rotate(angle);
        }
    }

    pub fn scale(&mut self, sx: f64, sy: f64) {
        for point in self.points.iter_mut() {
            point.scale(sx, sy);
        }
    }

}

pub fn interpolate_points(a: &Point2D, b: &Point2D, t: f64) -> Point2D {
    let diff_x: f64 = b.x - a.x;
    let diff_y: f64 = b.y - a.y;

    Point2D {
        x: a.x + (t * diff_x),
        y: a.y + (t * diff_y),
    }
}

#[derive(Clone)]
pub struct Rectangle {
    pub pos: Point2D,
    pub width: f64,
    pub height: f64
}

impl Rectangle {
    pub fn new(x : f64, y : f64, width : f64, height : f64) -> Rectangle {
        Rectangle {
            pos: Point2D {
                x,
                y
            },
            width,
            height
        }
    }
}

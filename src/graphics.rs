pub struct Point2D {
    x: f64,
    y: f64
}

pub fn interpolate_points(a: Point2D, b: Point2D, t: f64) -> Point2D {
    let diff_x : f64 = a.x - b.x;
    let diff_y : f64 = a.y - b.y;

    Point2D {
        x: a.x + (t * diff_x),
        y: a.y + (t * diff_y)
    }
}

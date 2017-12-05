use std::fmt;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point {x, y}
    }

    pub fn add(&self, x: usize, y: usize) -> Point {
        Point { x: &self.x + x, y: &self.y + y }
    }

    pub fn new_vec(data: Vec<(usize, usize)>) -> Vec<Point> {
        let mut vec = Vec::with_capacity(data.len());

        for (x, y) in data {
            vec.push(Point::new(x, y));
        }

        vec
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{},{}}}", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x , y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
    
    pub fn sort(self, p: Self) -> (Point, Point) {
        if self.y < p.y {
            (self, p)
        } else if self.y > p.y {
            (p, self)
        } else if self.x < p.x {
            (self, p)
        } else {
            (p, self)
        }
    }

    pub fn lerp_y(&self, p: &Self, x: usize) -> Option<usize> {
        if self.x == p.x {
            None
        } else {
            let dx = self.x as i32 - p.x as i32;
            let dy = self.y as i32 - p.y as i32 ;
            let m = dy as f64 / dx as f64;
            let y = ((m * x as f64) as i32 - p.x as i32 + p.y as i32) as usize;
            Some(y)
        }
    }

    pub fn lerp_x(&self, p: &Self, y: usize) -> Option<usize> {
        if self.y == p.y {
            None
        } else {
            let dx = self.x as i32 - p.x as i32;
            let dy = self.y as i32 - p.y as i32 ;
            let m = dy as f64 / dx as f64;
            let x = (((y as i32 - p.y as i32) as f64 / m) as i32 + p.x as i32) as usize;
            Some(x)
        }
    }
}

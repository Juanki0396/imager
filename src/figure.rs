use crate::point::Point;
use crate::dims::Dims;

#[derive(Debug)]
pub enum Figures {
    Triangle(Point, Point, Point),
    Rectangle(Point, Dims),
    Circle(Point, usize),
    Line(Point, Point),
}

impl Figures {
    pub fn new_rectangle(x: usize, y:usize, width: usize, height: usize) -> Self {
        Self::Rectangle(
            Point::new(x,y), 
            Dims::new(width, height)
            ).sort_points()
    }

    pub fn new_circle(x: usize, y:usize, r: usize) -> Self {
        Self::Circle(
            Point::new(x,y), 
            r
            ).sort_points()
    }

    pub fn new_line(x1: usize, y1:usize, x2: usize, y2: usize) -> Self {
        Self::Line(
            Point::new(x1,y1), 
            Point::new(x2,y2), 
            ).sort_points()
    }

    pub fn new_triangle(x1: usize, y1:usize, x2: usize, y2: usize, x3: usize, y3: usize) -> Self {
        Self::Triangle(
            Point::new(x1,y1), 
            Point::new(x2,y2), 
            Point::new(x3,y3), 
            ).sort_points()
    }

    fn sort_points(self) -> Self {
        match self {
            Self::Line(p1,p2) => {
                let (p1, p2) = p1.sort(p2);
                Self::Line(p1, p2)
            },
            Self::Circle(_,_) => self,
            Self::Rectangle(_,_) => self,
            Self::Triangle(p1,p2,p3) => {
                let (p1, p2) = p1.sort(p2);
                let (p2, p3) = p2.sort(p3);
                let (p1, p2) = p1.sort(p2);
                Self::Triangle(p1, p2, p3)
            },
        }
    }
}

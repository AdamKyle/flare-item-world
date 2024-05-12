/// Defined the shape of a rectangle.
/// x1,x2 are the top and botom while y1,y2 are the sides.
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Clone for Rect {
    fn clone(&self) -> Self {
        // Implement the cloning logic for Rect
        // Typically involves cloning each field individually
        // For example:
        Rect {
            x1: self.x1,
            x2: self.x2,
            y1: self.y1,
            y2: self.y2,
        }
    }
}

/// Impleent the Rectangle
impl Rect {
    // Create a new rectangle.
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    // Gets the center coordinates of the rectangle.
    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

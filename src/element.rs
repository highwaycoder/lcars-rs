use crate::math::vector::Vector2D;
use crate::color::Color;

pub struct Element {
    pub position: Vector2D,
    pub size: Vector2D,
    pub color: Color,
    pub children: Vec<Element>,
}

impl Element {
    pub fn new(position: Vector2D, size: Vector2D, color: Color) -> Element {
        Element {
            position,
            size,
            color,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Element) {
        self.children.push(child);
    }
}

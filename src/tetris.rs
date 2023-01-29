use std::{collections::HashSet, mem};

use crate::shape::{Pos, Shape};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}
#[derive(Debug)]
pub struct Tetris {
    width: i32,
    height: i32,
    current_shape: Shape,
    fixed_shapes: Vec<Shape>,
    lost: bool,
}

/// Represents game state
impl Tetris {
    /// constructor
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width as i32,
            height: height as i32,
            current_shape: &Shape::new_random() + Pos((width as i32 - 1) / 2, 0),
            fixed_shapes: vec![],
            lost: false,
        }
    }

    /// check if given shape is out of bound
    pub fn is_oob(&self, shape: &Shape) -> bool {
        !shape
            .positions()
            .all(|pos| 0 <= pos.0 && pos.0 < self.width && 0 <= pos.1 && pos.1 < self.height)
    }

    /// An iterator over all position on the board (0, 0) to (width, height)
    pub fn iter_positions(&self) -> impl Iterator<Item = Pos> {
        let height = self.height;
        let width = self.width;
        (0..height).flat_map(move |y| (0..width).map(move |x| Pos(x, y)))
    }

    pub fn get(&self, pos: Pos) -> Option<&'static str> {
        if self.current_shape.has_position(&pos) {
            Some(self.current_shape.typ())
        } else {
            self.fixed_shapes
                .iter()
                .find(|shape| shape.has_position(&pos))
                .map(|shape| shape.typ())
        }
    }

    /// check if given shape is colliding with existing shapes
    pub fn is_colliding(&self, shape: &Shape) -> bool {
        self.fixed_shapes
            .iter()
            .any(|fixed_shape| fixed_shape.collides_with(shape))
    }

    pub fn is_line_full(&self, y: i32) -> bool {
        self.fixed_shapes
            .iter() // for each Shape in fixed shapes
            .flat_map(|shape| shape.positions()) // for each Pos in each Shape
            .filter(|pos| pos.1 == y) // get Pos's that are on this line
            .collect::<HashSet<_>>() // remove dupes
            .len() as i32 // count
            == self.width
    }

    pub fn remove_line(&mut self, y: i32) {
        self.fixed_shapes
            .iter_mut()
            .for_each(|shape| shape.remove_line(y));
    }

    pub fn remove_full_line(&mut self) {
        for y in 0..self.height {
            if self.is_line_full(y) {
                self.remove_line(y);
            }
        }
    }

    pub fn tick(&mut self) {
        if self.lost {
            return;
        }
        let translated_cur_shape = &self.current_shape + Pos(0, 1);
        if self.is_oob(&translated_cur_shape) || self.is_colliding(&translated_cur_shape) {
            let new_fixed_shape = mem::replace(
                &mut self.current_shape,
                &Shape::new_random() + Pos((self.width - 1) / 2, 0),
            );

            self.fixed_shapes.push(new_fixed_shape);
            self.remove_full_line();

            if self.is_colliding(&self.current_shape) {
                self.lost = true;
            }
        } else {
            self.current_shape = translated_cur_shape;
        }
    }

    pub fn shift(&mut self, direction: Direction) {
        let translated_cur_shape = &self.current_shape
            + match direction {
                Direction::Left => Pos(-1, 0),
                Direction::Right => Pos(1, 0),
            };
        if !self.is_oob(&translated_cur_shape) && !self.is_colliding(&translated_cur_shape) {
            self.current_shape = translated_cur_shape;
        }
    }

    // TBD: direction of rotation?
    pub fn rotate(&mut self) {
        if self.lost {
            return;
        }
        let rot_cur_shape = self.current_shape.rotate();
        if !self.is_oob(&rot_cur_shape) && !self.is_colliding(&rot_cur_shape) {
            self.current_shape = rot_cur_shape;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tetris;

    #[test]
    fn test() {
        let mut tet = Tetris::new(10, 30);
        println!("{:#?}", tet);
        tet.tick();
        println!("{:#?}", tet);
    }
}

use std::collections::HashSet;
use std::iter::Copied;
use std::ops::Add;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos(pub i32, pub i32);

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone)]
pub struct Shape {
    t: &'static str,
    position: HashSet<Pos>,
    anchor: Pos,
}

macro_rules! impl_shape_constructor {
    ($( $new:ident $t:literal: [ $( $pos:expr),* ] anchored at $anchor:expr; )*) => {
        $(
            pub fn $new() -> Self {
                Self {
                    t: $t,
                    position: [$( $pos ), *].into_iter().collect(),
                    anchor: $anchor,
                }
            }
        )*
    };
}

impl Shape {
    impl_shape_constructor! {
        new_i "🟦" : [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)] anchored at Pos(1, 0);
        new_0 "🟨" : [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)] anchored at Pos(0, 0);
        new_t "🟫" : [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(1, 1)] anchored at Pos(0, 0);
        new_j "🟪" : [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(-1, 2)] anchored at Pos(0, 1);
        new_l "🟧" : [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 2)] anchored at Pos(0, 1);
        new_s "🟩" : [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(-1, 1)] anchored at Pos(0, 0);
        new_z "🟥" : [Pos(0, 0), Pos(-1, 0), Pos(0, 1), Pos(1, 1)] anchored at Pos(0, 0);
    }

    pub fn new_random() -> Self {
        let random = (rand::random::<f64>() * 7.0).floor() as u8;
        match random {
            0 => Self::new_i(),
            1 => Self::new_0(),
            2 => Self::new_t(),
            3 => Self::new_j(),
            4 => Self::new_l(),
            5 => Self::new_s(),
            6 => Self::new_z(),
            _ => unreachable!(),
        }
    }

    pub fn positions(&self) -> Copied<std::collections::hash_set::Iter<'_, Pos>> {
        return self.position.iter().copied();
    }

    pub fn collides_with(&self, other: &Shape) -> bool {
        self.position.intersection(&other.position).count() > 0
    }
    // pub fn positions(&self) -> impl Iterator<Item = Pos> + '_ {
    //     return self.position.iter().copied()
    // }

    pub fn rotate(&self) -> Self {
        let Pos(a, b) = self.anchor;
        let new_pos = self
            .positions()
            .map(|Pos(x, y)| Pos(-y + b + a, x - a + b))
            .collect();
        Self {
            t: self.t,
            position: new_pos,
            anchor: self.anchor,
        }
    }

    pub fn typ(&self) -> &'static str {
        self.t
    }

    pub fn has_position(&self, pos: &Pos) -> bool {
        self.position.contains(pos)
    }

    pub fn remove_line(&mut self, y: i32) {
        self.position.retain(|pos| pos.1 != y);
        self.position = self
            .position
            .iter()
            .copied()
            .map(|pos| {
                if pos.1 >= y {
                    pos
                } else {
                    Pos(pos.0, pos.1 + 1)
                }
            })
            .collect();
    }
}

impl Add<Pos> for &Shape {
    type Output = Shape;
    fn add(self, rhs: Pos) -> Self::Output {
        Shape {
            t: self.t,
            position: self.position.iter().map(|&pos| pos + rhs).collect(),
            anchor: self.anchor + rhs,
        }
    }
}

use piet_scene::geometry::Point;

pub trait Lerp {
    fn lerp(&self, other: &Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        *self + (*other - *self) * t
    }
}

impl Lerp for Point {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self::new(
            Lerp::lerp(&self.x, &other.x, t),
            Lerp::lerp(&self.y, &other.y, t),
        )
    }
}

pub fn lerp_iter<'a, T: Copy + Clone + Lerp>(
    start: &'a [T],
    end: &'a [T],
    t: f32,
) -> impl Iterator<Item = T> + 'a + Clone {
    start.iter().zip(end.iter()).map(move |(a, b)| a.lerp(b, t))
}

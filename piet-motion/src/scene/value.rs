use piet_scene::brush::Color;
use piet_scene::geometry::Point;

pub trait Value: Clone + core::fmt::Debug {
    fn lerp_value(&self, other: &Self, t: f32) -> Self;
}

impl Value for f32 {
    fn lerp_value(&self, other: &Self, t: f32) -> Self {
        *self + (*other - *self) * t
    }
}

impl Value for Point {
    fn lerp_value(&self, other: &Self, t: f32) -> Self {
        Self::new(
            Value::lerp_value(&self.x, &other.x, t),
            Value::lerp_value(&self.y, &other.y, t),
        )
    }
}

pub fn lerp_value_iter<'a, T: Copy + Clone + Value>(
    start: &'a [T],
    end: &'a [T],
    t: f32,
) -> impl Iterator<Item = T> + 'a + Clone {
    start
        .iter()
        .zip(end.iter())
        .map(move |(a, b)| a.lerp_value(b, t))
}

#[derive(Copy, Clone, Default, Debug)]
pub struct ColorF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColorF {
    pub fn to_color(&self) -> Color {
        fn f(x: f32) -> u8 {
            (x.max(0.0).min(1.0) * 255.0).round() as u8
        }
        Color {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
            a: f(self.a),
        }
    }
}

impl Value for ColorF {
    fn lerp_value(&self, other: &Self, t: f32) -> Self {
        Self {
            r: self.r.lerp_value(&other.r, t),
            g: self.g.lerp_value(&other.g, t),
            b: self.b.lerp_value(&other.b, t),
            a: self.a.lerp_value(&other.a, t),
        }
    }
}

#[derive(Clone, Debug)]
pub enum MaybeAnimated<T: Value> {
    Fixed(T),
    Animated(KeyframeValues<T>),
}

impl<T: Value> MaybeAnimated<T> {
    pub fn is_fixed(&self) -> bool {
        match self {
            Self::Fixed(_) => true,
            _ => false,
        }
    }
    
    pub fn evaluate(&self, time: f32) -> Option<T> {
        match self {
            Self::Fixed(value) => Some(value.clone()),
            Self::Animated(values) => values.evaluate(time),
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct KeyframeTime {
    pub time: f32,
}

impl KeyframeTime {
    pub fn find_frames(times: &[KeyframeTime], time: f32) -> Option<([usize; 2], f32)> {
        if times.is_empty() {
            return None;
        }
        use core::cmp::Ordering::*;
        let ix = match times.binary_search_by(|x| {
            if x.time < time {
                Less
            } else if x.time > time {
                Greater
            } else {
                Equal
            }
        }) {
            Ok(ix) | Err(ix) => ix,
        };
        let ix0 = ix.min(times.len() - 1);
        let ix1 = (ix + 1).min(times.len() - 1);
        let t0 = times[ix0].time;
        let t1 = times[ix1].time;
        let t = (t1 - t0) / (time - t0);
        Some(([ix0, ix1], t))
    }
}

#[derive(Clone, Debug)]
pub struct KeyframeValues<T: Value> {
    pub times: Vec<KeyframeTime>,
    pub values: Vec<T>,
}

impl<T: Value> KeyframeValues<T> {
    pub fn evaluate(&self, time: f32) -> Option<T> {
        let ([ix0, ix1], t) = KeyframeTime::find_frames(&self.times, time)?;
        Some(self.values.get(ix0)?.lerp_value(self.values.get(ix1)?, t))
    }
}

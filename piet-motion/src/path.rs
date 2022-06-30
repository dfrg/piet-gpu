use piet_scene::{geometry::Point, path::Element};

use bodymovin::properties::{Shape, ShapeKeyframe, ShapeValue};

pub fn shape_to_path<'a>(shape: &'a ShapeValue) -> impl Iterator<Item = Element> + 'a + Clone {
    let is_closed = shape.closed == Some(true);
    let shape = ShapeRef {
        in_points: &shape.in_point,
        out_points: &shape.out_point,
        vertices: &shape.vertices,
    };
    ShapeElements::new(shape, is_closed).unwrap_or_default()
}

pub fn shape_keyframes_to_path<'a>(
    keyframes: &'a [ShapeKeyframe],
    time: f64,
) -> impl Iterator<Item = Element> + 'a + Clone {
    if let Some((shape, target, t)) = find_keyframes(keyframes, time) {
        let is_closed = shape.closed == Some(true);
        let shape = ShapeRef {
            in_points: &shape.in_point,
            out_points: &shape.out_point,
            vertices: &shape.vertices,
        };
        let target = ShapeRef {
            in_points: &target.in_point,
            out_points: &target.out_point,
            vertices: &target.vertices,
        };
        ShapeElements::new_lerp(shape, target, is_closed, t).unwrap_or_default()
    } else {
        ShapeElements::default()
    }
}

fn find_keyframes<'a>(
    keyframes: &'a [ShapeKeyframe],
    time: f64,
) -> Option<(&'a ShapeValue, &'a ShapeValue, f64)> {
    if keyframes.is_empty() {
        return None;
    }
    // TODO: linear search.. fixme
    let mut index = keyframes.len() - 1;
    for (i, keyframe) in keyframes.iter().enumerate().rev() {
        if keyframe.start_time < time {
            index = i;
            break;
        }
    }
    let index2 = (index + 1).min(keyframes.len() - 1);
    let shape_frame = &keyframes[index];
    let target_frame = &keyframes[index2];
    let shape = shape_frame.start_value.as_ref().map(|x| x.get(0))??;
    let target = target_frame.start_value.as_ref().map(|x| x.get(0))??;
    let t0 = shape_frame.start_time;
    let t1 = target_frame.start_time;
    Some((shape, target, (time - t0) / (t1 - t0)))
}

fn make_point(p: (f64, f64)) -> Point {
    Point::new(p.0 as f32, p.1 as f32)
}

fn lerp_point(a: (f64, f64), b: (f64, f64), t: f64) -> Point {
    let x = a.0 + (b.0 - a.0) * t;
    let y = a.1 + (b.1 - a.1) * t;
    make_point((x, y))
}

#[derive(Copy, Clone, Default)]
struct ShapeRef<'a> {
    in_points: &'a [(f64, f64)],
    out_points: &'a [(f64, f64)],
    vertices: &'a [(f64, f64)],
}

impl<'a> ShapeRef<'a> {
    fn len(&self) -> usize {
        self.vertices.len()
    }

    fn first(&self) -> Option<Point> {
        Some(make_point(*self.vertices.get(0)?))
    }

    fn lerp_first(&self, other: &ShapeRef, t: f64) -> Option<Point> {
        Some(lerp_point(
            *self.vertices.get(0)?,
            *other.vertices.get(0)?,
            t,
        ))
    }

    fn last(&self) -> Option<Element> {
        let a = make_point(*self.out_points.last()?);
        let b = make_point(*self.in_points.get(0)?);
        let c = make_point(*self.vertices.get(0)?);
        Some(Element::CurveTo(a, b, c))
    }

    fn lerp_last(&self, other: &ShapeRef, t: f64) -> Option<Element> {
        let a = lerp_point(*self.out_points.last()?, *other.out_points.last()?, t);
        let b = lerp_point(*self.in_points.get(0)?, *other.in_points.get(0)?, t);
        let c = lerp_point(*self.vertices.get(0)?, *other.vertices.get(0)?, t);
        Some(Element::CurveTo(a, b, c))
    }

    fn get(&self, index: usize) -> Option<Element> {
        let a = make_point(*self.out_points.get(index - 1)?);
        let b = make_point(*self.in_points.get(index)?);
        let c = make_point(*self.vertices.get(index)?);
        Some(Element::CurveTo(a, b, c))
    }

    fn lerp(&self, other: &ShapeRef, index: usize, t: f64) -> Option<Element> {
        let a = lerp_point(
            *self.out_points.get(index - 1)?,
            *other.out_points.get(index - 1)?,
            t,
        );
        let b = lerp_point(*self.in_points.get(index)?, *other.in_points.get(index)?, t);
        let c = lerp_point(*self.vertices.get(index)?, *other.vertices.get(index)?, t);
        Some(Element::CurveTo(a, b, c))
    }
}

#[derive(Clone)]
enum State {
    Initial(Point, Option<Element>),
    Inner(usize, Option<Element>),
    Final(Element),
    Close,
    Done,
}

impl Default for State {
    fn default() -> Self {
        Self::Done
    }
}

#[derive(Clone, Default)]
struct ShapeElements<'a> {
    shape: ShapeRef<'a>,
    lerp_target: Option<(ShapeRef<'a>, f64)>,
    state: State,
}

impl<'a> ShapeElements<'a> {
    pub fn new(shape: ShapeRef<'a>, is_closed: bool) -> Option<Self> {
        let first = shape.first()?;
        let last = if is_closed { Some(shape.last()?) } else { None };
        Some(Self {
            shape,
            lerp_target: None,
            state: State::Initial(first, last),
        })
    }

    pub fn new_lerp(
        shape: ShapeRef<'a>,
        target: ShapeRef<'a>,
        is_closed: bool,
        t: f64,
    ) -> Option<Self> {
        let first = shape.lerp_first(&target, t)?;
        let last = if is_closed {
            Some(shape.lerp_last(&target, t)?)
        } else {
            None
        };
        Some(Self {
            shape,
            lerp_target: Some((target, t)),
            state: State::Initial(first, last),
        })
    }
}

impl<'a> Iterator for ShapeElements<'a> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Initial(point, close) => {
                self.state = State::Inner(1, close);
                Some(Element::MoveTo(point))
            }
            State::Inner(index, close) => {
                let el = if let Some((other, t)) = &self.lerp_target {
                    self.shape.lerp(other, index, *t)?
                } else {
                    self.shape.get(index)?
                };
                if index + 1 < self.shape.len() {
                    self.state = State::Inner(index + 1, close);
                } else {
                    self.state = if let Some(close) = close {
                        State::Final(close)
                    } else {
                        State::Done
                    };
                }
                Some(el)
            }
            State::Final(el) => {
                self.state = State::Close;
                Some(el)
            }
            State::Close => {
                self.state = State::Done;
                Some(Element::Close)
            }
            State::Done => None,
        }
    }
}

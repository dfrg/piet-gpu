use super::transform::*;
use super::value::*;
use bodymovin::properties::*;
use piet_scene::geometry::{Affine, Point};

pub fn convert_transform(transform: &bodymovin::helpers::Transform) -> Transform {
    let animated = AnimatedTransform {
        anchor: convert_pair(&transform.anchor_point),
        position: convert_pair(&transform.position),
        scale: convert_pair(&transform.scale),
        rotation: convert_scalar(&transform.rotation),
        skew: convert_scalar(&transform.skew),
        skew_angle: convert_scalar(&transform.skew_axis),
    };
    if animated.is_fixed() {
        Transform::Fixed(animated.evaluate(0.0).unwrap_or(Affine::IDENTITY))
    } else {
        Transform::Animated(Box::new(animated))
    }
}

pub fn convert_shape_transform(transform: &bodymovin::shapes::Transform) -> Transform {
    let animated = AnimatedTransform {
        anchor: convert_pair(&transform.anchor_point),
        position: convert_pair(&transform.position),
        scale: convert_pair(&transform.scale),
        rotation: convert_scalar(&transform.rotation),
        skew: convert_scalar(&transform.skew),
        skew_angle: convert_scalar(&transform.skew_axis),
    };
    if animated.is_fixed() {
        Transform::Fixed(animated.evaluate(0.0).unwrap_or(Affine::IDENTITY))
    } else {
        Transform::Animated(Box::new(animated))
    }
}

pub fn convert_scalar(value: &bodymovin::properties::Scalar) -> MaybeAnimated<f32> {
    use bodymovin::properties::Value::*;
    match &value.value {
        Fixed(value) => MaybeAnimated::Fixed(*value as f32),
        Animated(frames) => {
            let mut times = vec![];
            let mut values = vec![];
            for frame in frames {
                times.push(KeyframeTime {
                    time: frame.start_time as f32,
                });
                values.push(frame.start_value.map(|x| x.0).unwrap_or(0.0) as f32);
            }
            MaybeAnimated::Animated(KeyframeValues { times, values })
        }
    }
}

pub fn convert_pair(value: &bodymovin::properties::MultiDimensional) -> MaybeAnimated<Point> {
    use bodymovin::properties::Value::*;
    fn extract(x: &Vec<f64>) -> Point {
        Point::new(
            x.get(0).copied().unwrap_or(0.0) as f32,
            x.get(1).copied().unwrap_or(0.0) as f32,
        )
    }
    match &value.value {
        Fixed(value) => MaybeAnimated::Fixed(extract(value)),
        Animated(frames) => {
            let mut times = vec![];
            let mut values = vec![];
            for frame in frames {
                times.push(KeyframeTime {
                    time: frame.start_time as f32,
                });
                values.push(
                    frame
                        .start_value
                        .as_ref()
                        .map(|x| extract(x))
                        .unwrap_or(Point::default()),
                );
            }
            MaybeAnimated::Animated(KeyframeValues { times, values })
        }
    }
}

pub fn convert_color(value: &bodymovin::properties::MultiDimensional) -> MaybeAnimated<ColorF> {
    use bodymovin::properties::Value::*;
    fn extract(x: &Vec<f64>) -> ColorF {
        ColorF {
            r: x.get(0).copied().unwrap_or(0.0) as f32,
            g: x.get(1).copied().unwrap_or(0.0) as f32,
            b: x.get(2).copied().unwrap_or(0.0) as f32,
            a: x.get(3).copied().unwrap_or(1.0) as f32,
        }
    }
    match &value.value {
        Fixed(value) => MaybeAnimated::Fixed(extract(value)),
        Animated(frames) => {
            let mut times = vec![];
            let mut values = vec![];
            for frame in frames {
                times.push(KeyframeTime {
                    time: frame.start_time as f32,
                });
                values.push(
                    frame
                        .start_value
                        .as_ref()
                        .map(|x| extract(x))
                        .unwrap_or(ColorF {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                );
            }
            MaybeAnimated::Animated(KeyframeValues { times, values })
        }
    }
}

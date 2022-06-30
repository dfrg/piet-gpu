use super::value::*;
use piet_scene::geometry::{Affine, Point};

#[derive(Clone, Debug)]
pub struct AnimatedTransform {
    pub anchor: MaybeAnimated<Point>,
    pub position: MaybeAnimated<Point>,
    pub rotation: MaybeAnimated<f32>,
    pub scale: MaybeAnimated<Point>,
    pub skew: MaybeAnimated<f32>,
    pub skew_angle: MaybeAnimated<f32>,
}

impl AnimatedTransform {
    pub fn is_fixed(&self) -> bool {
        self.anchor.is_fixed()
            && self.position.is_fixed()
            && self.rotation.is_fixed()
            && self.scale.is_fixed()
            && self.skew.is_fixed()
            && self.skew_angle.is_fixed()
    }

    pub fn evaluate(&self, time: f32) -> Option<Affine> {
        let anchor = self.anchor.evaluate(time)?;
        let position = self.position.evaluate(time)?;
        let rotation = self.rotation.evaluate(time)?;
        let scale = self.scale.evaluate(time)?;
        let skew = self.skew.evaluate(time)?;
        let skew_angle = self.skew_angle.evaluate(time)?;
        let skew_matrix = if skew != 0.0 {
            const SKEW_LIMIT: f32 = 85.0;
            let skew = -skew.min(SKEW_LIMIT).max(-SKEW_LIMIT);
            let angle = skew_angle.to_radians();
            Affine::rotate(angle) * Affine::skew(skew, 0.0) * Affine::rotate(-angle)
        } else {
            Affine::IDENTITY
        };
        Some(
            Affine::translate(position.x, position.y)
                * Affine::rotate(rotation.to_radians())
                * skew_matrix
                * Affine::scale(scale.x / 100.0, scale.y / 100.0)
                * Affine::translate(-anchor.x, -anchor.y),
        )
    }
}

#[derive(Clone, Debug)]
pub enum Transform {
    Fixed(Affine),
    Animated(Box<AnimatedTransform>),
}

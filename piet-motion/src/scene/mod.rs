
mod anim;
pub mod convert;
mod data;
mod lottie;
mod keyframe;
mod shape;
pub mod transform;
pub mod value;

use piet_scene::geometry::Affine;

pub use data::{BrushId, PathId, TransformId};

#[derive(Clone, Debug)]
pub enum Command {
    PushTransform(Affine),
    PushAnimatedTransform(TransformId),
    PopTransform,
    Fill(PathId, BrushId),
    FillAnimated,
}



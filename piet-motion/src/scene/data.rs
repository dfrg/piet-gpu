use piet_scene::path::{Element, Verb};
use piet_scene::geometry::{Affine, Point, Rect};

use core::ops::Range;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PathId(usize);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct TransformId(usize);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BrushId(usize);

#[derive(Clone, Debug)]
struct PathData {
    verbs: Range<usize>,
    points: Range<usize>,
}

#[derive(Default, Debug)]
pub struct ScenePathData {
    verbs: Vec<Verb>,
    points: Vec<Point>,
    paths: Vec<PathData>,
}



impl ScenePathData {
    pub fn add(&mut self, verbs: &[Verb], points: &[Point]) -> PathId {
        let id = PathId(self.paths.len());
        let verb_start = self.verbs.len();
        let point_start = self.points.len();
        self.verbs.extend(verbs);
        self.points.extend(points);
        self.paths.push(PathData {
            verbs: verb_start..self.verbs.len(),
            points: point_start..self.points.len(),
        });
        id
    }

    pub fn clear(&mut self) {
        self.verbs.clear();
        self.points.clear();
        self.paths.clear();
    }
}

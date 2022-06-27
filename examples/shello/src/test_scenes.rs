use piet_scene::{brush::*, geometry::*, glyph::*, path::*, resource::*, scene::*};

pub fn render(scene: &mut Scene, rcx: &mut ResourceContext, which: usize, arg: u64) {
    match which {
        _ => basic_scene(scene, rcx, arg),
    }
}

fn basic_scene(scene: &mut Scene, rcx: &mut ResourceContext, arg: u64) {
    let mut builder = build_scene(scene, rcx);
    builder.push_transform(Affine::translate(400.0, 400.0) * Affine::rotate((arg as f64 * 0.01) as f32));
    builder.fill(
        Fill::NonZero,
        &Brush::Solid(Color::rgb8(0, 255, 0)),
        None,
        Rect {
            min: Point::new(0.0, 0.0),
            max: Point::new(400.0, 200.0)
        }.elements()
    );
    builder.pop_transform();
    builder.finish();
}


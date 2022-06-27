use super::text::*;
use parley::FontContext;
use piet_scene::{brush::*, geometry::*, resource::*, scene::*};

pub fn render(
    fcx: &mut FontContext,
    scene: &mut Scene,
    rcx: &mut ResourceContext,
    which: usize,
    arg: u64,
) {
    match which {
        _ => basic_scene(fcx, scene, rcx, arg),
    }
}

fn basic_scene(fcx: &mut FontContext, scene: &mut Scene, rcx: &mut ResourceContext, arg: u64) {
    let mut builder = build_scene(scene, rcx);
    builder.push_transform(
        Affine::translate(400.0, 400.0) * Affine::rotate((arg as f64 * 0.01) as f32),
    );
    let stops = &[
        Stop {
            offset: 0.0,
            color: Color::rgb8(128, 0, 0),
        },
        Stop {
            offset: 0.5,
            color: Color::rgb8(0, 128, 0),
        },
        Stop {
            offset: 1.0,
            color: Color::rgb8(0, 0, 128),
        },
    ][..];
    let gradient = Brush::LinearGradient(LinearGradient {
        start: Point::new(0.0, 0.0),
        end: Point::new(0.0, 400.0),
        extend: Extend::Pad,
        stops: stops.iter().copied().collect(),
    });
    builder.fill(
        Fill::NonZero,
        &gradient,
        None,
        Rect {
            min: Point::new(0.0, 0.0),
            max: Point::new(600.0, 400.0),
        }
        .elements(),
    );
    builder.pop_transform();
    let scale = (arg as f64 * 0.01).sin() * 0.5 + 1.5;
    let mut lcx = parley::LayoutContext::new();
    let mut layout_builder =
        lcx.ranged_builder(fcx, "Hello piet-gpu! ഹലോ ਸਤ ਸ੍ਰੀ ਅਕਾਲ مرحبا!", scale as f32);
    layout_builder.push_default(&parley::style::StyleProperty::FontSize(34.0));
    layout_builder.push(
        &parley::style::StyleProperty::Brush(ParleyBrush(Brush::Solid(Color::rgb8(255, 255, 0)))),
        6..10,
    );
    layout_builder.push(&parley::style::StyleProperty::FontSize(48.0), 6..10);
    layout_builder.push_default(&parley::style::StyleProperty::Brush(ParleyBrush(
        Brush::Solid(Color::rgb8(255, 255, 255)),
    )));
    let mut layout = layout_builder.build();
    layout.break_all_lines(None, parley::layout::Alignment::Start);
    builder.push_transform(Affine::translate(100.0, 400.0));
    render_text(&mut builder, &layout);
    builder.pop_transform();
    builder.finish();
}

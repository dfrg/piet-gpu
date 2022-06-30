use piet_scene::geometry::Point;
use piet_scene::path::Element;

pub fn ellipse_to_path(
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
) -> impl Iterator<Item = Element> + Clone {
    let a = 0.551915024494;
    let arx = a * rx;
    let ary = a * ry;
    let elements = [
        Element::MoveTo(Point::new(cx + rx, cy)),
        Element::CurveTo(
            Point::new(cx + rx, cy + ary),
            Point::new(cx + arx, cy + ry),
            Point::new(cx, cy + ry),
        ),
        Element::CurveTo(
            Point::new(cx - arx, cy + ry),
            Point::new(cx - rx, cy + ary),
            Point::new(cx - rx, cy),
        ),
        Element::CurveTo(
            Point::new(cx - rx, cy - ary),
            Point::new(cx - arx, cy - ry),
            Point::new(cx, cy - ry),
        ),
        Element::CurveTo(
            Point::new(cx + arx, cy - ry),
            Point::new(cx + rx, cy - ary),
            Point::new(cx + rx, cy),
        ),
        Element::Close,
    ];
    elements.into_iter()
}

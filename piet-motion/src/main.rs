use bodymovin::layers::*;
use bodymovin::shapes::*;
use bodymovin::sources::Asset;
use bodymovin::Bodymovin;

fn main() {
    let scene = Bodymovin::load("/users/chad/code/data/lottie/rocket.json").unwrap();
    // println!("{:#?}", scene);
    traverse(&scene);
}

fn traverse(scene: &Bodymovin) {
    for layer in &scene.layers {
        traverse_layer(scene, layer, 0);
    }
}

fn traverse_layer(scene: &Bodymovin, layer: &AnyLayer, depth: usize) {
    match layer {
        AnyLayer::PreComp(precomp) => {
            traverse_precomp(scene, &precomp.mixin.ref_id, depth);
        }
        AnyLayer::Shape(shape) => {
            wd(depth);
            println!("[Layer {:?} {}->{} ({:?}) ({:?})]", shape.name, shape.in_point, shape.out_point, shape.blend_mode, piet_motion::scene::convert::convert_transform(&shape.transform));
            traverse_shapes(scene, &shape.mixin.shapes, depth + 1);
        }
        _ => {}
    }
}

fn traverse_shapes(scene: &Bodymovin, shapes: &[AnyShape], depth: usize) {
    let mut buf = "".to_owned();
    for shape in shapes {
        match shape {
            AnyShape::Group(group) => {
                traverse_shapes(scene, &group.items, depth);
                return;
            }
            AnyShape::Shape(_) => {
                buf += "<path>";
            }
            AnyShape::Ellipse(_) => {
                buf += "<ellipse>";
            }
            AnyShape::Star(_) => {
                buf += "<star>";
            }
            AnyShape::Rect(_) => {
                buf += "<rect>";
            }
            AnyShape::Fill(_) => buf += "F",
            AnyShape::Stroke(_) => buf += "S",
            AnyShape::GradientFill(_) => buf += "Gf",
            AnyShape::GradientStroke(_) => buf += "Gs",
            AnyShape::Transform(x) => buf += &format!("X({:?})", piet_motion::scene::convert::convert_shape_transform(x)),
            AnyShape::Merge(_) => buf += "M",
            AnyShape::Trim(_) => buf += "T",
            AnyShape::Repeater(_) => buf += "*",
            AnyShape::RoundedCorners(_) => buf += "C",
        }
    }
    wd(depth);
    println!("{}", buf);
}

fn traverse_precomp(scene: &Bodymovin, id: &str, depth: usize) {
    for asset in &scene.assets {
        match asset {
            Asset::PreComp(precomp) => {
                if &precomp.id == id {
                    for layer in &precomp.layers {
                        traverse_layer(scene, layer, depth + 1);
                    }
                    return;
                }
            }
            _ => {}
        }
    }
    println!("MISSING ASSET {}", id);
}

fn wd(depth: usize) {
    for _ in 0..depth {
        print!(" ");
    }
}

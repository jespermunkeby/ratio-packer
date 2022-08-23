
use std::env;
use ratio_packer::combination_entity::{CombinationEntity, Direction, Rectangle};
use nannou::{prelude::*};

struct Model {
}

fn model(_app: &App) -> Model {

    Model{
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: Frame){

    let rects = [
        CombinationEntity::new(1., Option::None),
        CombinationEntity::new(1., Option::None),
        CombinationEntity::new(4., Option::None),
        CombinationEntity::new(0.5, Option::None),
        CombinationEntity::new(0.2, Option::None),
    ];

    let mut a = rects[0].clone();
    a.combine(rects[1].clone(), Direction::Horizontal);

    let mut b = rects[2].clone();
    b.combine(rects[3].clone(), Direction::Vertical);
    b.combine(rects[4].clone(), Direction::Horizontal);

    a.combine(b.clone(), Direction::Horizontal);



    let draw = app.draw();
    draw.background().rgb(0.,0., 0.); 
    let factor = 300.;
    for rect_and_id in a.get_rects_with_id(Rectangle::default()){
       
        draw.rect().xy([rect_and_id.rectangle.center().x*factor, rect_and_id.rectangle.center().y*factor].into()).w(rect_and_id.rectangle.w()*factor).h(rect_and_id.rectangle.h()*factor).stroke_color(RED).stroke_weight(1.);
    }

    draw.to_frame(app, &frame).unwrap();
}


fn main() { 
    env::set_var("RUST_LOG", "ratio_packer::combination_entity=debug");
    env_logger::init();

    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
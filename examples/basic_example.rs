
use std::env;
use ratio_packer::combination_entity::{CombinationEntity, Direction};
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
        CombinationEntity::new(1.),
        CombinationEntity::new(1.),
        CombinationEntity::new(4.),
        CombinationEntity::new(0.5),
        CombinationEntity::new(0.2),
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
    for rect in a.get_rects(){
       
        draw.rect().xy([rect.center().x*factor, rect.center().y*factor].into()).w(rect.w()*factor).h(rect.h()*factor).stroke_color(RED).stroke_weight(1.);
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
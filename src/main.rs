
mod combination_entity;

use std::env;
use combination_entity::{CombinationEntity, Direction};
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
    let window = app.window_rect();
    let mouse = &app.mouse;
    let t = &app.time;

    let x = (mouse.x/window.w()) + 0.5;
    let y = (mouse.y/window.h()) + 0.5;

    let rects = [
        CombinationEntity::new(x*10.*(1. + 0.5*(t*1.).sin())),
        CombinationEntity::new(y*5./(1.+ 0.7*(t*4.).cos())),
        CombinationEntity::new(x*0.1*(5.+ 0.1*(t*3.).sin())),
    ];

    let mut a = rects[0].clone();
    a.combine(rects[1].clone(), Direction::Vertical);
    a.combine(rects[1].clone(), Direction::Horizontal);
    a.combine(rects[2].clone(), Direction::Horizontal);

    let mut b = rects[0].clone();
    b.combine(rects[0].clone(), Direction::Horizontal);
    b.combine(rects[2].clone(), Direction::Vertical);
    b.combine(rects[1].clone(), Direction::Vertical);

    let mut c = rects[2].clone();
    c.combine(rects[2].clone(), Direction::Horizontal);
    c.combine(a.clone(), Direction::Vertical);
    c.combine(rects[1].clone(), Direction::Vertical);

    a.combine(b.clone(), Direction::Vertical);
    a.combine(b.clone(), Direction::Horizontal);
    a.combine(b.clone(), Direction::Horizontal);
    a.combine(c.clone(), Direction::Horizontal);



    let draw = app.draw();
    draw.background().rgb(0.,0., 0.); 
    let factor = 300.;
    for rect in a.get_rects(){
        draw.rect().xy([rect.center().x*factor, rect.center().y*factor].into()).w(rect.w()*factor).h(rect.h()*factor).stroke_color(RED).stroke_weight(1.).no_fill();
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
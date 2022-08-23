use std::env;
use std::fs::{read_dir};
use image::DynamicImage;
use image::io::Reader;
use image::imageops::{overlay, resize, FilterType};
use nannou::lyon::geom::euclid::num::Floor;
use nannou::prelude::Float;
use ratio_packer::combination_entity::{CombinationEntity, Direction, Rectangle, Position};
use rand::prelude::*;
use rand::seq::SliceRandom;

fn _combine_multiple_evenly(entities:Vec<CombinationEntity>) -> Vec<CombinationEntity>{

    if entities.len() == 1 {
        return entities;
    } else{
        let mut shuffled = entities.clone();
        shuffled.shuffle(& mut thread_rng());

        if (shuffled.len() %2) != 0{ //make even length
            println!("made even len");
            let popped = shuffled.pop().unwrap();
            shuffled[0].combine(popped, Direction::Vertical);
        }
        
        
        //pair up
        let end_index = shuffled.len();
        let halfway_index = shuffled.len()/2;

        _combine_multiple_evenly(
            shuffled[0..halfway_index].into_iter().zip(&shuffled[halfway_index..end_index]).map(|(a, b)| {
                let mut rng = thread_rng();
                
                //randomly (not optimized for format)
                let direction = if rng.gen::<f32>() < 0.5{ Direction::Horizontal} else {Direction::Vertical};
                
                if rng.gen::<f32>() < 0.5{
                    let mut combined = a.clone();
                    combined.combine(b.clone(), direction);
                    combined
                } else {
                    let mut combined = b.clone();
                    combined.combine(a.clone(), direction);
                    combined
                }
            }).collect()
        )
    }
}

fn combine_multiple_evenly(entities:&Vec<CombinationEntity>) -> Result<CombinationEntity,&'static str>{
    if entities.is_empty() {
        return Err("Expected 'elements' to be non-empty");
    }

    return Result::Ok(_combine_multiple_evenly(entities.clone())[0].clone());
}

fn create_collage(images: &Vec<DynamicImage>) -> DynamicImage{

    let entities = images.into_iter()
    .enumerate()
    .map(|(i, image)|
        CombinationEntity::new(image.height() as f32/image.width() as f32, Option::Some(i as u32))
    ).collect::<Vec<CombinationEntity>>();

    let combined = combine_multiple_evenly(&entities).unwrap();

    let res_w = 1000;
    let width = res_w;
    let height = ((res_w as f32)*(combined.aspect_ratio())).floor() as u32;


    let mut output = DynamicImage::new_rgba8(width, height);

    combined.get_rects_with_id(Rectangle::new(Position::BottomLeft([0.,0.].into()), width as f32, height as f32)).into_iter().for_each(|rect_and_id|{
        let mut corresponding_image: DynamicImage = images[rect_and_id.id.unwrap() as usize].clone();
        let corresponding_rectangle = rect_and_id.rectangle;

        //resize
        corresponding_image = resize(&mut corresponding_image, corresponding_rectangle.w().ceil() as u32, corresponding_rectangle.h().ceil() as u32, FilterType::CatmullRom).into();
        overlay(&mut output, &corresponding_image, corresponding_rectangle.left().floor() as i64, corresponding_rectangle.bottom().floor() as i64);
    });


    output
}

fn main() { 
    env::set_var("RUST_LOG", "ratio_packer::combination_entity=debug");
    env_logger::init();

    let here_path = std::path::Path::new(".")
        .join("examples")
        .join("image_packer");

    let images = read_dir(here_path.join("input_images")).unwrap()
        .map(|path| path.unwrap().path().display().to_string())
        .map(|path|Reader::open(path).unwrap().decode().unwrap()).collect::<Vec<DynamicImage>>();

    let output = create_collage(&images);

    output.save(here_path.join("output.jpg")).unwrap();
}
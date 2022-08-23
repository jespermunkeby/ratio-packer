mod rectangle;

use std::clone;

use log::{debug, error};
pub use rectangle::{Position, Rectangle};
use rectangle::vec2::Vec2;

#[derive(Clone)]
pub struct CombinationEntity{
    aspect_ratio: f32,
    id: Option<u32>,
    combination: Option<Combination>,
}

#[derive(Clone)]
struct Combination{
    elements:(
        Box<CombinationEntity>,
        Box<CombinationEntity>,
    ),

    combination_direction: Direction,
}

#[derive(Clone, Copy)]
pub enum Direction{
    Horizontal,
    Vertical
}

fn combined_ratio(r_a: f32, r_b: f32, direction: Direction) -> f32{
    match direction {
        Direction::Horizontal => (r_a*r_b)/(r_a+r_b),
        Direction::Vertical => r_a+r_b
    }
}

#[test]
fn test_combined_ratio(){
    assert_eq!(2., combined_ratio(1., 1., Direction::Vertical));
    assert_eq!(0.5, combined_ratio(1., 1., Direction::Horizontal));
    assert_eq!(1./3., combined_ratio(1., 0.5, Direction::Horizontal));
    assert_eq!(1., combined_ratio(0.5, 0.5, Direction::Vertical));
}

fn inscribe_ratio(ratio:f32, bound: Rectangle) -> Rectangle{
    let bound_ratio = bound.h()/bound.w();
    let (w, h) = if bound_ratio < ratio {  //vertically bounded
        (bound.h()/ratio,bound.h())
    }  else {                              //horizontally bounded or equal
        (bound.w(),bound.w()*ratio)
    };

    Rectangle::new(Position::Center(bound.center()), 
        w, 
        h
    )
}

#[test]
fn test_inscribe_ratio(){
     for ratio in [0.1, 0.2, 0.5, 0.9, 1.0, 1.1, 1.5, 1.8, 20., 50.]{
         for bound in [
             Rectangle::new(Position::Center([0.,0.].into()), 1., 1.),
             Rectangle::new(Position::Center([-10.,0.].into()), 10., 18.),
             Rectangle::new(Position::Center([10.,10.].into()),  2., 18.),
             Rectangle::new(Position::Center([-10.,-10.].into()), 20., 18.),
             Rectangle::new(Position::Center([40.,60.].into()), 20., 1.),
             Rectangle::new(Position::Center([40.,60.].into()), 1., 40.),

         ] {
            let rect = inscribe_ratio(ratio, bound);

            assert_eq!(rect.center(), bound.center());

            assert!(
                (rect.top() <= bound.top()) ||
                (rect.bottom() >= bound.bottom())
            );

            assert!(
                (rect.right() <= bound.right()) ||
                (rect.left() >= bound.left())
            );

            assert!(
                ((rect.left() == bound.left()) && (rect.right() == bound.right())) ||
                ((rect.top() == bound.top()) && (rect.bottom() == bound.bottom()))
            )
            
         }
     }
}

fn ratios_and_bound_to_rects(ratio_a: f32, ratio_b:f32, bound: Rectangle, direction: Direction) -> (Rectangle, Rectangle){

    // ---------1--------
    // o---------o------o 
    // |    a    |   b  |
    // o---------o------o 
    //     da       db

    let da:f32;
    let db:f32;

    let combined_ratio = combined_ratio(ratio_a, ratio_b, direction);

    let inscribed_rect= inscribe_ratio(
        combined_ratio, bound);

    match direction {
        Direction::Horizontal =>{
            da = (1./ratio_a)/((1./ratio_a) + (1./ratio_b));
            db = 1.-da;

            let a = Rectangle::new(Position::BottomLeft([inscribed_rect.left(), inscribed_rect.bottom()].into()), inscribed_rect.w()*da, inscribed_rect.h());
            let b = Rectangle::new(Position::BottomLeft([a.right(), a.bottom()].into()),inscribed_rect.w()*db, inscribed_rect.h());

            (a,b)
        },
        Direction::Vertical =>{
            da = ratio_a/(ratio_a + ratio_b);
            db = 1.-da;

            

            let a = Rectangle::new(Position::BottomLeft([inscribed_rect.left(), inscribed_rect.bottom()].into()), inscribed_rect.w(), inscribed_rect.h()*da);
            let b = Rectangle::new(Position::BottomLeft([a.left(), a.top()].into()),inscribed_rect.w(), inscribed_rect.h()*db);
            (a,b)
        }
    }

}

#[test]
fn test_ratios_and_bound_to_rects(){
    for (ratio_a, ratio_b) in [(0.1, 0.5), (0.2, 1.), (0.5, 20.), (0.9, 0.8), (1.0, 3.), (1.1, 3.), (1.5, 5.), (1.8, 7.), (20., 10.)]{
        for bound in [
            Rectangle::new(Position::Center([0.,0.].into()), 1., 1.),
            Rectangle::new(Position::Center([-10.,0.].into()), 10., 18.),
            Rectangle::new(Position::Center([10.,10.].into()),  2., 18.),
            Rectangle::new(Position::Center([-10.,-10.].into()), 20., 18.),
            Rectangle::new(Position::Center([40.,60.].into()), 20., 1.),
        ] {
            let horizontal = ratios_and_bound_to_rects(ratio_a, ratio_b, bound, Direction::Horizontal);
            let vertical = ratios_and_bound_to_rects(ratio_a, ratio_b, bound, Direction::Vertical);

            for rect in [horizontal.0, horizontal.1, vertical.0, vertical.1] {
                assert!(
                    (rect.top() <= bound.top()) ||
                    (rect.bottom() >= bound.bottom())
                );
    
                assert!(
                    (rect.right() <= bound.right()) ||
                    (rect.left() >= bound.left())
                );
            }
            
        }
    }
}

#[derive(Clone)]
pub struct RectangleWithID {
    pub id: Option<u32>,
    pub rectangle: Rectangle
}

impl CombinationEntity {

    pub fn new(aspect_ratio: f32, id:Option<u32>) -> CombinationEntity{
        CombinationEntity{
            aspect_ratio,
            combination: Option::None,
            id: id
        }
    }

    pub fn aspect_ratio(&self) -> f32{
        self.aspect_ratio
    }

    pub fn combine(&mut self, other: CombinationEntity, combination_direction: Direction){
        self.combination = Option::Some(Combination{
            elements:(
                Box::new(self.clone()),
                Box::new(other.clone())
            ),

            combination_direction
        });

        self.aspect_ratio = combined_ratio(self.aspect_ratio(), other.aspect_ratio(), combination_direction);
    }

    pub fn get_rects_with_id(&self, bound: Rectangle) -> Vec<RectangleWithID>{

        return match &self.combination {
            Some(combination) => {
                let a = combination.elements.0.to_owned();
                let b = combination.elements.1.to_owned();

                let (bound_a, bound_b) = ratios_and_bound_to_rects(a.aspect_ratio(), b.aspect_ratio(), bound, combination.combination_direction);
                [a.get_rects_with_id(bound_a), b.get_rects_with_id(bound_b)].concat()
            },

            None => {

                [RectangleWithID {
                    id: self.id,
                    rectangle: inscribe_ratio(self.aspect_ratio(), bound),
                }].into()
            }
        };
    }
}
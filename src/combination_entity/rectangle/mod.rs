pub mod vec2;
use vec2::Vec2;
pub enum Position{
    TopLeft(Vec2),
    TopRight(Vec2),
    BottomLeft(Vec2),
    BottomRight(Vec2),
    Center(Vec2)
}

#[derive(Clone, Copy, Debug)]
pub struct Rectangle{
    left: f32,
    right: f32,
    top: f32,
    bottom: f32
}


impl Rectangle{
    pub fn new(position: Position, w:f32, h:f32) -> Rectangle{
        match position {
            Position::TopLeft(p) => Rectangle{
                left: p.x,
                right: p.x+w,
                top: p.y,
                bottom: p.y-h
            },

            Position::TopRight(p) => Rectangle{
                left: p.x-w,
                right: p.x,
                top: p.y,
                bottom: p.y-h
            },

            Position::BottomLeft(p) => Rectangle{
                left: p.x,
                right: p.x+w,
                top: p.y + h,
                bottom: p.y
            },

            Position::BottomRight(p) => Rectangle{
                left: p.x - w,
                right: p.x,
                top: p.y + h,
                bottom: p.y
            },

            Position::Center(p) => Rectangle{
                left: p.x - w/2.,
                right: p.x + w/2.,
                top: p.y + h/2.,
                bottom: p.y - h/2.
            },
        }
    }

    pub fn top(&self) -> f32{
        self.top
    }

    pub fn bottom(&self) -> f32{
        self.bottom
    }

    pub fn left(&self) -> f32{
        self.left
    }

    pub fn right(&self) -> f32{
        self.right
    }

    pub fn center(&self) ->Vec2{
        Vec2 { x: (self.left+self.right)/2., y: (self.top+self.bottom)/2. }
    }

    pub fn w(&self) -> f32{
        self.right - self.left
    }

    pub fn h(&self) -> f32{
        self.top - self.bottom
    }
}

#[test]
fn test_rectangle(){
    // +---+---+   
    // | a | b |
    // +---+---+
    // | c | d |
    // +---+---+ 

    let outer = Rectangle::new(Position::Center([0.5,0.5].into()), 1., 1.);
    let a = Rectangle::new(Position::TopLeft([0.,1.].into()), 0.5, 0.5);
    let b = Rectangle::new(Position::BottomRight([1.,0.5].into()), 0.5, 0.5);
    let c = Rectangle::new(Position::TopRight([0.5,0.5].into()), 0.5, 0.5);
    let d = Rectangle::new(Position::BottomLeft([0.5,0.].into()), 0.5, 0.5);

    assert_eq!(outer.center(), [0.5,0.5].into());
    assert_eq!(outer.left(), 0.);
    assert_eq!(outer.right(), 1.);
    assert_eq!(outer.bottom(), 0.);
    assert_eq!(outer.top(), 1.);

    assert!(
        (
            [0.,1.] == 
            [outer.left(), outer.top()]
        )

        &&

        (
            [0.,1.] == 
            [a.left(), a.top()]
        )
    );

    print!("{}, {}", b.left(), b.top());

    assert!(
        (
            [0.5,1.] == 
            [a.right(), a.top()]
        )

        &&

        (
            [0.5,1.] == 
            [b.left(), b.top()]
        )
    );

    assert!(
        (
            [1.,1.] == 
            [outer.right(), outer.top()]
        )

        &&

        (
            [1.,1.] == 
            [b.right(), b.top()]
        )
    );

    let center = [outer.center().x, outer.center().y];
    assert_eq!([a.right(), a.bottom()], center);
    assert_eq!([b.left(), b.bottom()], center);
    assert_eq!([c.right(), c.top()], center);
    assert_eq!([d.left(), d.top()], center);

}
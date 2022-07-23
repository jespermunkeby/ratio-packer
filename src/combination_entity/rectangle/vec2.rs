#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec2{
    pub x:f32,
    pub y:f32
}

impl Vec2{
}

impl From<[f32;2]> for Vec2{
    fn from(v: [f32;2]) -> Self {
        Vec2{
            x: v[0],
            y: v[1]
        }
    }
}
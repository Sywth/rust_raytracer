use std::f32::consts::PI;

pub trait Angle{
    fn to_radians(&self) -> Self;
    fn to_degrees(&self) -> Self;
}

impl Angle for f32{
    fn to_radians(&self) -> Self {
        return (self / 180.0) * PI ;
    }

    fn to_degrees(&self) -> Self {
        return (self / PI) * 180.0;
    }
}


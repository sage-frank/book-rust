fn main() {
   
    assert_eq!(Point {x:1,y:0} + Point {x:2,y:3}, Point {x:3,y:3});
    assert_eq!(Millimeters(1000) + Meters(1.0), Millimeters(2000));
    assert_eq!(Meters(1.0) + Millimeters(1000), Meters(2.0));

}

#[derive(Debug,PartialEq)]
struct Point {
    x:u32,
    y:u32,
}

use std::ops::Add;

impl Add for Point  {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }


}

#[derive(Debug,PartialEq)]
struct Meters(f32);

#[derive(Debug,PartialEq)]
struct Millimeters(u32);

impl Add<Meters> for Millimeters {
    type Output = Self;

    fn add(self, other: Meters) -> Self {
        Self(self.0 + (other.0 * 1000.0) as u32 )
    }
}

impl Add<Millimeters> for Meters {
    type Output = Self;

    fn add(self, other: Millimeters) -> Self {
        Self(self.0 + (other.0 as f32 / 1000.0))
    }
}
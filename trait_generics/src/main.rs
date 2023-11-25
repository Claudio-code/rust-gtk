use std::ops::{Add, Sub};

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.y,
            y: self.y + rhs.x,
        }
    }
}

// traits is like interfaces in java
// but traits in rust like traits in php too
trait Overview {
    fn overview(&self) -> String;

    fn another_message(&self) {
        println!("ok test");
    }
}

struct DevCourse {
    headline: String,
    author: String,
}

struct CoachCourse {
    headline: String,
    author: String,
}

impl Overview for DevCourse {
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}

impl Overview for CoachCourse {
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}

fn main() {
    let coord_user = Point { x: 5.0, y: 5.0 };
    let coord_seller = Point { x: 2.1, y: 2.2 };

    let coord_user2 = Point { x: 20.2, y: 20.2 };
    let coord_seller2 = Point { x: 11.2, y: 11.2 };

    let result1 = coord_seller + coord_user;
    let result2 = coord_user2 - coord_seller2;

    println!("result1 {:?}", result1);
    println!("result2 {:?}", result2);

    let course1 = DevCourse {
        author: String::from("Claudio"),
        headline: String::from("headline"),
    };
    let course2 = CoachCourse {
        author: String::from("Silva"),
        headline: String::from("headline"),
    };

    print_message(&course1);
    print_message(&course2);
    print_message_over(&course2);
}

fn print_message(overview: &dyn Overview) {
    overview.another_message();
    println!("course {}", overview.overview());
}

fn print_message_over(over: &impl Overview) {
    println!("overview {}", over.overview());
}

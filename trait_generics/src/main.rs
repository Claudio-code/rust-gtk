struct Point<T> {
    x: T,
    y: T,
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
    let coord_user = Point {x: 5.0, y: 5.0};
    let coord_seller = Point {x: 't', y: 'w'};
    
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
}


fn print_message(overview: &dyn Overview) {
    overview.another_message();
    println!("course {}", overview.overview());
}

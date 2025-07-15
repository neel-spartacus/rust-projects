/*fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}*/

//Using tuples
/*fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}*/
/*fn area(width: u32, height: u32) -> u32 {
    width * height
}*/

//Using Structs


/*struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}*/

/*struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}
fn main() {
    let r = &mut Box::new(Rectangle {
        width: 1,
        height: 2
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);

    r.set_width(4);
    let area3 = r.area();
    assert_eq!(8, area3);
    Rectangle::set_width(&mut *r,5);
    let area4 = r.area();
    assert_eq!(10, area4);
}*/

//Good moves & bad moves

/*#[derive(Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn max(self, other: Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Rectangle {
            width: w,
            height: h
        }
    }
    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other);
        *self = max;
    }
}*/

/*fn main() {
    let mut rect = Rectangle { width: 0, height: 1 };
    let other_rect = Rectangle { width: 1, height: 0 };
    rect.set_to_max(other_rect);
}*/

struct Point {
    x: i32,
    y: i32
}
impl Point {
    fn get_x(&mut self) -> &mut i32 {
        &mut self.x
    }
}
fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x = p.get_x();
    *x += 1;
    println!("{} {}", *x, p.y);
}

/*fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}*/

/*fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}*/
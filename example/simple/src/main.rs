struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = 1;
    let p2 = 2;
    let p3 = p1 + p2;
    let mut p4 = Point { x: p3, y: 3 };
    p4.x += p3;

    println!("Point coordinates: ({}, {})", p4.x, p4.y);
}

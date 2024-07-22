fn main() {
    let w: u32 = 32;
    let h: u32 = 32;
    println!("{}", area(w, h));

    let rect1 = (30, 50);
    println!("{}", area_tup(rect1));
    
    let rect2 = Rectangle {
        h: 32,
        w: 23
    };
    println!("rect2 is {:#?}", rect2);

    println!("{}",area_struct(&rect2));
    
    println!("From Method: {}", rect2.area());
    println!("Square {}", Rectangle::square(20).area());

}

fn area(w: u32, h:u32) -> u32 {
    return w*h;
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

#[derive(Debug)]
struct Rectangle {
    w:u32,
    h:u32
}

impl Rectangle {
    fn area(&self) -> u32 {
       return self.h * self.w;
    }

    fn square(size:u32) -> Self {
        return  Self {
            h:size,
            w:size
        };
    }
}
fn area_struct(rectangle: &Rectangle) -> u32{
    return dbg!(rectangle.h * rectangle.w);
}
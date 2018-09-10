struct Rectangle{
    width: u32,
    height: u32
}

fn main() {
    let width = 30;
    let height = 50;

    let rect1 = Rectangle{
        width: 10,
        height: 10,
    };

    println!("Area is {:?}", area(width, height) );
    println!("Area is {:?}", area1(&rect1) );
}

fn area(w:u32, h:u32) -> u32 {
    w*h
}

fn area1(rect: &Rectangle) -> u32{
    &rect.height*&rect.width
}

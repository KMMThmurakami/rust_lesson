struct Rectangle {
    width: u32,
    height: u32,
}

pub fn sub() {
    println!("======ST sec_7======");
    println!("構造体");

    let height: u32 = 5;

    #[rustfmt::skip]
    let mut rectangle: Rectangle = Rectangle { 
        width: 10,
        height,
    };

    println!("{}", rectangle.width);
    println!("{}", rectangle.height);

    rectangle.height = 10;
    println!("{}", rectangle.height);

    println!("======ED sec_7======");
}

struct Rectangle {
    width: u32,
    height: u32,
}

// Rustの構造体メソッドはimplブロックで定義する
impl Rectangle {
    // 構造体メソッドの引数selfは共有参照にしないと
    // 所有権が移動して再利用できなくなる
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 型関連関数＝他の言語でいうstaticメソッド
    // 第一引数にselfを取らない場合は型関連関数
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

// 列挙型
enum Shape {
    Circle,
    Square(u32),                         // タプル型バリアント
    Triangle { base: u32, height: u32 }, // 構造体型バリアント
}

impl Shape {
    fn sample_method(&self) {
        println!("call sample_method");
    }
}

pub fn sub() {
    println!("======ST sec_7======");
    println!("構造体");

    let height: u32 = 5;

    // #[rustfmt::skip]
    // let mut rectangle: Rectangle = Rectangle {
    //     width: 10,
    //     height,
    // };
    let mut rectangle: Rectangle = Rectangle::new(10, height);

    println!("width: {}", rectangle.width);
    println!("height: {}", rectangle.height);

    rectangle.height = 10;
    println!("height: {}", rectangle.height);

    println!("area: {}", rectangle.area());
    println!("area: {}", rectangle.area());

    println!("\n列挙型");
    let c: Shape = Shape::Circle;
    let s: Shape = Shape::Square(1);
    let t: Shape = Shape::Triangle { base: 10, height: 5 };

    c.sample_method();
    s.sample_method();
    t.sample_method();

    match s {
        Shape::Square(size) => println!("Square size: {}", size),
        _ => (),
    }

    match t {
        Shape::Triangle { base, height } => {
            println!("Triangle base: {}, height: {}", base, height);
        }
        _ => (),
    }

    println!("\nOption型");
    // 列挙型で定義された型のひとつ
    // 値が存在しない可能性のある場合に使用
    // Rustにはnullが存在しない→nullの代わり
    // let a: Option<i32> = Some(1);
    // let b: Option<&str> = Some("str");
    // let c: Option<i32> = None;

    let v: Vec<i32> = vec![1, 2, 3];
    let val: Option<&i32> = v.get(1);

    match val {
        // Some(1) => println!("value is {}", 1),
        Some(x) if *x == 1 => println!("value is 1"), // マッチガード
        Some(x) => println!("{}", x),
        _ => println!("None"),
    }

    // if let Some(x) = val {
    //     println!("{}", x)
    // }

    println!("======ED sec_7======");
}

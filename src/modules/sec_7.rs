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

    #[rustfmt::skip]
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

    println!("列挙型");
    let c: Shape = Shape::Circle;
    let s: Shape = Shape::Square(1);
    let t: Shape = Shape::Triangle { base: 10, height: 5 };

    c.sample_method();
    s.sample_method();
    t.sample_method();

    println!("======ED sec_7======");
}

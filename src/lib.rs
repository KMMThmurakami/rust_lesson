//! ライブラリクレートのドキュメント
// 必ずpubにする
// マークダウンが使える
/// ### 使用例
/// ```
/// fn main() {
///     rust_lesson::say_hello();
/// }
/// ```
pub fn say_hello() {
    println!("Hello.");
}

// トレイトの実装
pub mod sample_trait {
    pub trait Shape {
        /// 面積
        fn calc_area(&self) -> f64;
        /// 周囲
        fn calc_perimeter(&self) -> f64;
        fn do_something();
        /// デフォルトメソッド
        fn default_something(&self) -> &str {
            "This is default_something function."
        }
    }

    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    impl Shape for Rectangle {
        fn calc_area(&self) -> f64 {
            self.width * self.height
        }

        fn calc_perimeter(&self) -> f64 {
            self.width * 2.0 + self.height * 2.0
        }

        fn do_something() {
            println!("This is Rectangle function.");
        }

        fn default_something(&self) -> &str {
            "This is Rectangle default_something function."
        }
    }

    pub struct Circle {
        pub radius: f64,
    }

    impl Shape for Circle {
        fn calc_area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }

        fn calc_perimeter(&self) -> f64 {
            self.radius * 2.0 * std::f64::consts::PI
        }

        fn do_something() {
            println!("This is Circle function.");
        }
    }

    // Shapeトレイトを実装した任意の型の共有参照を引数に取る
    pub fn double_area(shape: &impl Shape) -> f64 {
        shape.calc_area() * 2.0
    }
}

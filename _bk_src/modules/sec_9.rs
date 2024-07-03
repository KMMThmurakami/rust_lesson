use std::fmt::{Debug, Display};

use rust_lesson::sample_trait::*;

// このmax関数はi32の比較しか行えない
// fn max(a: i32, b: i32) -> i32 {
//     if a >= b {
//         a
//     } else {
//         b
//     }
// }
// ジェネリクス
// ジェネリック型は慣習でTとすることが多い
fn max<T: PartialOrd + Debug>(a: T, b: T) -> T
// 複雑な場合はwhereで繋げると可読性が上がる
// where
//     T: PartialOrd,
{
    if a >= b {
        // トレイト境界:PartialOrdトレイトを実装しているものの制約をつける
        a
    } else {
        b
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T: PartialOrd + Debug> Point<T> {
    fn max(&self) -> &T {
        if self.x >= self.y {
            // トレイト境界:PartialOrdトレイトを実装しているものの制約をつける
            &self.x
        } else {
            &self.y
        }
    }

    fn print_arg<U: Display>(&self, val: U) {
        println!("{:?}", self.x);
        println!("{}", val);
    }
}

// i32型の構造体だけで使えるメソッド
impl Point<i32> {
    fn min(&self) -> i32 {
        if self.x <= self.y {
            self.x
        } else {
            self.y
        }
    }
}

pub fn sub() {
    println!("======ST sec_9======");
    println!("トレイト");
    // println!("{}", std::f64::consts::PI);

    #[rustfmt::skip]
    let rect: Rectangle = Rectangle {
        width: 4.0,
        height: 5.0,
    };
    #[rustfmt::skip]
    let circle: Circle = Circle {
        radius: 2.0,
    };

    println!("Rectangle area is      {}", rect.calc_area());
    println!("Rectangle perimeter is {}", rect.calc_perimeter());
    Rectangle::do_something();
    println!("Circle area is      {}", circle.calc_area());
    println!("Circle perimeter is {}", circle.calc_perimeter());
    Circle::do_something();

    println!("Rectangle default: {}", rect.default_something());
    println!("Circle default:    {}", circle.default_something());

    println!("Rectangle double area: {}", double_area(&rect));
    println!("Circle double area:    {}", double_area(&circle));

    // Derive属性による継承
    // #[属性名]
    println!("{:?}", (1, 2, 3));

    #[derive(Debug, PartialEq)]
    struct S {
        val1: i32,
        val2: i32,
    }
    println!("{:?}", S { val1: 1, val2: 2 });

    let s1: S = S { val1: 1, val2: 2 };
    let s2: S = S { val1: 1, val2: 2 };
    println!("{}", s1 == s2);

    println!("ジェネリクス");
    println!("{}", max(1, 2));
    println!("{}", max(1.1, 0.2));
    println!("{}", max("a", "b"));

    println!("ジェネリックな列挙型や構造体");
    // Option型はジェネリックな列挙型

    let p1: Point<i32> = Point { x: 1, y: 2 };
    let p2: Point<f64> = Point { x: 1.0, y: 2.0 };
    let p3: Point<&str> = Point { x: "a", y: "b" };
    println!("{:?}", p1.max());
    println!("{:?}", p2.max());
    println!("{:?}", p3.max());

    p1.print_arg("test");
    p1.print_arg(true);

    p1.min();
    // p2.min(); // f64型のPoint構造体なのでエラー

    println!("======ED sec_9======");
}

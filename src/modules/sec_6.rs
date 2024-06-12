use std::rc::Rc;

pub fn sub() {
    println!("======ST sec_6======");
    // println!("スタック");
    // println!("Rustのスタック領域は8MBまで、超えるとスタックオーバーフローを起こす");
    // println!("ヒープ");
    // println!("動的なデータを格納、コンパイル時にサイズが確定している必要はない");
    println!("所有権");
    // 所有者と呼ばれる変数に対応している
    // いかなる時も所有者はひとつ
    // 所有者がスコープから外れたら値は破棄される

    // let a = 100;
    // {
    //     let v1 = vec![1, 2, 3];
    //     println!("{:03?}", v1);

    //     let mut v2 = v1;
    //     println!("{:^3?}", v2);

    //     v2.push(4);
    // }
    // println!("{}", a);

    let mut v1: Vec<i32> = vec![1, 2, 3];
    // ポインタを表示する
    println!("v1         : {:?}", v1);
    println!("v1 ptr     : {:?}", v1.as_ptr());
    println!("v1[0]      : {:p}", &v1[0]);
    println!("v1 len     : {:?}", v1.len());
    println!("v1 capacity: {:?}", v1.capacity());
    v1.push(4);
    println!("v1 ptr     : {:?}", v1.as_ptr());
    println!("v1 len     : {:?}", v1.len());
    println!("v1 capacity: {:?}", v1.capacity());

    // println!("v1 ptr     : {:?}", v1.as_ptr());
    // let v2 = v1;
    // println!("v2 ptr     : {:?}", v2.as_ptr()); // 先頭アドレスが同じ

    let v2: Vec<i32> = v1.clone();
    println!("v1 ptr     : {:?}", v1.as_ptr());
    println!("v2 ptr     : {:?}", v2.as_ptr()); // v1とv2のポインタは異なる

    let s1: String = String::from("Hello");
    let s2: String = String::from("Rust");
    // シャドーイング:変数再利用
    // let (s, s1, s2): (String, String, String) = concat(s1, s2); // 関数から所有権を返してもらう
    let s: String = concat(&s1, &s2); // 参照を渡すことで所有権が関数に移動しなくなる
    println!("{:?}", s);
    println!("{}", s1);
    println!("{}", s2);

    println!("参照");
    let x1: Vec<i32> = vec![1, 2, 3];
    let x2: &Vec<i32> = &x1;
    println!("{:?}", x1);
    println!("{:?}", x2);
    println!("x1 ptr: {:?}", x1.as_ptr());
    println!("x2 ptr: {:?}", x2.as_ptr());

    let mut a = 10; // mutable object
    let a_mut_ref = &mut a; // mutable reference
    *a_mut_ref = 20; // dereference and assign
    println!("{}", a_mut_ref); // auto dereference
    println!("{}", a); // auto dereference

    println!("");
    println!("ライフタイム");
    let r: &mut i32;
    {
        let mut x: i32 = 1;
        r = &mut x;
        *r = 2;
        println!("{}", r);
    }
    // ここでrを使用しようとすると、xの生存期間が終わっているためエラーになる

    println!("");
    println!("スマートポインタ");
    let x: Box<i32> = Box::new(1);
    println!("{:p}", x);
    println!("{}", *x);

    // Rc: 所有権を2つ以上持たせる
    println!("");
    let a: Rc<String> = Rc::new("hello".to_string());
    println!("{}", Rc::strong_count(&a));
    {
        let b: Rc<String> = Rc::clone(&a);
        println!("{:p}", a);
        println!("{:p}", b);
        println!("{}", Rc::strong_count(&a));
    }
    println!("{}", Rc::strong_count(&a));
    println!("======ED sec_6======");
}

// fn concat(a: String, b: String) -> (String, String, String) {
//     let c: String = format!("{}, {}", a, b);
//     (c, a, b)
// }

// ↓参照を使った形式に変更
fn concat(a: &String, b: &String) -> String {
    let c: String = format!("{}, {}", a, b);
    c
}

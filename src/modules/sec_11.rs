pub fn sub() {
    println!("======ST sec_11======");
    println!("クロージャ");
    // 無名関数
    // let c1 = |x: i32| x + 1;
    // println!("{:?}", c1(10));

    let m = 10;
    let c2 = |x: i32| x + m; // ←mは自由変数 宣言されたときの状態を閉じ込める 環境をキャプチャする
    println!("{:?}", c2(10));

    let v: Vec<i32> = vec![1, 2, 3];
    let c3 = move || {
        println!("{:?}", v);
    };
    c3();
    // println!("{:?}", v);
    // moveを付けると参照ではなくなり所有権が移動する

    println!("======ED sec_11======");
}

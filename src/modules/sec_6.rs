fn concat(a: String, b: String) -> (String, String, String) {
    let c: String = format!("{}, {}", a, b);
    (c, a, b)
}

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
    let (s, s1, s2): (String, String, String) = concat(s1, s2); // 関数から所有権を返してもらう
    println!("{:?}", s);
    println!("{}", s1);
    println!("{}", s2);
    println!("======ED sec_6======");
}

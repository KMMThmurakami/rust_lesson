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

    let a = 100;
    {
        let v1 = vec![1, 2, 3];
        println!("{:03?}", v1);

        let mut v2 = v1;
        println!("{:^3?}", v2);

        v2.push(4);
    }
    println!("{}", a);
    println!("======ED sec_6======");
}

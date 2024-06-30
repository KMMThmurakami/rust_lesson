pub fn sub() {
    println!("======ST sec_12======");
    println!("コレクション");
    // ヒープに格納されるのでコンパイル時にサイズが決まっている必要がない
    let mut v1: Vec<&str> = vec!["Rust", "Python", "Java"];
    println!("{:?}", v1);
    println!("{:?}", v1.as_ptr());
    println!("{:?}", v1.len());
    println!("{:?}", v1.capacity());

    println!("{:?}", &v1[0]);
    println!("{:?}", v1.get(0));

    // ベクタの一番後ろに追加
    v1.push("PHP");
    println!("{:?}", v1);
    // ベクタの一番後ろを削除
    let val = v1.pop();
    println!("{:?}", val);
    println!("{:?}", v1);

    v1.insert(1, "PHP");
    println!("{:?}", v1);
    v1.remove(1);
    println!("{:?}", v1);

    println!("======ED sec_12======");
}

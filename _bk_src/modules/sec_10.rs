fn need_even(a: i32) -> Result<i32, String> {
    if a % 2 == 0 {
        Ok(a)
    } else {
        Err(String::from("引数は偶数にしてください"))
    }
}

fn double_even(b: i32) -> Result<i32, String> {
    let x: i32 = need_even(b)?; // Okだった場合、Okの中身が返される
    Ok(x * 2)
}

pub fn sub() {
    println!("======ST sec_10======");
    println!("エラー処理");
    println!("panic");
    // 起こってはいけない異常事態に使用される
    // println!("{}", [1, 2, 3][10]);
    // println!("{}", 1 / 0);
    // panic!("error");

    println!("Result型");
    // Rustには他の言語にあるような例外の機構がない
    // 失敗する可能性のある処理ではResult型を使う
    // println!("{:?}", need_even(2));
    // println!("{:?}", need_even(1));
    // let x = match need_even(1) {
    //     Ok(val) => val,
    //     Err(err) => {
    //         println!("{}", err);
    //         panic!()
    //     }
    // };
    // println!("{}", x);

    let s: Result<i32, String> = need_even(1);
    println!("{}", s.is_ok());
    println!("{}", s.is_err());

    // ok、errは所有権が移動するので注意
    // println!("{:?}", s.ok());
    // println!("{:?}", s.err());

    // println!("{:?}", s.unwrap_or(0));
    // println!("{:?}", s.unwrap());
    // println!("{:?}", s.expect("expectから発生"));

    println!("エラーの委譲");
    // println!("{:?}", double_even(1));
    match double_even(1) {
        Ok(val) => println!("{}", val),
        Err(err) => {
            println!("mainでハンドリング");
            println!("{}", err)
        }
    }

    println!("======ED sec_10======");
}

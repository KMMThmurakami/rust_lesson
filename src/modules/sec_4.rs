use core::ops::RangeInclusive;

fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    return a.to_string();
}

pub fn sub() {
    println!("======ST sec_4======");
    // 式
    // 何らかの評価を行うコード片
    // 評価した値を返し、変数に束縛することが可能
    // 式は文の部分要素

    // 文
    // 処理を実行するものの、値は返さない
    // 変数に束縛することはできない

    //             式----
    // let x: i32 = 1 + 2;
    // 文-----------------

    // Rustは式ベースの言語

    // 式にはセミコロンを書かない
    // →値を返すことができる

    println!("------ブロックとスコープ------");
    println!("a");
    {
        println!("b");
    }
    println!("c");

    // シャドーイング
    let y: i32 = 10;
    println!("{}", y);
    {
        let y: i32 = 5;
        println!("{}", y);
    }
    println!("{}", y);

    println!("------if------");
    let x: i32 = 30;
    if x > 0 && x <= 10 {
        // 条件部分は必ず論理型でなければならない
        println!("{}", "first");
    } else if x > 11 && x <= 20 {
        println!("{}", "second");
    } else {
        println!("{}", "else");
    }

    // ifは式なので値を返すことができる
    // 戻り値の型はそろえる
    #[rustfmt::skip]
    let x2: i32 = if x > 10 {
        x
    } else {
        0
    };
    println!("変数 x2 = {}", x2);

    println!("------match------");
    // switch文に似ている
    let x: i32 = 4;
    let x3 = match x {
        0 => 0,
        1 => x + 2,
        _ => x * 2,
    };
    println!("{:?}", x3);

    println!("------ループ------");
    // println!("------loop------");
    // let mut cnt = 0;
    // loop {
    //     if cnt >= 10 {
    //         break;
    //     }
    //     println!("{}", cnt);
    //     cnt += 1;
    // }

    // println!("------while------");
    // let mut cnt = 0;
    // while cnt < 10 {
    //     println!("{}", cnt);
    //     cnt += 1;
    // }

    println!("------for------");
    for i in [1, 2, 3] {
        println!("{}", i);
    }

    let r: RangeInclusive<i32> = 1..=10;
    for cnt10 in r {
        println!("{}", cnt10 * cnt10);
    }

    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for &element in a.iter() {
        println!("the value is: {}", element);
    }

    let v = vec![1, 2, 3];
    println!("type: {}", type_of(v.into_iter()));

    let v = vec![1, 2, 3];
    println!("type: {}", type_of(v.iter()));

    for x in vec![1, 2, 3].iter() {
        println!("type_of: {}", type_of(x));
    }

    println!("======ED sec_4======");
}

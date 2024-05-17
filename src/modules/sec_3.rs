const A: i32 = 100; // 定数は型推論されない

// 関数
fn say_hello() {
    println!("------関数------");
    // 返り値がない場合はunit型が返されている
}

fn add(a: i32, b: i32) -> i32 {
    // return a + b;
    a + b // 関数の最後に返したい値をセミコロンなしで書く方が一般的
          // returnは条件分岐で早期リターンするときに使う
}

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn sub() {
    println!("======ST sec_3======");
    println!("------println------");
    println!("Hello, world!");
    // print!("Hello, ");
    // print!("Rust!");

    // プレースホルダー{}
    println!("Hello, {}", "students");

    println!("------変数、定数------");
    let a: i32 = 1; // Rustでは変数に値を入れることを束縛・バインドという
    println!("変数 a = {}", a);

    let mut b: i32 = 2;
    println!("変数 b = {}", b);
    b = 3;

    println!("変数 b = {}", b);
    println!("定数 A = {}", A);

    // 数値型
    // i 符号付き整数
    // size:実行環境に依存する
    // u 符号なし整数
    // f 不動小数点数

    // 型推論
    println!("------数値型、論理型------");
    let a10 = 1; // デフォルトはi32
    let b10 = 2.0; // デフォルトはf64
    println!("変数 a10 = {}", a10);
    print_typename(a10);
    println!("変数 b10 = {}", b10);
    print_typename(b10);

    let c10: u16 = 3;
    println!("変数 c10 = {}", c10);
    print_typename(c10);

    let d10 = 4.0f32;
    println!("変数 d10 = {}", d10);
    print_typename(d10);

    // i32の1をf64にキャスト
    let f10: f64 = 1 as f64 + 2.0;
    println!("変数 f10 = {}", f10);
    print_typename(f10);

    // 論理型 true or false
    let g10: bool = false;
    println!("変数 g10 = {}", g10);
    print_typename(g10);

    println!("------タプル型------");
    let t1: (i32, bool, f64) = (1, true, 2.0);
    let t2: (f64, bool, i32) = (2.0, true, 1); // t1 != t2

    println!("変数 t1 = {:?}", t1);
    println!("変数 t2 = {:?}", t2);

    let i1 = t1.0;
    println!("変数 i1 = {}", i1);

    let (x, y, z) = t2;
    println!("変数 x = {}", x);
    println!("変数 y = {}", y);
    println!("変数 z = {}", z);

    // 参照外し
    let mut a = 10; // mutable object
    let a_mut_ref = &mut a; // mutable reference
    *a_mut_ref = 20; // dereference and assign
    println!("{}", a_mut_ref); // auto dereference

    println!("------配列------");
    // 全て同じ型
    let l1 = [1, 2, 3];
    println!("変数 l1 = {:?}", l1);
    let l2 = [0; 100];
    println!("変数 l2 = {:?}", l2);

    // スライス
    let l3 = &l1[0..2];
    println!("変数 l3 = {:?}", l3);
    let l4 = &l1[0..=2];
    println!("変数 l4 = {:?}", l4);
    let l5 = &l1[0..];
    println!("変数 l5 = {:?}", l5);

    println!("------ベクタ型------");
    // 配列に似ているが、初期化後に要素数を変更することができる
    let mut v1 = vec![1, 2, 3];
    v1.push(1);
    println!("変数 v1 = {:?}", v1);
    let mut v2: Vec<i32> = Vec::new();
    v2.push(111);
    println!("変数 v1 = {:?}", v2);
    let x = v2.pop();
    println!("変数 v1 = {:?}", v2);
    println!("変数 x = {:?}", x);
    let x = v2.pop();
    println!("変数 v1 = {:?}", v2);
    println!("変数 x = {:?}", x);
    // ベクタは奥が深い

    println!("------文字型------");
    let char1 = 'a';
    println!("変数 char1 = {}", char1);

    // 文字列スライス
    let str1 = "abc,";
    println!("変数 str1 = {}", str1);

    // 文字列型
    let mut str2 = String::from("aaa");
    // let str2 = "aaa".to_string();
    println!("変数 str2 = {}", str2);
    str2.push_str(",bbb");
    println!("変数 str2 = {}", str2);

    // formatは結合の順番を気にしなくてよい
    let str3: String = format!("{}{}", str1, str2);
    println!("変数 str3 = {}", str3);

    // 関数
    say_hello();
    let ret = add(1, 2);
    println!("変数 ret = {}", ret);

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

    println!("======ED sec_3======");
}

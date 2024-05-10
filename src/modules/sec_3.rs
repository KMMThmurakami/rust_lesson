const A: i32 = 100;  // 定数は型推論されない

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
    let a: i32 = 1;  // Rustでは変数に値を入れることを束縛・バインドという
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
    let a10 = 1;    // デフォルトはi32
    let b10 = 2.0;  // デフォルトはf64
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
    let f10 :f64 = 1 as f64 + 2.0;
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

    let (x,y,z) = t2;
    println!("変数 x = {}", x);
    println!("変数 y = {}", y);
    println!("変数 z = {}", z);

    // 参照外し
    let mut a = 10;            // mutable object
    let a_mut_ref = &mut a;    // mutable reference
    *a_mut_ref = 20;           // dereference and assign
    println!("{}", a_mut_ref); // auto dereference

    println!("======ED sec_3======");
}

const A: i32 = 100;  // 定数は型推論されない

pub fn sub() {
    println!("======ST sec_3======");
    println!("Hello, world!");
    // print!("Hello, ");
    // print!("Rust!");
    
    // プレースホルダー{}
    println!("Hello, {}", "students");
    
    let a: i32 = 1;  // Rustでは変数に値を入れることを束縛・バインドという
    println!("変数 a = {}", a);
    
    let mut b: i32 = 2;
    println!("変数 b = {}", b);
    b = 3;
    
    println!("変数 b = {}", b);
    println!("定数 A = {}", A);
    println!("======ED sec_3======");
}

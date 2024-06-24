use rand::Rng;

// 外部からも呼び出せる
pub fn test_fn1() {
    println!("sub1 test_fn1");
    test_fn2();
}
fn test_fn2() {
    println!("sub1 test_fn2");
}

pub struct TestStruct {
    pub val1: i32,
    pub val2: i32,
}

pub fn sub() {
    println!("======ST sec_8======");
    println!("クレート");
    let random_number = rand::thread_rng().gen_range(1..101);
    println!("{}", random_number);

    println!("バイナリとライブラリ");
    // 今まで書いてきたのはバイナリクレート
    // 単体で実行可能
    // cargoはデフォルトでsrc/main.rsをパッケージと同じ名前のバイナリクレートと認識する
    // main.rs = rust_lessonのバイナリクレートと認識されている
    // src/bin配下にファイルを配置することで複数のバイナリクレートを持つことができる

    // ライブラリクレートはパッケージで0個または1個もつことができる
    // ライブラリクレートは他のプログラムで取り込まれ機能提供するクレート
    // バイナリクレート間は機能を共有することができない
    rust_lesson::say_hello();

    println!("======ED sec_8======");
}

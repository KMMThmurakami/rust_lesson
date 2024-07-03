// バイナリクレートは必ずmain関数を含む必要がある
// 実行時は明示的に実行するバイナリクレートを指定する
// cargo r --bin bin1
fn main() {
    println!("this is bin1");
    rust_lesson::say_hello();
}

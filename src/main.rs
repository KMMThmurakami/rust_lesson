// mod modules;
// use modules::*;
use rust_lesson::services::validate::*;
use std::io;

fn main() {
    // 実行内容を取得
    let mut service_type: String = String::new();
    println!("実行したい内容を入力してください(0:登録、1:集計)");
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().expect("数値で入力してください");

    // 入力値のバリデーション
    InputValidator::validate_service_type(service_type);

    if service_type == 0 {
        println!("登録");
    } else if service_type == 1 {
        println!("集計");
    }
}

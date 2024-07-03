// use std::env;
use std::fs::{self};
// use std::fs::File;
// use std::io;
// use std::io::prelude::*;
// use std::io::BufReader;
// use std::fs::OpenOptions;
// use serde::{Deserialize, Serialize};
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};

// #[derive(Serialize, Deserialize, Debug)]
// struct Person {
//     name: String,
//     age: u8,
//     phones: Vec<String>,
// }

pub fn sub() {
    println!("======ST sec_14======");
    // println!("コマンドライン引数");
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // println!("文字列を入力してください");
    // let mut input: String = String::new();

    // io::stdin().read_line(&mut input).unwrap();
    // println!("{:?}", input);

    // let num: i32 = input.trim().parse().unwrap();
    // println!("{:?}", num);

    // let mut f = File::open("src/sample1.txt").unwrap();
    // let mut contents = String::new();

    // f.read_to_string(&mut contents).unwrap();
    // println!("{}", contents);

    // let contents = fs::read_to_string("src/sample1.txt").unwrap();
    // println!("{}", contents);

    // let mut buf_reader = BufReader::new(f);
    // let mut line = String::new();
    // buf_reader.read_line(&mut line).unwrap();
    // println!("{}", line);
    // buf_reader.read_line(&mut line).unwrap();
    // println!("{}", line);

    // let lines = buf_reader.lines();
    // for l in lines {
    //     println!("{}", l.unwrap());
    // }

    // let mut bytes = Vec::new();
    // f.read_to_end(&mut bytes).unwrap();
    // println!("{:?}", bytes);
    // println!("{:?}", String::from_utf8(bytes).unwrap());

    // let mut f1 = File::create("src/sample2.txt").unwrap();
    // let bytes = b"write example!\n"; // 文字列リテラルの前にbを付けるとバイト列に変換
    // println!("{:?}", bytes);
    // f1.write_all(bytes).unwrap();

    // let mut f2 = File::create("src/sample3.txt").unwrap();
    // writeln!(f2, "Hello, {}", "Rust").unwrap();

    // let mut f1 = OpenOptions::new().append(true).open("src/sample1.txt").unwrap(); // 指定したファイルが存在した場合は追記
    // writeln!(f1, "Hello, {}", "Rust").unwrap();

    // let mut f2 = OpenOptions::new().write(true).create_new(true).open("src/sample1.txt").unwrap(); // ファイルが存在しない場合のみ書き込み
    // writeln!(f2, "Hello, {}", "Rust").unwrap();

    // let p: Person = Person {
    //     name: String::from("Yamada Taro"),
    //     age: 30,
    //     phones: vec![String::from("080-xxxx-xxxx"), String::from("090-xxxx-xxxx")],
    // };

    // let json_data: String = serde_json::to_string_pretty(&p).unwrap();

    // let mut f = File::create("src/sample.json").unwrap();
    // writeln!(f, "{}", json_data).unwrap();

    // let f2 = File::open("src/sample.json").unwrap();
    // let buf_reader = BufReader::new(f2);
    // let data: Person = serde_json::from_reader(buf_reader).unwrap();
    // println!("{:?}", data);

    let path = Path::new("src");
    println!("{:?}", path.exists());
    println!("{:?}", path.is_dir());
    println!("{:?}", path.is_file());
    println!("{:?}", path.file_name().unwrap());

    let mut path_buf = PathBuf::from("src");
    path_buf.push("sample1.txt");
    println!("{:?}", path_buf);
    path_buf.set_file_name("path.txt");
    println!("{:?}", path_buf);
    path_buf.pop();
    println!("{:?}", path_buf);

    // fs::create_dir("src/test1").unwrap(); // 1階層
    // fs::create_dir_all("src/test2/test2-1/test2-1-1").unwrap(); // 複数の階層をまとめて作成

    // fs::remove_dir("src/test1").unwrap(); // 1階層
    // fs::remove_dir_all("src/test2").unwrap(); // 複数の階層をまとめて作成

    // fs::remove_file("src/sample1.txt").unwrap();
    // fs::copy("src/sample2.txt", "src/sample3.txt").unwrap();
    // fs::rename("src/sample3.txt", "src/test1/sample3.txt").unwrap();

    let mut permissions = fs::metadata("src/sample2.txt").unwrap().permissions();
    permissions.set_mode(0o600);
    fs::set_permissions("src/sample2.txt", permissions).unwrap();

    println!("======ED sec_14======");
}

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn sub() {
    println!("======ST sec_12======");
    println!("コレクション");
    // ヒープに格納されるのでコンパイル時にサイズが決まっている必要がない
    // let mut v1: Vec<&str> = vec!["Rust", "Python", "Java"];
    // println!("{:?}", v1);
    // println!("{:?}", v1.as_ptr());
    // println!("{:?}", v1.len());
    // println!("{:?}", v1.capacity());

    // println!("{:?}", &v1[0]);
    // println!("{:?}", v1.get(0));

    // // ベクタの一番後ろに追加
    // v1.push("PHP");
    // println!("{:?}", v1);
    // // ベクタの一番後ろを削除
    // let val = v1.pop();
    // println!("{:?}", val);
    // println!("{:?}", v1);

    // v1.insert(1, "PHP");
    // println!("{:?}", v1);
    // v1.remove(1);
    // println!("{:?}", v1);

    let v1: Vec<&str> = vec!["Rust", "Python", "Java"];
    let v2: Vec<&str> = vec!["PHP", "Go"];
    let v3 = [v1, v2].concat();
    println!("{:?}", v3);

    let (v4, v5) = v3.split_at(2);
    println!("{:?}", v4);
    println!("{:?}", v5);

    let mut v6 = vec![3, 6, 1, 7, 2];
    v6.sort();
    println!("{:?}", v6);
    v6.reverse();
    println!("{:?}", v6);

    #[derive(Debug)]
    struct S {
        val1: i32,
        val2: i32,
    }

    let mut v7 = vec![S { val1: 3, val2: 1 }, S { val1: 2, val2: 2 }, S { val1: 1, val2: 3 }];
    v7.sort_by_key(|s| s.val1);
    println!("{:?}", v7);
    println!("{:?}", v7[1].val2);

    let v8 = vec![3, 6, 1, 7, 2];
    println!("{:?}", v8.contains(&4));

    let x = v8.iter().position(|x| *x == 4);
    println!("{:?}", x);

    println!("\nキュー");
    let mut q = VecDeque::new();
    q.push_back(1);
    q.push_back(2);
    q.push_back(3);
    println!("{:?}", q);
    println!("{:?}", q.pop_front());
    println!("{:?}", q);

    // バイナリヒープ
    // 最大値が常に先頭に来る
    let mut bh = BinaryHeap::new();
    bh.push(1);
    bh.push(10);
    bh.push(20);
    bh.push(2);
    println!("{:?}", bh);
    println!("{:?}", bh.pop());
    println!("{:?}", bh);

    println!("\nマップ");
    let mut map = HashMap::new();
    map.insert("Japan", 11);
    map.insert("USA", 3);
    map.insert("China", 1);
    map.insert("India", 2);
    println!("{:?}", map);

    map.insert("Japan", 10);
    println!("{:?}", map);
    println!("{:?}", map.get("Japan"));
    println!("{:?}", map.remove("India"));
    println!("{:?}", map);

    for (k, v) in &map {
        println!("{:?}", k);
        println!("{:?}", v);
    }

    println!("\nセット");
    // ハッシュセット 値の順番の保証はない
    let mut set1 = HashSet::new();
    set1.insert(1);
    set1.insert(1);
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    set1.insert(4);
    println!("{:?}", set1);

    println!("{:?}", set1.contains(&4));
    println!("{:?}", set1.remove(&2));

    let mut set2 = HashSet::new();
    set2.insert(1);
    set2.insert(2);
    set2.insert(3);
    set2.insert(5);

    // 和集合
    let set3 = &set1 | &set2;
    println!("{:?}", set3);
    // 積集合
    let set4 = &set1 & &set2;
    println!("{:?}", set4);
    // 差集合
    let set5 = &set1 - &set2;
    println!("{:?}", set5);
    // 排他的論理和
    let set6 = &set1 ^ &set2;
    println!("{:?}", set6);

    println!("======ED sec_12======");
}

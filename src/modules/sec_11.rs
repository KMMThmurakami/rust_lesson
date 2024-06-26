struct Counter {
    start: u32,
    end: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.start > self.end {
            None
        } else {
            let result = Some(self.start);
            self.start += 1;
            result
        }
    }
}

pub fn sub() {
    println!("======ST sec_11======");
    println!("クロージャ");
    // 無名関数
    // let c1 = |x: i32| x + 1;
    // println!("{:?}", c1(10));

    let m = 10;
    let c2 = |x: i32| x + m; // ←mは自由変数 宣言されたときの状態を閉じ込める 環境をキャプチャする
    println!("{:?}", c2(10));

    let v: Vec<i32> = vec![1, 2, 3];
    let c3 = move || {
        println!("{:?}", v);
    };
    c3();
    // println!("{:?}", v);
    // moveを付けると参照ではなくなり所有権が移動する

    println!("イテレータ");
    let v = vec![1, 2, 3, 4, 5];
    let v1_iter = v.iter();
    println!("{:?}", v);
    println!("{:?}", v1_iter);
    for x in v1_iter {
        println!("{:?}", x);
    }

    let mut v2_iter = v.iter();
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());

    let mut v = vec![1, 2, 3, 4, 5];
    let mut v2_iter = v.iter_mut();
    println!("{:?}", v2_iter.next());

    let mut c = Counter { start: 1, end: 5 };
    // for i in c {
    //     println!("{:?}", i);
    // }
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());

    println!("======ED sec_11======");
}

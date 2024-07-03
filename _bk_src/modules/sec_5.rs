fn fizz_buzz1(end: i32) {
    let mut i: i32 = 1;
    while i <= end {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
        i += 1;
    }
}

fn fizz_buzz2(end: i32) {
    let r: std::ops::RangeInclusive<i32> = 1..=end;
    for i in r {
        match i % 15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}", i),
        };
    }
}

fn fizz_buzz3(end: i32) {
    let r: std::ops::RangeInclusive<i32> = 1..=end;
    for i in r {
        match (i % 3, i % 5) {
            (0, 0) => println!("{}:FizzBuzz", i),
            (0, _) => println!("{}:Fizz", i),
            (_, 0) => println!("{}:Buzz", i),
            _ => println!("{}", i),
        };
    }
}

pub fn sub() {
    println!("======ST sec_5======");
    println!("======fizz_buzz1======");
    fizz_buzz1(30);
    println!("======fizz_buzz2======");
    fizz_buzz2(30);
    println!("======fizz_buzz3======");
    fizz_buzz3(30);
    println!("======ED sec_5======");
}

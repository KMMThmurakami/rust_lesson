fn maybe_panic(flag: bool) {
    if flag == false {
        print!("safe!");
        panic!("unexpected!!!");
    } else {
        panic!("flag is true!!!");
    }
}

#[cfg(test)] // build時はコンパイルされない
mod test_module {
    #[test]
    #[should_panic(expected = "flag is true")]
    fn test_maybe_panic() {
        super::maybe_panic(true);
    }

    #[test]
    fn test_sample() {
        let a = 1 + 1;
        let b = 2;
        assert_eq!(a, b);
    }

    #[test]
    #[ignore]
    fn test_sample2() {
        let a = 1 + 1;
        let b = 2;
        assert_eq!(a, b);
    }
}

pub fn sub() {
    println!("======ST sec_13======");
    println!("テスト");
    // cargo t --bin rust_lesson
    // cargo t --bin rust_lesson test_maybe_panic
    maybe_panic(true);

    println!("======ED sec_13======");
}

use rand::Rng;

pub fn sub() {
    println!("======ST sec_8======");
    println!("クレート");
    let random_number = rand::thread_rng().gen_range(1..101);
    println!("{}", random_number);

    println!("======ED sec_8======");
}

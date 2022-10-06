fn main() {
    let a = 1;
    let b = 2;
    println!("Hello, world!");
    println!("{}", add(&a, &b));
    println!("{}", a)
}

fn add(a:&i32, b:&i32) -> i32 {
    return *a + *b;
}



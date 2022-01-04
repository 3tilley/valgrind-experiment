pub fn foo() -> u32 {
    let mut counter: u32 = 0;
    for i in 0..10000 {
        counter += i;
    }
    counter
}

pub fn bar<T: std::ops::AddAssign + Copy>(a: T) -> T {
    let mut counter: T = a;
    for _ in 0..10000 {
        counter += a
    }
    counter
}

fn main() {
    let answer = bar(3);

    println!("{}", answer);
}

fn main() {
    for i in 1..=100 {
        let s = if i % 15 == 0 {
            "FizzBuzz"
        } else if i % 5 == 0 {
            "Buzz"
        } else if i % 3 == 0 {
            "Fizz"
        } else {
            println!("{}", i);
            continue;
        };
        println!("{}", s);
    }
}

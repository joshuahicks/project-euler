fn main() {
    let mut sum = 0;
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    println!("The sum of all multiples of 3 or 5 is: {}", sum);
}

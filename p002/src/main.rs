use std::collections::HashMap;

fn main() {
    let mut hist: HashMap<i32, i32> = HashMap::new();

    let mut index = 0;
    let mut sum = 0;
    loop {
        let val = fib(index, &mut hist);

        if val > 4000000 {
            break;
        }

        if val % 2 == 0 {
            sum += val;
        }
        index += 1;
    }

    println!("The sum of the even Fibonacci numbers whose value does not exceed 4 million is : {}", sum);
}

fn fib(n: i32, hist: &mut HashMap<i32, i32>) -> i32 {
    if hist.contains_key(&n) {
        return hist[&n]
    }

    if n < 2 {
        return n
    }

    let n_fib = fib(n-1, hist) + fib(n-2, hist);
    hist.insert(n, n_fib);
    n_fib
}

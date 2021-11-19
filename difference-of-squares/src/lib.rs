pub fn square_of_sum(n: u32) -> u32 {
    println!("square of sum of 1...{}", n);
    let sum = (1..=n).sum::<u32>();
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    println!("sum of squares of 1...{}", n);
    (1..=n).map(|x| x * x).sum()
}

pub fn difference(n: u32) -> u32 {
    println!(
        "difference between square of sum of 1...{n} and sum of squares of 1...{n}",
        n = n,
    );
    square_of_sum(n) - sum_of_squares(n)
}

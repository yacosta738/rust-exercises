pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let sum: u32 = digits.iter().map(|d| d.pow(digits.len() as u32)).sum();
    sum == num
}

/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    Vec::new()
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    //calculate the fibonacci sequence
    let mut fibonacci_sequence = create_empty();
    let n = 5;
    fibonacci_sequence.push(1);
    fibonacci_sequence.push(1);
    for i in 2..n {
        fibonacci_sequence.push(fibonacci_sequence[i - 1] + fibonacci_sequence[i - 2]);
    }
    fibonacci_sequence
}

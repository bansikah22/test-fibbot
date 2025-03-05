fn main() {
    println!("This is the test project for FibBot action!");
    // Any additional test logic can be added here
    println!("Fibonacci sequence up to 10:");
    for i in fibonacci_sequence(10) {
        println!("{}", i);
    }
    
    // Test the FibBot action
    fn fibonacci_sequence(n: usize) -> Vec<u32> {
        let mut sequence = vec![0, 1];
        for i in 2..n {
            let next = sequence[i - 1] + sequence[i - 2];
            sequence.push(next);
        }
        sequence
    }
}


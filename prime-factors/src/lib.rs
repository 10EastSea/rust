pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut results = Vec::new();
    
    let mut i = 2;
    while num > 1 {
        while num % i == 0 {
            results.push(i);
            num /= i
        }
        i += 1
    }

    return results;
}

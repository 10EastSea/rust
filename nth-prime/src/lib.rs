fn is_prime(n: u32) -> bool {
    let mut i = 2;
    while i <= (((n as f64).sqrt() as u32) + 1) { // 에라토스테네스체에 의해 n의 root 값 이하만 확인하면 됨
        if (n % i) == 0 { return false; }
        i += 1; // 1씩 증가해가며 확인
    }
    return true;
}

pub fn nth(n: u32) -> u32 {
    if n == 0 { return 2; }
    else {
        let mut count = 0;
        let mut candidate = 2;

        while count != n { // 현재 번째가 n번째가 아닌 경우 계속 반복
            candidate += 1; // 숫자를 1씩 증가해가며 소수인지 확인
            if is_prime(candidate) { count += 1; } // 소수라면 count 개수 1 증가
        }

        return candidate;
    }
}

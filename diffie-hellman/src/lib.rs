use rand::prelude::*;

fn square_and_multiply(x: u64, a: u64, n: u64) -> u64 {
    let mut result = 1;

    // 곱셈 연산을 하는 도중 overflow를 막기 위해 타입을 전부 2배씩 크게 캐스팅 함
    let mut exponent = a as u128;   // 지수
    let modular = n as u128;        // 모듈러
    let mut base = (x % n) as u128; // 밑

    while exponent > 0 {
        // 맨 아래 bit가 1인지 확인 -> 맞으면 result에 결과 값 곱함
        if (exponent & 1) == 1 { result = (result * base) % modular; }
        
        exponent >>= 1; // 오른쪽으로 1 bit shift
        base = (base * base) % modular; // base를 제곱으로 증가
    }

    return result as u64;
}

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    square_and_multiply(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    square_and_multiply(b_pub, a, p)
}

fn collatz_sequence(mut n: i32) -> u32 {
    let mut steps = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1
        }
        steps += 1; // steps++ 과 같은 후위 증가 연산자 사용 불가
    }

    steps
}

/**
 * Rust FP Style - 모범 답안 리서치
 * 
 * Some(value)은 Option<T> 타입의 값이 존재하는 경우를 나타내는 열거형(variant)
 * 반대로 값이 없을 때는 None을 사용
 * 
 * Some() <-> None
 */
fn collatz_sequence_fp(n: i32) -> usize {
    use std::iter::successors;

    successors(Some(n), |&x| {
        if x == 1 {
            None
        } else if x % 2 == 0 {
            Some (x / 2)
        } else {
            Some (3 * x + 1)
        }
    }).count()
    -1 // 시작 값 n을 포함하기에 -1 해야 한다.
}


fn main() {
    let n = 100;
    println!("Collatz sequence steps for {}: {}", n, collatz_sequence(n));
    println!("Collatz sequence steps for {}: {}", n, collatz_sequence_fp(n));
}
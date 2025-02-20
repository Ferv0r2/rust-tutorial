/**
 * !를 사용하면 매크로. println!도 매크로이다
 * 
 * #[var] 형태로 macro를 export할 수 있다 `mod var` 형태로 import 가능
 * 
 * `expr`는 어떻게 타입을 알까?
 * - Rust의 매크로(macro_rules!)에서 expr을 사용할 때 타입이 정해지는 것이 아니라,
 * - 컴파일러가 매크로가 확장된 후에 타입을 검사한다
 */

#[macro_export] // 이와 같은 형태로 export가 가능
macro_rules! fizzbuzz {
    ($n: expr) => {
        for x in 1..=$n {
            if x % 3 == 0 && x % 5 == 0 {
                println!("FizzBuzz: {}", x);
            } else if x % 3 == 0 {
                println!("Fizz: {}", x);
            } else if x % 5 == 0 {
                println!("Buzz: {}", x);
            }
        }
    }
}
fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn main() {
    let n = 4;
    println!("{n}! = {}", factorial(n));
    fizzbuzz!(100);
}
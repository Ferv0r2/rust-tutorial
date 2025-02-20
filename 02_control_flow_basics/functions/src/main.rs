// TypeScript와 동일하게 {params}: {type} 형태로 작성, '=>' 대신 '->', return 생략 시 추론
// 주의! Overloading이 지원되지 않음. 단, 매크로를 사용하면 가능
fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn main() {
    println!("gcd: {}", gcd(143, 52));
}
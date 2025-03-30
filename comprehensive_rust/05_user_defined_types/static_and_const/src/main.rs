/**
 * 상수(const)는 그 상수가 사용되는 곳에 인라인 된다.
 * const 값을 생성할 때에는 const로 마킹된 함수만이 호출 가능하며, 이 함수들은 컴파일 시에 호출된다. 물론 const함수들을 런타임에 호출하는 것도 가능하다.
 */
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

/**
 * 정적 변수(static)는 프로그램이 수행되는 동안 유지가 된다. 그러므로 다른 변수로 이동(move)되지 않는다.
 */
static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");
    println!("{BANNER}");
}
/**
 * assert! -> 특정 조건이 true인지 확인
 * assert_eq! -> 두 값이 같은지 확인
 * assert_ne! -> 두 값이 다른지 확인
 * 
 * 각 매크로 앞에 {debug_} 추가 시 디버그 모드에서만 감지 (prod 환경에선 무시)
 * e.g) debug_assert!
 */
fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert!(prime % i == 0);
            assert_eq!(prime % i, 0);
            assert_ne!(prime % i, 0);

        }
    }
}
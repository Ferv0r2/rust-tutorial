/**
 * 참조(&T): 값의 메모리 위치를 가리키며, 소유권을 넘기지 않고도 값을 접근할 수 있게 한다.
 * -> C의 포인터 개념
 * 
 * & 연산자 -> 참조
 * * 연산자 -> 역참조
 * 
 * ! Rust 컴파일러가 허상 참조(Dangling Reference) 방지한다.
 * -> 이미 소멸된 값(메모리 해제 후)의 참조
 */

/**
 * 허상 참조 예제 -> Borrow Checker에서 에러 발생
 * 
 * Compile Error: missing lifetime specifier
 * - this fundction's return type contains a borrowed value, but there is no value for it to be borrowed from
 * - cannot return reference to local variable `point` - returns a reference to data owned by the current function
 * 
 * fn x_axis(x: i32) -> &(i32, i32) {
 *   let point = (x, 0);
 *   return &point;
 * }
 */

fn main() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b; // C++과의 차이점, 참조된 값이 변경되는 것이 아닌 새로 바인딩
    println!("r: {}", *r);
}
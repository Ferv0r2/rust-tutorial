/**
 * Tuple과 같이 구조화된 값을 다룰 때, 내부 값을 변수로 추출할 수 있다.
 * (JavaScript 구조 분해와 동일)
 */

fn print_tuple_directing(tuple: (i32, i32)) {
    let left = tuple.0;
    let right = tuple.1;
    println!("left: {left}, right: {right}");
}

fn print_tuple_destructuring(tuple: (i32, i32)) {
    let (left, right) = tuple;
    println!("left: {left}, right: {right}");
}

// tuple 뿐만 아니라 구조체나 열거형도 가능하다.
fn print_struct() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 1, y: 2 };
    let Point { x, y } = p;

    println!("x: {x}, y: {y}");
}

// 반증 불가 패턴(irrefutable_pattern) <-> 반증 가능 패턴(refutable_pattern)

// 항상 참 -> 항상 매칭되므로 컴파일 가능
fn irrefutable_pattern() {
    let (a, b) = (1, 2);
    println!("a: {a}, b: {b}");
}

// 조건부 참 -> 매칭될 때, 컴파일 가능
fn irrefutable_pattern_condition() {
    if let Some(value) = Some(10) {
        println!("Value: {value}");
    }
}

// 반증 불가 -> 컴파일 불가능
// None이 들어오면 Some(value)로 해체할 수 없다.
// 반증 가능한 패턴은 if let 또는 match로 처리해야 한다.
fn refutable_pattern() {
    // let Some(value) = Some(10); // error
}

fn main() {
    print_tuple_directing((1, 2));
    print_tuple_destructuring((1, 2));
    print_struct();
    irrefutable_pattern();
    irrefutable_pattern_condition();
    refutable_pattern();
}
// 고정 길이를 가진다
fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    // a[10] = 0; / -> Overflow시 런타임 중 에러 발생
    println!("a: {a:?}");
    // println!("a: {a:#?}"); // -> 배열이 줄바꿈되어 출력
}
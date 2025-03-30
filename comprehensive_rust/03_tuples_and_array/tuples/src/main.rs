// 고정 길이를 가진다
// Rust에서 ()는 unit type이라 불리며, 다른 언어의 void와 같다

fn unit_type() {}

fn main() {
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
    println!("void type: {:?}", unit_type()); // result: ()
}
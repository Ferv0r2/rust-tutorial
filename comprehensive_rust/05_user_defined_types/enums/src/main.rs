
use std::mem::transmute;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

// 변형으로 표현되는 열거형 패턴
#[derive(Debug)]
enum PlayerMove {
    Pass,                        // 단순 변형
    Run(Direction),              // 튜플 변형
    Teleport { x: u32, y: u32 }, // 구조체 변형
}

fn enum_example() {
    let m: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("이번 차례: {:?}", m);
}

/**
 * 식별자 값을 직접 지정할 수 있다. (as 키워드)
 * repr 속성이 없는 경우, 10002는 2 bytes로 표현 가능하기에 식별자의 타입 크기는 2 bytes가 됨
 */
#[repr(u32)]
enum Bar {
    A, // 0
    B = 10000,
    C, // 10001
}

fn rper_example() {
    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);
}

macro_rules! dbg_bits {
    ($e:expr, $bit_type:ty) => {
        println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
    };
}

/**
 * 널포인터 최적화: Some::<T>(_)와 같은 타입들에 대해서 러스트는 size_of::<T>()가 size_of::<Option<T>>()와 같은 것을 보장한다.
 * 단, 코드에서 설정한 비트 패턴은 컴파일러가 보장해주지 않는다. -> unsafe
 */
fn null_pointer_optimization() {
    unsafe {
        println!("bool:");
        dbg_bits!(false, u8);
        dbg_bits!(true, u8);

        println!("Option<bool>:");
        dbg_bits!(None::<bool>, u8);
        dbg_bits!(Some(false), u8);
        dbg_bits!(Some(true), u8);

        println!("Option<Option<bool>>:");
        dbg_bits!(Some(Some(false)), u8);
        dbg_bits!(Some(Some(true)), u8);
        dbg_bits!(Some(None::<bool>), u8);
        dbg_bits!(None::<Option<bool>>, u8);

        println!("Option<&i32>:");
        dbg_bits!(None::<&i32>, usize);
        dbg_bits!(Some(&0i32), usize);
    }
}

fn main() {
    enum_example();
    rper_example();
    null_pointer_optimization();
}
# Learn Rust

- [Comprehensive Rust](https://google.github.io/comprehensive-rust/ko/index.html)
- [이지 러스트](https://m.yes24.com/Goods/Detail/142661976)

## Basic

- 이지 러스트에서 러스트 언어 고유의 특성이 느껴질 때 기록

### 1.8 Display 및 Debug - p.40

- Display
  - 간단한 변수 출력
  - `println!`
- Debug
  - 디버깅 출력
  - `{:?}`: 일반적인 출력
  - `{:?#}`: pretty-print

### 1.9 가장 작은 숫자와 큰 숫자

- 타입 이름 뒤에 `MIN`과 `MAX`를 사용

```rust
fn main() {
  println!("Min: {}, Max: {}", u8::MIN, u8::MAX); // 0, 255
  println!("Min: {}, Max: {}", i8::MIN, i8::MAX); // -128, 127
  println!("Min: {}, Max: {}", u16::MIN, u16::MAX); // 0, 65535
  println!("Min: {}, Max: {}", i16::MIN, i16::MAX); // -32768, 32767
  println!("Min: {}, Max: {}", u32::MIN, u32::MAX); // 0, 4294967295
  println!("Min: {}, Max: {}", i32::MIN, i32::MAX); // -2147483648, 2147483647
  println!("Min: {}, Max: {}", u64::MIN, u64::MAX); // 0, 18446744073709551615
  println!("Min: {}, Max: {}", i64::MIN, i64::MAX); // -9223372036854775808, 9223372036854775807
}
```

### 1.11 섀도잉

이전에 선언한 변수와 동일한 이름으로 새 변수 선언 시 처리

```rust
fn main() {
    // i32 타입
  let x = 8;
  let x = x + 1;
  {
    // f64 타입 앞의 x와 완전히 다른 변수
    let x = 9.2;
    println!("x: {}", x); // 9.2
  }
  println!("x: {}", x); // 9 (9.2가 아님)
}
```
- 변수를 숨겨도 변수가 소멸되지 않고 차단됨

```rust
fn times_two(x: i32) -> i32 {
  x * 2
}

// 섀도잉이 있다면
fn main() {
    let final_number = {
        let y = 10;
        let x = 9;
        let x = times_two(x);
        let x = x + y;
        x
    };
  println!("final_number: {}", final_number);
}

// 섀도잉이 없다면
fn main() {
  let final_number = {
    let y = 10;
    let x = 9;
    let x_times_two = times_two(x); // x의 두 번째 이름
    let x_twice_and_y = x_times_two + y; // x의 세 번째 이름
    x_twice_and_y // 섀도잉을 사용했다면 그냥 x
  };
  println!("final_number: {}", final_number);
}
```

### 2.2 출력 알아보기

- `r#`을 통해 문자열 내의 특수문자를 그대로 표현할 수 있음
- 만약 문자열에 `#`이 있다면, `#`개수를 늘려 문자열 내의 `#`을 그대로 표현할 수 있음
```rust
fn main() {
    println!("He said, \"You can find c:\\files\\documents\\file.txt.\".");
    println!(r#"He said, "You can find c:\files\documents\file.txt."."#);
    println!(r##"#He said, "You can find c:\files\documents\file.txt."."##);
}
```
- 또한 `r#`을 활용하여 키워드를 변수로 사용할 수 있음

```rust
fn main() {
    let r#let = 6; // 변수 이름은 let
    let mut r#mut = 10; // 변수 이름은 mut
}
```

- `b`를 통해 문자열을 바이트 배열로 변환할 수 있음

```rust
fn main() {
    let bytes = b"Hello, world!";
    println!("{:?}", bytes); // [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
}
```

- 그 외 포맷의 예

```rust
fn main() {
    let title = "Title";
    
    // 변수 이름 없음, -로 채움, 30자 길이로 중앙에 배치
    println!("{:-^30}", title); // "------------Title------------"
    let bar = "|";

    // 변수 이름 없음, 왼쪽과 오른쪽에 공백으로 채움
    println!("{: <15}{: >15}", bar, bar); // "|                              |"
    let a = "SEOUL"
    let b = "TOKYO"

    // 변수 이름 city1 및 city2, 각각 왼쪽과 오른쪽에 -로 채움
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);  // SEOUL--------------------TOKYO

    // result
    // ------------Title------------
    // |                            |
    // SEOUL--------------------TOKYO
}
```
// 해당 좌표의 제곱을 더하고 제곱근을 사용하여 벡터의 크기를 계산한다.
// 추후 배울 iter 메서드를 활용하면 좀 더 축약하여 가능하다.

fn magnitude(vector: &[f64]) -> f64 {
    let mut mag_squared = 0.0;
    for coord in vector {
        mag_squared += coord * coord;
    }

    mag_squared.sqrt()
}

// 참조를 통해 원본 값을 별도 변수에 담지 않고 수정 가능하다.
fn normalize(vector: &mut [f64]) {
    let mag = magnitude(vector);
    
    if mag != 0.0 {
        for item in vector {
            *item /= mag;
        }
    }
}

fn main() {
    println!("단위 벡터의 크기: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("{v:?} 크기: {}", magnitude(&v));
    normalize(&mut v);
    println!("정규화 후 {v:?}의 크기: {}", magnitude(&v));
}
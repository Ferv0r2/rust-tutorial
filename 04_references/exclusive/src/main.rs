
/**
 * 배타적 참조
 * 
 * 데이터에 대한 유일한 수정 권한을 확보하여, 데이터 경쟁 없이 안전하게 값을 변경할 수 있는 방법
 * 이는 앞서 확인한 Borrow Checker가 컴파일 단계에서 여러 참조가 동시에 존재하는 것을 막아 메모리 안전성을 보장
 */

fn main() {
    let mut point = (1, 2);
    let x_coord = &mut point.0; // 배타적 가변 참조 생성
    *x_coord = 20; // 배타점 참조를  통해 수정
    println!("point: {point:?}");
}

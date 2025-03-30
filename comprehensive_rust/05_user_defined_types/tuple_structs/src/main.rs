/**
 * 모호한 타입 자동 변환으로 인한 사례(화성 기후 궤도선)를 통해 NewType을 소개
 */
struct Point(i32, i32);
struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    todo!("NASA 로켓 과학자에게 물어보세요")
}

fn set_thruster_force(force: Newtons) {
    // ...
    return;
}

fn main() {
    // 튜플 구조체 사용 가능
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);

    // 종종 단일 필드의 래퍼(wrapper) 혹은 뉴타입(newtype)으로 부름
    let force = compute_thruster_force();
    // set_thruster_force(force); // mismatched types expected `Newtons`, found `PoundOfForce`
}
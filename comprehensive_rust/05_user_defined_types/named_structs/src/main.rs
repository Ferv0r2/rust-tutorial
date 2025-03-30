/**
 * 특징
 * - typedef 없이 바로 구조체 선언 가능하다.
 * - 상속이 없다.
 * 
 */
struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{}은(는) {}세입니다.", person.name, person.age);
}

fn main() {
    let mut peter = Person { name: String::from("피터"), age: 27 };
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("에이버리");
    let age = 39;
    let avery = Person { name, age }; // 필드명과 변수명이 같으면 축약 가능
    describe(&avery);

    let jackie = Person { name: String::from("재키"), ..avery }; // 스프레드 문법 지원 (구조 분해와 동일)
    describe(&jackie);
}
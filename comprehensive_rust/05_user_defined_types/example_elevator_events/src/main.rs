#[derive(Debug)]
enum Event {
    // 버튼을 눌렀을 때 이벤트
    ButtonPressed(Button),

    // 엘레베이터가 도착했을 때 이벤트
    CarArrived(i32),

    // 엘레베이터 문이 열릴 때 이벤트
    CarDoorOpened,

    // 엘레베이터 문이 닫힐 때 이벤트
    CarDoorClosed,
}


/// 이동 방향
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

type Floor = i32;

// 버튼 이벤트
#[derive(Debug)]
enum Button {
    // 특정 층에서 로비에 있는 버튼
    LobbyCall(Direction, Floor),

    // 엘레베이터 내부의 버튼
    CarFloor(Floor)
}

/// 엘리베이터가 지정된 층에 도착했을 때
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

/// 엘리베이터 문이 열릴 때
fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

/// 엘리베이터 문이 닫힐 때
fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

/// 지정된 층의 엘리베이터 로비에서 방향 버튼을 누를 때
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

/// 엘리베이터에서 층 버튼을 누를 때
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}

fn main() {
    println!(
        "1층 승객이 위쪽 버튼을 눌렀습니다. {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("엘리베이터가 1층에 도착했습니다: {:?}", car_arrived(0));
    println!("엘리베이터 문이 열렸습니다. {:?}", car_door_opened());
    println!(
        "승객이 3층 버튼을 눌렀습니다. {:?}",
        car_floor_button_pressed(3)
    );
    println!("엘리베이터 문이 닫혔습니다: {:?}", car_door_closed());
    println!("엘리베이터가 3층에 도착했습니다. {:?}", car_arrived(3));
}

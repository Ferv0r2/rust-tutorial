enum CarryableConcreteItem {
    Left,
    Right,
}

type Item = CarryableConcreteItem;

// 별칭은 다음과 같이 길고 복잡한 타입에서 더 유용하다.
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;

// 사용 예제가 궁금하여 찾아보았는데, 처음 보는 메서드가 많다.
fn print_inventory(inventory: &PlayerInventory, label: &str) {
    let inv = inventory.read().unwrap();
    println!("{label}:");
    for (i, item) in inv.iter().enumerate() {
        let name = match *item.borrow() {
            Item::Left => "Left",
            Item::Right => "Right",
        };
        println!("  Slot {}: {}", i, name);
    }
}

fn main() {
    // 인벤토리 생성
    let inventory: PlayerInventory = RwLock::new(Vec::new());

    // 쓰기 락을 한 번만 걸어 두 개의 아이템을 추가
    {
        let mut inv = inventory.write().unwrap();
        inv.push(Arc::new(RefCell::new(Item::Left)));
        inv.push(Arc::new(RefCell::new(Item::Right)));
    }

    // 현재 인벤토리 상태 출력
    print_inventory(&inventory, "초기 상태");

    // 첫 번째 슬롯의 아이템을 Right로 변경
    {
        let inv = inventory.read().unwrap();
        *inv[0].borrow_mut() = Item::Right;
    }

    // 수정 후 상태 출력
    print_inventory(&inventory, "수정 후");
}




mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm_zici!");
}

// NSWE

#[wasm_bindgen]
pub fn get_direction(x: i32, y: i32) -> String {
    match (x.cmp(&0), y.cmp(&0)) {
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => "东".to_string(),
        (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => "西".to_string(),
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => "北".to_string(),
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => "南".to_string(),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => "东北".to_string(),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => "东南".to_string(),
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => "西北".to_string(),
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => "西南".to_string(),
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => "原点".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_direction() {
        // 测试东 (x>0, y=0)
        assert_eq!(get_direction(1, 0), "东");
        assert_eq!(get_direction(100, 0), "东");

        // 测试西 (x<0, y=0)
        assert_eq!(get_direction(-1, 0), "西");
        assert_eq!(get_direction(-100, 0), "西");

        // 测试北 (x=0, y<0)
        assert_eq!(get_direction(0, -1), "北");
        assert_eq!(get_direction(0, -100), "北");

        // 测试南 (x=0, y>0)
        assert_eq!(get_direction(0, 1), "南");
        assert_eq!(get_direction(0, 100), "南");

        // 测试东北 (x>0, y<0)
        assert_eq!(get_direction(1, -1), "东北");
        assert_eq!(get_direction(100, -100), "东北");

        // 测试东南 (x>0, y>0)
        assert_eq!(get_direction(1, 1), "东南");
        assert_eq!(get_direction(100, 100), "东南");

        // 测试西北 (x<0, y<0)
        assert_eq!(get_direction(-1, -1), "西北");
        assert_eq!(get_direction(-100, -100), "西北");

        // 测试西南 (x<0, y>0)
        assert_eq!(get_direction(-1, 1), "西南");
        assert_eq!(get_direction(-100, 100), "西南");

        // 测试原点 (x=0, y=0)
        assert_eq!(get_direction(0, 0), "原点");
    }
}

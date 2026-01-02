//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_zici::get_direction;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
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

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

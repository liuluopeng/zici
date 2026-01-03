use wasm_zici::{get_all_new_chars_from_txt, get_new_chars};
use wasm_bindgen::JsValue;
use js_sys::Array;

fn main() {
    // 测试加载函数
    println!("Testing get_all_new_chars_from_txt...");
    match get_all_new_chars_from_txt() {
        Ok(_) => println!("✓ Successfully loaded new_chars.txt"),
        Err(e) => println!("✗ Failed to load new_chars.txt: {:?}", e),
    }
    
    // 测试获取函数
    println!("\nTesting get_new_chars...");
    
    // 测试有效输入
    let test_cases = vec![
        (1, 1, "一年级上册"),
        (1, 2, "一年级下册"),
        (6, 1, "六年级上册"),
        (6, 2, "六年级下册"),
    ];
    
    for (grade, term, description) in test_cases {
        let result = get_new_chars(grade, term);
        let array = Array::from(&result);
        println!("✓ {}: {} characters", description, array.length());
    }
    
    // 测试无效输入
    let invalid_cases = vec![
        (0, 1, "Invalid grade 0"),
        (7, 1, "Invalid grade 7"),
        (1, 0, "Invalid term 0"),
        (1, 3, "Invalid term 3"),
    ];
    
    for (grade, term, description) in invalid_cases {
        let result = get_new_chars(grade, term);
        let array = Array::from(&result);
        if array.length() == 0 {
            println!("✓ {}: Returns empty array as expected", description);
        } else {
            println!("✗ {}: Should return empty array but got {} characters", description, array.length());
        }
    }
}

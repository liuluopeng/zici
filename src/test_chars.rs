use wasm_zici::{get_all_new_chars_from_txt, get_new_chars};

fn main() {
    println!("正在加载汉字...");

    // 测试加载文件
    match get_all_new_chars_from_txt() {
        Ok(_) => println!("汉字加载成功！"),
        Err(e) => {
            println!("加载失败: {}", e);
            return;
        }
    }

    // 测试获取不同年级学期的生字
    test_get_chars(1, 1);
    test_get_chars(1, 2);
    test_get_chars(3, 1);
    test_get_chars(6, 2);

    // 测试无效输入
    test_get_chars(0, 1);
    test_get_chars(7, 1);
    test_get_chars(1, 3);
}

fn test_get_chars(grade: usize, term: usize) {
    let chars = get_new_chars(grade, term);
    println!(
        "{}年级{}学期: {}个汉字, 前10个: {:?}",
        grade,
        term,
        chars.len(),
        &chars[0..std::cmp::min(10, chars.len())]
    );
}

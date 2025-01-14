fn main() {
    // 验证18位身份证号
    println!("{}", id_validator::is_valid("320706198809015782")); // true

    // 验证18位身份证号
    println!("{}", id_validator::is_valid("320706199909015782")); // false

    // 模拟生成身份证号
    println!("{}", id_validator::gen_code());
}

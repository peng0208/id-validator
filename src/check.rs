use chrono::NaiveDate;

// 加权因子
static WEIGHT: [u32; 18] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2, 1];
// 校验码数组
static CHECK_CODES: &str = "10X98765432";

pub fn is_valid(id: &str) -> bool {
    if id.len() != 18 {
        return false;
    }
    is_valid_18(id)
}

fn is_valid_18(id: &str) -> bool {
    if !is_valid_date(&id[6..14]) {
        return false;
    }

    // 取第18位校验码
    let check_code = &id[17..].chars().next().unwrap();
    // 计算实际校验码
    let cal_code = cal_check_code(id);
    cal_code.eq(check_code)
}

// 计算校验码
pub(crate) fn cal_check_code(id: &str) -> char {
    let mut sum = 0;
    for (i, c) in id[..17].chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            sum += digit * WEIGHT[i];
        }
    }
    CHECK_CODES.chars().nth((sum % 11) as usize).unwrap()
}

// 检查日期
fn is_valid_date(date_str: &str) -> bool {
    if date_str.len() != 8 {
        return false;
    }
    NaiveDate::parse_from_str(date_str, "%Y%m%d").is_ok()
}

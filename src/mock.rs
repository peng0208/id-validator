use crate::check::cal_check_code;
use crate::region::REGION_CODES;
use chrono::{Duration, Local};
use rand::distributions::{Distribution, Uniform};
use std::ops::Range;

// 生成符合规则的身份证号
pub fn gen_code() -> String {
    let id = format!("{}{}{}", gen_area(), gen_date(), gen_range(0..1000));
    format!("{}{}", id, cal_check_code(id.as_str()))
}

fn gen_area() -> String {
    let areas = REGION_CODES.keys().collect::<Vec<_>>();
    areas[gen_range(0..areas.len())].to_string()
}

fn gen_date() -> String {
    let dt = Local::now() - Duration::days(gen_range(6750..15000) as i64);
    dt.format("%Y%m%d").to_string()
}

fn gen_range(range: Range<usize>) -> usize {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(range);
    die.sample(&mut rng)
}

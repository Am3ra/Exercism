pub fn is_armstrong_number(num: u32) -> bool {
    let s:String = num.to_string();
    let mut num : i32 = num as i32;
    s.chars().for_each(|x| num -= x
        .to_digit(10)
        .unwrap()
        .pow(s.len() as u32) as i32);

    num == 0
}

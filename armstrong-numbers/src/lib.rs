

pub fn is_armstrong_number(num: u32) -> bool {
    let mut unit: u32 = 1;
    let mut digit_count = 1;
    while num >= unit * 10{
        unit *= 10;
        digit_count += 1;
    }
    let mut temp_num = num;
    let mut res = 0;
    while unit > 0{
        res += (temp_num / unit).pow(digit_count);
        temp_num = temp_num % unit;
        unit = unit / 10;
    }
    return res == num;
}



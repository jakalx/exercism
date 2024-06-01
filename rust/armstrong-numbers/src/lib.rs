pub fn is_armstrong_number(num: u32) -> bool {
    let num: u64 = num.into();
    let v: Vec<_> = digits(num);
    num == v.iter().map(|x| x.pow(v.len() as u32)).sum()
}

fn digits(num: u64) -> Vec<u64> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().into())
        .collect()
}

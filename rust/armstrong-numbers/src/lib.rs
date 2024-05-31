pub fn is_armstrong_number(num: u32) -> bool {
    let v: Vec<_> = digits(num as u64).collect();
    num as u64 == v.iter().map(|x| x.pow(v.len() as u32)).sum()
}

fn digits(mut num: u64) -> impl Iterator<Item = u64> {
    let mut divisor = 1;
    while num >= divisor * 10 {
        divisor *= 10;
    }

    std::iter::from_fn(move || {
        if divisor == 0 {
            None
        } else {
            let v = num / divisor;
            num %= divisor;
            divisor /= 10;
            Some(v)
        }
    })
}

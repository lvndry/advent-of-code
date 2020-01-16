fn main() {
    let start = 248345;
    let end = 746315;

    let count = (start..=end).into_iter().filter(is_valid).count();

    println!("count {}", count);
}

fn is_valid(num: &u32) -> bool {
    let digits = [
        num / 1000000 % 10,
        num / 100000 % 10,
        num / 10000 % 10,
        num / 1000 % 10,
        num / 100 % 10,
        num / 10 % 10,
        num / 1 % 10,
    ];

    is_sorted(&digits)
        && digits.windows(2).enumerate().any(|(i, win)| {
            win[0] == win[1]
                && *digits.get(i - 1).unwrap_or(&0) != win[0]
                && *digits.get(i + 2).unwrap_or(&0) != win[0]
        })
}

fn is_sorted<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}

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

    is_sorted(&digits) && digits.windows(2).any(|win| win[0] == win[1])
}

fn is_sorted<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}

// fn calculate_possible_passwd(start: u32, _end: u32) -> u32 {
//     find_strictly_increasing_num(start, 6)
// }
//
// fn find_strictly_increasing_num(start: u32, len: i32) -> u32 {
//     let mut sum = 0;
//     if len == 0 {
//         sum += 1;
//     }
//
//     for i in start..9 {
//         find_strictly_increasing_num(i + 1, len - 1);
//     }
//
//     sum
// }

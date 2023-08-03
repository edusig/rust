fn number_of_steps(num: i32) -> i32 {
    let mut cur = num;
    let mut steps = 0;
    while cur > 0 {
        if cur % 2 == 0 {
            cur = cur/2;
        } else {
            cur = cur-1;
        }
        steps += 1;
    }
    steps
}

fn number_of_steps_v2(num: i32) -> i32 {
    match num > 1 {
        true => (num.count_zeros() - num.leading_zeros() + 2 * num.count_ones() - 1) as i32,
        false => num,
    }
}

fn main() {
    println!("{:?}", number_of_steps(14));
    println!("{:?}", number_of_steps(8));
    println!("{:?}", number_of_steps(123));

    println!("{:?}", number_of_steps_v2(14));
    println!("{:?}", number_of_steps_v2(8));
    println!("{:?}", number_of_steps_v2(123));
}

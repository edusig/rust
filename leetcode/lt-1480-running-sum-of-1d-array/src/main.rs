pub fn running_sum_v1(nums: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::with_capacity(nums.len());
    let mut sum: i32 = 0;
    for i in nums {
        output.push(i + sum);
        sum += i;
    }
    return output;
}

pub fn running_sum_v1_1(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    let mut output: Vec<i32> = Vec::new();
    for i in nums {
        sum += i;
        output.push(sum);
    }
    output
}

pub fn running_sum_v2(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    nums.iter().map(|x| {sum+=x; sum}).collect()
}

fn main() {
    println!("{:?}", running_sum_v1(vec![1,2,3,4]));
    println!("{:?}", running_sum_v1(vec![1,1,1,1,1]));
    println!("{:?}", running_sum_v1(vec![3,1,2,10,1]));
    
    println!("{:?}", running_sum_v1_1(vec![1,2,3,4]));
    println!("{:?}", running_sum_v1_1(vec![1,1,1,1,1]));
    println!("{:?}", running_sum_v1_1(vec![3,1,2,10,1]));
    
    println!("{:?}", running_sum_v2(vec![1,2,3,4]));
    println!("{:?}", running_sum_v2(vec![1,1,1,1,1]));
    println!("{:?}", running_sum_v2(vec![3,1,2,10,1]));
}

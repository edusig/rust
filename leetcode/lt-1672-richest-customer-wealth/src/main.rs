pub fn maximum_wealth_v1(accounts: Vec<Vec<i32>>) -> i32 {
    let mut max = 0;
    for person in accounts {
        let mut sum = 0;
        for account in person {
            sum += account;
        }
        if sum > max {
            max = sum;
        }
    }
    max
}

pub fn maximum_wealth_v2(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter().fold(0, |max, p| {
        let cur = p.iter().fold(0, |sum, a| sum + *a);
        if cur > max {cur} else {max}
    })
}

fn maximum_wealth_v2_1(accounts:Vec<Vec<i32>>) -> i32 {
    accounts.iter().map(|x| x.iter().sum()).max().unwrap()
}

fn main() {
    let v1 = vec![vec![1,2,3],vec![3,2,1]];
    let v2 = vec![vec![1,5],vec![7,3],vec![3,5]];
    let v3 = vec![vec![2,8,7],vec![7,1,3],vec![1,9,5]];

    println!("{:?}", maximum_wealth_v1(v1.clone()));
    println!("{:?}", maximum_wealth_v1(v2.clone()));
    println!("{:?}", maximum_wealth_v1(v3.clone()));
    
    println!("{:?}", maximum_wealth_v2(v1.clone()));
    println!("{:?}", maximum_wealth_v2(v2.clone()));
    println!("{:?}", maximum_wealth_v2(v3.clone()));

    println!("{:?}", maximum_wealth_v2_1(v1.clone()));
    println!("{:?}", maximum_wealth_v2_1(v2.clone()));
    println!("{:?}", maximum_wealth_v2_1(v3.clone()));
}
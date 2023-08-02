pub fn fizz_buzz_v1(n: i32) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for i in 1..n+1 {
        if i % 3 > 0 && i % 5 > 0 {
            output.push((i).to_string());
            continue;
        }
        let mut val = String::from("");
        if i % 3 == 0 {
            val.push_str("Fizz");
        } 
        if i % 5 == 0 {
            val.push_str("Buzz");
        }
        output.push(val);
    }
    output
}

pub fn fizz_buzz_v2(n: i32) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for i in 1..n+1 {
        if i % 15 == 0 {
            output.push(String::from("FizzBuzz"));
        } else if i % 5 == 0 {
            output.push(String::from("Buzz"));
        } else if i % 3 == 0 {
            output.push(String::from("Fizz"));
        } else {
            output.push(i.to_string());
        }
    }
    output
}

pub fn fizz_buzz_v3(n: i32) -> Vec<String> {
    (1..n+1).into_iter().map(|i| {
        if i % 15 == 0 {
            return String::from("FizzBuzz");
        } else if i % 5 == 0 {
            return String::from("Buzz");
        } else if i % 3 == 0 {
            return String::from("Fizz");
        } 
        return i.to_string();
    }).collect()
}

fn main() {
    println!("{:?}", fizz_buzz_v1(3));
    println!("{:?}", fizz_buzz_v1(5));
    println!("{:?}", fizz_buzz_v1(15));

    println!("{:?}", fizz_buzz_v2(3));
    println!("{:?}", fizz_buzz_v2(5));
    println!("{:?}", fizz_buzz_v2(15));

    println!("{:?}", fizz_buzz_v3(3));
    println!("{:?}", fizz_buzz_v3(5));
    println!("{:?}", fizz_buzz_v3(15));
}

fn main() {
    reading_elements();
}

fn reading_elements() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element!"),
    }

    let not_found = v[100]; // this will panic
    let not_found = v.get(100);
}
// https://en.wikipedia.org/wiki/Pig_Latin
// fn pig_latin(input: &str) {
//     let first = String::from(input).chars().next().unwrap();
// }

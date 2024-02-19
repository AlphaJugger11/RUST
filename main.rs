fn main() {
    let s1 = String::from("hello ");
    let s2: String = String::from("world");
    let s3 = concatenate_strings(s1, s2);
    println!("{}", s3);
}

fn concatenate_strings(s1: String, s2: String) -> String {
    let mut result: String = String::from("");
    result.push_str(&s1);
    result.push_str(&s2);
    result
}

fn main() {
    let s1 = String::from("ahoy");
    let len = calculate_length(&s1);

    println!("s1 = {s1}, len = {len}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10 {
            break counter * 2;
        };
    };
    println!("counter = {counter}");
    println!("result = {result}");

    let mut number = 5;
    while number != 0 {
        println!("number = {number}");
        number -= 1;
    };
    println!("OUT");

    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("{element}");
    };
    for element in (1..6).rev() {
        println!("{element}");
    };
}

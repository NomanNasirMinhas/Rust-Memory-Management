use std::io;

fn main() {
    println!("Hello, Welcome to Memory Management");
    let num1 = String::from("Hello from Num 1");
    let num2 = num1;

    println!("NUM 2 = {}", num2);
    // println!("NUM 1 = {}", num1); //This Line Will Throw Error

    let num3 = "Hello from Num 3";
    let num4 = num3; //Shallow Copying by Default for Strings

    println!("NUM 3 = {}", num3);
    println!("NUM 4 = {}", num4);

    let num5 = String::from("Hello from Num 5");
    let num6 = num5.clone(); // Deep Copying

    println!("NUM 5 = {}", num5);
    println!("NUM 6 = {}", num6);

    let num7 = 9999;
    let num8 = num7; // Deep Copying By Default

    println!("NUM 7 = {}", num7);
    println!("NUM 8 = {}", num8);

    println!("-----------------------------------------------");
    let mut input = String::new();
    println!("Please Enter Your Text");
    io::stdin().read_line(&mut input).expect("Error in Reading Input");
    // let length = calculate_lenghth(input); //Throws Error as input is deleted after this
    let length = calculate_lenghth(&input);

    println!("Length of {} is = {}",input,length);

    println!("-----------------------------------------------");
    let mut input2 = String::new();
    println!("Please Enter Your Name");
    io::stdin().read_line(&mut input2).expect("Error in Reading Input");
    add_label(&mut input2);
    println!("Label is \n{}",input2);
}

fn calculate_lenghth(text: &String) -> usize{
    text.trim().len()
}

fn add_label (text: &mut String){
    text.push_str(" Is Your Name");
}
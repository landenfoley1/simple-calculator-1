
use std::io;

fn main() {
    let res:u32;
    let num1:u32;
    let num2:u32;
    let res2:u32;
    let mut input = String::new();
    println!("calculator, Enter 2 numbers then pick what you want to do with it.");
    println!("Enter number 1: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    num1 = input.trim().parse().expect("Not a valid number");  
    println!("Number is: {}",num1);
    println!("Enter number 2: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a valid string");
    
    num2 = input.trim().parse().expect("Not a valid number");  
    println!("Number is: {}",num2);
    println!("add = a, subtract = s, multiply = m, divide = d, power = p");
    println!("what do you want to do ");
    let mut word = String::new();

    io::stdin().read_line(&mut word).expect("failed to readline");
    let word = word.trim();
    println!("You entered {}", word);
    if word == "a"{
        res = num1 + num2;
        println!("your sum is {}",res) 
    }else if word == "s"{
        res = num1 - num2;
        println!("your diffrence is {}",res)  
    }else if word == "m"{
        res = num1 * num2;
        println!("your pruduct is {}",res)
    }else if word == "d"{
        res = num1 / num2;
        res2 = num1 % num2;
        println!("your quotient is {} and your remainder is {}",res, res2)
    }else if word == "p"{
        res = num1.pow(num2);
        println!("the power is {}", res)
    }else{
        println!("invalid")
    }
    
}
    
    

    
    
    


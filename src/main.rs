use std::io;

struct Student {
    name: String,
    grades: Vec<i32>,
}

fn main() {
    println!("Grade Calculator");
    let mut inputs: Vec<i32> = vec![];
    let student_name= read_name_input();
    println!("Enter your grades to calculate your average:");
    let mut count = 0;
    while count < 3 {
        let input = read_input();
        let int_version: i32 = input.parse::<i32>().unwrap();
        inputs.push(int_version);
        count += 1;
    }
    let student = Student {
        name: student_name,
        grades: inputs,
    };
    print!("Hello, {}! You got a: ", student.name);
    let second_avg = calculate_average(&student.grades);
    match second_avg {
        70.0.. => println!("First."),
        60.0..69.00 => println!("2:1."),
        50.0..59.00 => println!("2:2."),
        40.0..49.0 => println!("Third."),
        _ => println!("Fail."),
    }
}

fn read_name_input() -> String {
    println!("Enter your name: ");
    let mut name_input = String::new();
    let mut count = 0;
    while count < 1 {
        io::stdin().read_line(&mut name_input).expect("Failed to read input");
        count += 1;
    }
    return name_input.trim().to_string();
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    return input.trim().to_string();
}

fn calculate_average(grades: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for grade in grades {
        sum += grade;
    }
    return sum as f64 / grades.len() as f64;
}
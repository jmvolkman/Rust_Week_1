use std::vec;
use std::io;

fn main() {

    println!("Hello, JMV!");

    arg_fn();
    //number_test();
    //condition();
    //array_test();
    //junk_test();
    //health_info();
    //loop_test();
    //match_try();
    //main_car();

    println!("Goodbye, JMV!");

//end of main
}
//sub functions to call
fn arg_fn(){
    println!("Please enter number of elements to sum:");
   
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    println!("Entered: {}", input);

    let count:i32=  match input.parse::<i32>() {
        Ok(n) => n,
        Err(e) => -1,
    };2

    println!("Parsed: {}", input);

    let mut total = 0;
    if count > 0 {
        for int_loop in 1..=count {
            println!("Entry {}: ", int_loop);
            io::stdin().read_line(&mut input).expect("Failed to read input");
            total += input.parse::<i32>().unwrap();
        }
    }
     
    println!("The total is {}", total);

    // There are no variadic arguments in Rust
    let numbers = [];
    let result = sum(&numbers);
    println!("The sum is {}", result);
}
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

#[allow(dead_code)]
fn number_test(){
    let numbers = [1, 2, 3, -5];   // Include a negative number to trigger the panic
    print_sum(&numbers);          // Call the unit function with the slice as an argument
    process_numbers(&numbers);     // Call function with slice of integers as an argument
}
#[allow(dead_code)]
// a unit function that doesn't return anything
fn print_sum(numbers: &[i32]) {
    let sum:i32= numbers.iter().sum(); // Calculate the sum of elements in slice
    if sum % 2 == 0 {               // Check if sum is even
        println!("The sum is even.");
    } else {
        println!("The sum is odd.");
    }
}
#[allow(dead_code)]
fn process_numbers(slice: &[i32]) {
    for (index, number) in slice.iter().enumerate() {
        if *number < 0 {
            panic!("Negative number found at index {}", index); // Stop execution and show error message
        }
    }
}
#[allow(dead_code)]
fn array_test(){

    // Declare array, initialize all values, compiler infers length = 7
    let _days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    
    // Declare array, initialize all values to 0, length = 5
    let _bytes = [0; 5];

    // Declare vector, initialize with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);  
    
    // Declare vector, value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    // Create empty vector, declare vector mutable so it can grow and shrink
    let mut fruit = Vec::new();
    // Push values onto end of vector, type changes from generic `T` to String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);

    // Declare vector, initialize with three values
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);
// Add 5 to the value at index 1, which is 5 + 3 = 8
index_vec[1] = index_vec[1] + 5;
println!("Vector: {:?}", index_vec);

}
#[allow(dead_code)]
fn condition(){
    let formal = true;
    let greeting = if formal { // if used here as an expression
    "Good day to you."     // return a String
    } else {
    "Hey!"                 // return a String
    };
    println!("{}", greeting);   // prints "Good day to you."

    let num = 500; // num variable can be set at some point in the program
    let out_of_range: bool;
    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }

    if out_of_range { // if used here as an expression
        println!("out of range");    // return a String
        } else {
            println!("good input");                 // return a String
        };
    
}
#[allow(dead_code)]
fn match_try(){

    println!("Please enter a greeting:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    let str_hello: &'static str ="hello";
    let str_good_bye: &'static str ="good bye";
    let str_how_are_you: &'static str ="how are you?";

    // use of match expression to pattern match against variable "name"
    match name.to_lowercase().trim() {
        //str_how_are_you => println!("I'm doing well!"),
        "How are you?"|"how are you?" => println!("I'm doing well!"), 
        "Good bye"|"Good Bye"|"good bye" => println!("Sorry to see you go."),
        "hello"|"Hello" => println!("Hi, nice to meet you!"),
        "good morning" => println!("Good day!"),
        "good evening" => println!("Good night!"),
        //str_good_bye => println!("Sorry to see you go."),
        //str_hello => println!("Hi, nice to meet you!"),        
        _ => println!("I can't find a greeting, good bye."),
    }

    println!("Input: {}:{}:{}:{}", name.trim(), str_good_bye, str_hello, str_how_are_you);
    

    println!("Please enter a greeting:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim() {
        "Good Bye" => println!("Good Bye"),
        "Hello" => println!("Hello"),
        "World" => println!("World"),
        _ => println!("Other: {}", input.to_lowercase().trim()),
    }
}
#[allow(dead_code)]
fn loop_test(){
    // use 1..10 or 1..=10
    for int_loop in 1..=10 {
        println!("int_loop = {}", int_loop);
    }

    for int_loop in (1..=10).rev() {
        println!("int_loop = {}", int_loop);
    }

    let  numbers = vec!{1,2,3,4,5};
    for number in numbers {
        println!("number = {}", number);
    }

    for int_loop in 1..=10 {
        if int_loop % 2 == 0 {
            //skip even numbers
            continue;
        }
        println!("int_loop = {}", int_loop);
        if int_loop == 7 {
            //exit loop when 7
            break;
        }
    }
}
#[allow(dead_code)]
fn junk_test(){
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    //let maybe_number = Some(42);
    let maybe_number: Option<Option<()>> = Some(None);
if let Some(number) = maybe_number {
    println!("The number is: {:?}", number);
} else  {
    println!("There is no number!");
    }


    let spaces = "   ";
    
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, _z) = tup;
    
    println!("The value of x is: {}", x);
    println!("The value of **y** is: **{}**", y);

    let bln_proceed = true;
    if bln_proceed {
        println!("Proceeding...");
    } else {
        println!("Not Proceeding...");
    }
    
}
#[allow(dead_code)]
fn health_info(){

    let gender: &str = "Male";
    let age = 41;

let mut agemessage: String = "".to_string();

if age > 19 {

    agemessage.push_str(&age.to_string());
    agemessage.push_str(" year old ");
    agemessage.push_str(adult(gender));
   

} else if age > 12 {

        agemessage.push_str(&age.to_string());
        agemessage.push_str(" year old ");
        agemessage.push_str(teen(gender));
       

    } else {
        agemessage.push_str(&age.to_string());
        agemessage.push_str(" year old ");
        agemessage.push_str(child(gender));
    }

    println!("{}",agemessage);

    let mut height = 179;
    height = height - 20;

   let height_message: &str = if height > 180 {
        "Tall"
    } else if height > 170 {
        "Average"
    } else {    
        "Short"
    };
    println!("Height Result: {}", height_message);

    let health = if height < 180 {"good"} else {"unknown"};
    println!("Health Result: {}", health);

    //let health = if height < 180 {true} else {false};

}


fn adult(gender: &str)-> &'static str {
    if gender.to_lowercase().starts_with("m") {
        "Man"
    } else if gender.to_lowercase().starts_with("f") {
       "Woman"
    } else {
        "Human"
    }
}
fn teen(gender: &str) -> &'static str{
    if gender.to_lowercase().starts_with("m") {
        "Teenage Boy"
    } else if gender.to_lowercase().starts_with("f")  {
        "Teenage Girl"
    } else {
        "Teenager"
    }
}
fn child(gender: &str) -> &'static str{

    if gender.to_lowercase().starts_with("m") {
        "Boy"
    } else if gender.to_lowercase().starts_with("f")  {
        "Girl"
    } else {
        "Child"
    }
}


#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {    color: String,    motor: Transmission,    roof: bool,    age: (Age, u32)}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission { Manual, SemiAuto, Automatic }
#[derive(PartialEq, Debug)]
enum Age { New, Used }
#[allow(dead_code)]
fn main_car() {

    // Initialize a hash map for the car orders
    // - Key: Car order number, i32
    // - Value: Car order details, Car struct
    use std::collections::HashMap;
    let mut orders:  HashMap<i32, Car> = HashMap::new();
    
    // Initialize counter variable
    let order = 1;
    // Declare a car as mutable "Car" struct
    let mut car: Car;
    // Start with zero miles
    let mut miles = 0;

    // Call car_factory to fulfill order
    // Add order <K, V> pair to "orders" hash map
    // Call println! to show order details from the hash map     

    let order_total = 11;
    for order in order..=order_total {

        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        // Reset miles for order variety
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }

}

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality (miles: u32) -> (Age, u32) {

    // Check if car has accumulated miles
    // Return tuple early for Used car
    if miles > 0 {
        return (Age::Used, miles);
    }

    // Return tuple for New car, no need for "return" keyword or semicolon
    (Age::New, miles)
}

// Build "Car" using input arguments
fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Prevent panic: Check color index for colors array, reset as needed
    // Valid color = 1, 2, 3, or 4
    // If color > 4, reduce color to valid index
    let mut color = order as usize;
    if color > 4 {        
        // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
        color = (color % 4) + 1;
    }

    // Add variety to orders for motor type and roof type
    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {          // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {   // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    }                            // 1, 5, 7, 11

        // Show details about car order
    // Corrected code: If order is for Used car, check roof type, print details
    // Corrected code: Else, order is for New car, check roof tye, print details
    // Call the `println!` macro to show the car order details
    if car_quality(miles).0 == Age::Used {
        if roof {
            println!("Preparing a used car: {:?}, {}, Hard top, {} miles", motor, color, miles);
        } else {
            println!("Preparing a used car: {:?}, {}, Convertible, {} miles", motor, color, miles);
        }
    } else {
        if roof {
            println!("Building a new car: {:?}, {}, Hard top, {} miles", motor, color, miles);
        } else {
            println!("Building a new car: {:?}, {}, Convertible, {} miles", motor, color, miles);
        }
    }

    // Return requested "Car"
    Car {
        color: String::from(colors[(color-1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

//EOF
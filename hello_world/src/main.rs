fn main() {
    println!("Hello, world!");

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

    let gender: &str = "Male";
    let age = 41;

let mut agemessage: String = "".to_string();

if age > 19 {

    agemessage.push_str(&age.to_string());
    agemessage.push_str(" year old ");
    agemessage.push_str(adult(gender));
   

}
    if age > 12 {

        agemessage.push_str(&age.to_string());
        agemessage.push_str(" year old ");
        agemessage.push_str(teen(gender));
       

    } else {
        agemessage.push_str(&age.to_string());
        agemessage.push_str(" year old ");
        agemessage.push_str(child(gender));
    }

    println!("{}",agemessage);

    let height = 179;
    if height > 180 {
        println!("Tall")
    } else if height > 170 {
        println!("Average")
    } else {    
        println!("Short")
    }

    //end of main
    }

fn adult(gender: &str)
-> &'static str {
    if gender.to_lowercase().starts_with("m") {
        "Man"
    } else if gender.to_lowercase().starts_with("f") {
       "Woman"
    } else {
        "Human"
    }
}
fn teen(gender: &str) -> &'static str
{
    if gender.to_lowercase().starts_with("m") {
        "Teenage Boy"
    } else if gender.to_lowercase().starts_with("f")  {
        "Teenage Girl"
    } else {
        "Teenager"
    }
}
fn child(gender: &str) -> &'static str
{
    if gender.to_lowercase().starts_with("m") {
        "Boy"
    } else if gender.to_lowercase().starts_with("f")  {
        "Girl"
    } else {
        "Child"
    }
}
use std::io;

fn main() {


// Create a mutable string variable to store user input
    let mut input = String::new(); 
  

println!("Performing addition and subtraction");
    // Prompt the user to enter the value for x
        println!("Enter the value for x:"); 
        io::stdin()
            .read_line(&mut input)
            .expect("Read error"); 
    // Read the input from the user and handle any potential errors




// Parse the input string to an i32 value, trimming any whitespace and handling parsing errors
    let x: i32 = input.trim().parse().expect("Invalid number"); 
    input.clear(); 
// Clear the input string to reuse it for the next input



    // Prompt the user to enter the value for y
        println!("Enter the value for y:"); 
        io::stdin()
            .read_line(&mut input)
            .expect("Read error"); 
    // Read the input from the user and handle any potential errors



// Parse the input string to an i32 value, trimming any whitespace and handling parsing errors
    let y: i32 = input.trim().parse().expect("Invalid number"); 


    let sum = x + y;
    let difference = x - y;


    println!("x = {}, y = {}", x, y);
    println!("Sum = {}", sum );
    println!("Difference = {}", difference)
// Print the values of x, y, sum & difference 

}

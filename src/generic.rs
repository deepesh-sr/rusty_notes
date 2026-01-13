// Generics are often implemented in a place where we are going to define our data type in the parameters. 
fn largest_int(input : &[i32])-> &i32{
    let mut largest_value = &input[0];

    for item in input {
        if item > largest_value {
            largest_value = item;
        }
    }
    largest_value
}

fn largest_chat(input : &[char])-> &char{
    let mut largest_value = &input[0];

    for item in input {
        if item > largest_value {
            largest_value = item;
        }
    }
    largest_value
}

/*
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}

*/


// here we have two different function that does the same thing. 
// what if we could also make this 2 function a single one and this too works different data types. ( generic data types ) with specific traits. 

// there introduced generics 

// Syntax 

/*
fn largest<T : std::cmp::PartialOrd> (list : &[T])-> &T {
    

// we read this as " Function Largest is generic over some type T"

// Here we must implement a trait so that we can perform arthemetic operations on T , that means T should have some 
// properties, so that we can perform operation on it. 


}

this will work for any data structure that has a trait std::cmp::PartialOrd. 
*/
fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    
    // Option 1: Initialize a fixed-size array with 100 elements, all set to 0
    // let a: [i32; 100] = [0; 100];

    // Option 2: Create an empty vector and populate it by pushing numbers from 1 to 100
    // let mut a: Vec<i32> = vec![];
    // for i in 1..=100 {
    //     a.push(i);
    // }

    // Option 3: Generate a vector containing numbers from 1 to 100 using the `collect` method
    let a: Vec<i32> = (1..=100).collect();


    

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}

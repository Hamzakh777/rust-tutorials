use std::io;

fn main() {
    println!("Enter your weight in KG: ");
    // the string is owned by input var - its stored in the heap since its size is unknow at compile time
    // String is type of smart pointer
    // when main function exists, the drop funciton will be executed on String
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // we need to convert the string into a float
    let weight: f32 = input.trim().parse().unwrap();
    // all vars in Rust are immutable, you have to explicitly tell rust if a var should be mutable
    let mars_weight = calculate_weight_on_mars(weight);
    // every time we see an exclamation mark, that means its a macro
    println!("Weight on Mars: {}kg", mars_weight);
}


fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.82) * 3.711
    // the last statement that doesn't have `;` in a function will be returned
}
// ownership in rust
// 1. each value in rust has an owned by a variable (the owner)
// 2. when the owner goes out of scope, the value will be dealocated.
// 3. there can only be one owner at a time
// ===> means we can't pass a var as param to a function (fn) and expect to use the same var
// in its initial scope after (fn) exists (transfering ownership)

// to overcome that, Rust has a feature called Refrences. You pass a var to a function 
// without changing ownership. Add `&` at the start of the var when passing it as param

// `&mut` basically says no borrowing and mutable
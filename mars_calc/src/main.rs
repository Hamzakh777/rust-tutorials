use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    // all vars in Rust are immutable, you have to explicitly tell rust if a var should be mutable
    let mars_weight = calculate_weight_on_mars(80.1);
    // every time we see an exclamation mark, that means its a macro
    println!("Weight on Mars: {}kg", mars_weight);
}


fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.82) * 3.711
    // the last statement that doesn't have `;` in a function will be returned
}

// ownership in rust


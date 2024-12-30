// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// Don't worry about the function bodies themselves, we are only interested in
// the signatures for now.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Fix the function signature.
fn sale_price(price: i64) -> i64{
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn say_even_or_odd(num: i64) -> String {
    if is_even(num) {
        "even".to_string()
    } else {
        "odd".to_string()
    }
}

fn main() {
    let original_price: i64 = 359;
    let odd_even:&str = if is_even(original_price) {
        "even"
    } else {
        "odd"
    };
    println!("Your sale price is {}", sale_price(original_price));
    println!("The price was {} and it is {}", original_price, say_even_or_odd(original_price));
    println!("The price was {} and it is {}", original_price, odd_even);
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fibonacci(n: u32) -> u32 {
    if n == 1 || n == 0 {
        n
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}

fn carol() {
    
}
fn main() {
    let celsius = 37.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}°C is {}°F", celsius, fahrenheit);
    println!("Fibonacci term 6 is {}", fibonacci(6));
}


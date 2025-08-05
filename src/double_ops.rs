pub fn add_doubles(a: f64, b: f64) -> f64 {
    a + b
}

pub fn multiply_doubles(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide_doubles(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}
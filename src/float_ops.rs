pub fn add_floats(a: f32, b: f32) -> f32 {
    a + b
}

pub fn multiply_floats(a: f32, b: f32) -> f32 {
    a * b
}

pub fn divide_floats(a: f32, b: f32) -> Result<f32, &'static str> {
    if b == 0.0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

pub fn sqrt_float(a: f32) -> f32 {
    a.sqrt()
}
use num::Float;
pub trait FloatP: Float {
    fn value(x: f64) -> Self;
    
    
}

impl FloatP for f32 {
    fn value(x: f64) -> Self {
        x as f32
    }
}

impl FloatP for f64 {
    fn value(x: f64) -> Self {
        x
    }
}
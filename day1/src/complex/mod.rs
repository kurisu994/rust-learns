use std::f64::consts::PI;

mod complex1;
mod complex2;

/// 复杂数据结构的display
pub fn complex_display() {
    complex1::show();
    // Display 格式
    println!("Pi is roughly {:>.*}", 3, PI);
    complex2::complex2::show();
}
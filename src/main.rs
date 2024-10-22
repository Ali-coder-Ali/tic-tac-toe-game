

// main.rs

mod math;

fn main() {
    let sum = math::add(5, 3);
    let difference = math::subtract(10, 4);

    println!("نتيجة الجمع: {}", sum);          // 8
    println!("نتيجة الطرح: {}", difference);   // 6
}

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let index = 5; // Index out of bounds

    // This will panic at runtime
    println!("Value at index {}: {}", index, vec[index]);
}
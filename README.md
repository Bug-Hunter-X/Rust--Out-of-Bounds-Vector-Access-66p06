# Rust: Out-of-Bounds Vector Access

This repository demonstrates a common runtime error in Rust: panicking due to an out-of-bounds index when accessing a vector. The `bug.rs` file contains the erroneous code, while `bugSolution.rs` provides a corrected version.

The error occurs because the program attempts to access an element beyond the vector's valid index range. This is a frequent mistake, especially when dealing with user input or dynamic data structures.  Understanding how to handle these scenarios is vital for robust Rust programming.

The solution demonstrates safe and efficient ways to access vector elements and handle potential index errors.
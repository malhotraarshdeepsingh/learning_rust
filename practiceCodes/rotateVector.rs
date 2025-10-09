// Write a function that rotates a vector k positions to the right without allocating a new Vec.

fn rotate_vector<T> (Vec<T>, k: usize) -> Vec<T> {
    let len = V.len();

    if len == 0 { return V; }

    let k = k % len; // Handle cases where k >= len
    let mut result = Vec::with_capacity(len);

    result.extend(V[len - k..].iter().cloned());
    result.extend(V[..len - k].iter().cloned());

    result
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let k = 2;
    let rotated = rotate_vector(v, k);
    println!("{:?}", rotated); // Output: [4, 5, 1, 2, 3]
}
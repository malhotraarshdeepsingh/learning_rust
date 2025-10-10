// Implement this safely (without clones):
// fn interleave<T>(a: Vec<T>, b: Vec<T>) -> Vec<T>
// It should return [a1, b1, a2, b2, ...] even if a and b differ in length.
// Bonus: Can you make it generic over any iterable?

fn interleave<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let min_len = a.len().min(b.len());

    for i in 0..min_len {
        result.push(a[i].clone());
        result.push(b[i].clone());
    }

    result.extend_from_slice(&a[min_len..]);
    result.extend_from_slice(&b[min_len..]);

    result
}

// without clone
fn interleave_no_clone<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let min_len = a.len().min(b.len());

    for i in 0..min_len {
        result.push(a[i]);
        result.push(b[i]);
    }

    result.extend(a.into_iter().skip(min_len));
    result.extend(b.into_iter().skip(min_len));

    result
}

fn main() {
    let a = vec![1, 2, 3];
    let b = vec!['a', 'b', 'c', 'd', 'e'];
    let result = interleave(a, b);
    println!("{:?}", result); // Output: [1, 'a', 2, 'b', 3, 'c', 'd', 'e']
}

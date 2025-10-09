// Implement your own version of Box<T> — called MyBox<T> — which:
// Stores its data on the heap
// Implements Deref and Drop
// Works like the real Box<T> in basic use cases

struct MyBox<T> {
    ptr: *mut T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        let b = Box::new(value);
        MyBox {
            ptr: Box::into_raw(b),
        }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: you know self.ptr is valid because MyBox owns the value.
        unsafe { &*self.ptr }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

fn main() {
    let x = MyBox::new(42);
    println!("{}", *x);

    let mut y = MyBox::new(String::from("Hello"));
    (*y).push_str(" World!");
    println!("{}", y);
}

// You can extend it with:
// Clone (clone the heap value)
// From<T> (so MyBox::from(10) works)
// Debug / Display tra

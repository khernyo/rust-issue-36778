use std::thread;

fn main() {
    let child = thread::spawn(|| {
        std::cell::RefCell::new([0i8; 232500]);
    });
    child.join().unwrap();
}

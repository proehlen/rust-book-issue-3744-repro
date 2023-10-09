use std::sync::Mutex;
use std::thread::scope;

fn main() {
    let counter = Mutex::new(0);

    _ = scope(|s|{
        for _ in 0..10 {
            s.spawn(|| {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
        }
    });


    println!("Result: {}", *counter.lock().unwrap());
}

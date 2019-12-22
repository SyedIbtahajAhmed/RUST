use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10{
            println!("hi number {} from the spawn thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });
    handle.join().unwrap();

    thread::spawn(|| {
        for i in 1..10{
            println!("hi number {} from the spawn thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..5{
        println!("hi number {} from the Main thread!", i);
        thread::sleep(Duration::from_millis(2));
    }

    
}

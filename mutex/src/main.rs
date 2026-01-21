
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

fn main() {
    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6 ;
    // }
    // println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }
    println!("counter = {:?}", *counter.lock().unwrap());

}

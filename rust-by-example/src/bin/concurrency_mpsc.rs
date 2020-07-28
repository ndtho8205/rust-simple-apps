use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn timer(id: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(id as u64));
        println!("thread {}: sent!", id);
        tx.send(id).unwrap();
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        timer(i, tx.clone());
    }
    timer(10, tx);

    for received in rx {
        println!("main thread: {} received!", received);
    }
}

use std::{
    error::Error,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

// pub static NTHREADS: usize = 4;

pub type SendFunc<T> = Box<dyn Fn(T) -> Result<(), Box<dyn Error>> + Send>;

pub fn create_input_thread<T: Send + 'static>(doer: fn(SendFunc<T>)) -> Receiver<T> {
    let (tx, rx): (Sender<T>, Receiver<T>) = mpsc::channel();
    let mut children = Vec::new();

    let thread_tx = tx.clone();
    let child = thread::spawn(move || {
        println!("Thread started");
        let callback = Box::new(move |data: T| {
            thread_tx
                .send(data)
                .map_err(|e| Box::new(e) as Box<dyn Error>)
        });
        doer(callback);
        println!("Thread finished work");
    });
    children.push(child);
    rx
}

pub fn create_output_thread<T: Send + 'static, F: FnMut(T) + Send + 'static>(
    receiver: Receiver<T>,
    doer_creator: impl Fn() -> F,
    auto_kill: bool,
) {
    let mut doer = doer_creator();

    let mut children = Vec::new();

    let child = thread::spawn(move || {
        println!("Thread started");
        loop {
            let data = receiver.recv();
            match data {
                Ok(d) => doer(d),
                Err(e) => {
                    if auto_kill {
                        println!("Error receiving data: {}", e);
                        break;
                    }
                }
            }
        }
        println!("Thread finished receiving");
    });
    children.push(child);
}

use std::{thread,time::Duration};
use std::sync::{Arc, mpsc};

fn main() {
    /*
    In this example,although the spawned thread has more work,it'll stop halfway when the main thread gets executed.
    Irrespective of how much work is left or done,it dies with the main thread. To allow the secondary thread to run,
    a JoinHandle function is used which blocks the main(or any other) thread from executing until the handle thread
    has finished all it's executions. It simply blocks the thread that called join until the spawned thread is finished.
     */


    // thread::spawn(|| {                                           // Without handle,will die with the main thread
    //     for i in 1..10{
    //         println!("This is a number from spawned thread {}",i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    let handle = thread::spawn(|| {             // Blocks the main thread until it's finished
        for i in 1..10{
            println!("This is a number from spawned thread {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //handle.join().unwrap(); // if the method is called here, the spawn thread will first do all executions,then main.

    for i in 1..5{
        println!("This is a number from the main thread {}",i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();      // If it's called here,threads will alternate until main runs out,then spawn finishes.

    let v = vec![1,2,3];
    let hand = thread::spawn(move || {   // Without the move function,it won't take ownership and
        for i in v{                               // won't be operated in a spawned thread.
            println!("The vector {:?}",i);
        }
    });
    hand.join().unwrap();




    // ------------------------------------Channels---------------------------------------------
    use std::sync::mpsc;
    let (tx,rx) = mpsc::channel();
    let tx2 = tx.clone();   // Creating another transmitter to send messages to the main thread
    let tx3 = tx.clone();   // The transmitter expires after being moved to closure.

    thread::spawn(move || {
        let msg = String::from("Hello");
        tx.send(msg).unwrap();          // Unwrap() since return type is Result, handle gracefully in production
    });
    let rec = rx.recv().unwrap();   //try_recv() could be used when the thread has to do any other work
    println!("{}",rec);

    thread::spawn( move || {
        tx2.send(String::from("Hello")).unwrap();
        tx2.send(String::from("Cutie")).unwrap();
        tx2.send(String::from("Pie")).unwrap();
    });
    thread::spawn(move || {
        tx3.send(String::from("Coffee")).unwrap();
        tx3.send(String::from("with")).unwrap();
        tx3.send(String::from("me?")).unwrap();
    });
    for msg in rx{
        println!("Received --> {}",msg);        //rx can be used as an iterator for multiple messages
        thread::sleep(Duration::from_secs(1));
    }






    // ------------------------------------Shared State---------------------------------------------
    use std::sync::{Mutex,Arc};
     let m = Mutex::new(5);
    {
        let mut value = m.lock().unwrap();  // Lock makes sure that the data is accessed safely.
        *value = 6;
    }
    println!("{:?}",m);

    let counter = Arc::new(Mutex::new(0));
    let mut v = vec![];
    for _ in 1..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
           let mut value = counter.lock().unwrap();
            *value+=1;
        });
        v.push(handle);
    }
}

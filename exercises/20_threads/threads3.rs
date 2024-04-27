// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let q = Arc::new(q);
    let q_clone = Arc::clone(&q);
    let tx_clone = tx.clone();
    
    thread::spawn(move || {
        for val in &q_clone.first_half {
            println!("sending {:?}", val);
            tx_clone.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    let q_clone2 = Arc::clone(&q);
    let tx_clone2 = tx.clone();

    thread::spawn(move || {
        for val in &q_clone2.second_half {
            println!("sending {:?}", val);
            tx_clone2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

// ! Writeup !
// Le problème ici était que les threads se partageaient la même instance de Queue, 
// ce qui entraînait des problèmes de concurrence et de propriété.
// J'ai utilisé Arc pour résoudre le problème, en créant une copie de la queue pour chaque thread
// je peux partager la propriété de la queue entre eux et traiter les valeurs de manière indépendante.

#[test]
fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}

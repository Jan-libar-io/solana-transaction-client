use std::thread;

pub mod threads;
pub mod types;
pub mod services;

fn main() {
    // we can create multiple channels, one for each part of the chain
    // and send messages between them
    // later the entry point of the chain will be listining for http requests or websocket messages
    // and will send them to the sequencer
    let (transaction_sender, transaction_receiver) = std::sync::mpsc::channel();

    //each part of the sequencer will be spawned in a separate thread
    let sequencer_handle = thread::spawn(|| threads::sequencer::sequencer(transaction_receiver));

    // listen for keyboard input with the following options:
    // 1. Send a transaction
    // 2. Send a transaction every n seconds, where n is a number input from the keyboard
    // 3. Send n transactions as fast as possible, where n is a number input from the keyboard
    // 4. exit

    loop {
        println!("Please select an option:");
        println!("1. Send a transaction");
        println!("2. Send a transaction every n seconds");
        println!("3. Send n transactions as fast as possible");
        println!("4. exit");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => {
                transaction_sender.send(services::create_transaction()).unwrap();
            }
            "2" => {
                // send a transaction every n seconds
                println!("Please enter the number of seconds:");
                let mut seconds_input = String::new();
                std::io::stdin().read_line(&mut seconds_input).unwrap();
                let seconds: u64 = seconds_input.trim().parse().unwrap();
                loop {
                    transaction_sender.send(services::create_transaction()).unwrap();
                    thread::sleep(std::time::Duration::new(seconds, 0));
                }
            }
            "3" => {
                // send n transactions as fast as possible
                println!("Please enter the number of transactions:");
                let mut transactions_input = String::new();
                std::io::stdin().read_line(&mut transactions_input).unwrap();
                let transactions: u64 = transactions_input.trim().parse().unwrap();
                for _ in 0..transactions {
                    transaction_sender.send(services::create_transaction()).unwrap();
                }
            }
            "4" => {
                break;
            }
            _ => {
                println!("Invalid option");
            }
        }
    }

    // close channels
    drop(transaction_sender);

    // close threads
    sequencer_handle.join().unwrap();
}

use std::sync::mpsc::{Receiver, Sender};

use data::TicketDraft;

pub mod data;
pub mod store;

pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    // the thread spawned will be running even after it goes out of scope... 
    // that is why we usually use .join() on the handle or use scoped threads
    std::thread::spawn(move || server(receiver)); 
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    while let Ok(cmd) = receiver.recv() {
        match cmd {
            Command::Insert(draft_ticket) => println!("We received a draft ticket {:?}",draft_ticket),
        }
    }

}

// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, RecvError, Sender, SyncSender, TrySendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

#[derive(thiserror::Error,Debug)]
pub enum StoreError{
    #[error("The channel you're trying to connect is full please send the message again")]
    ChannelBusy{
        source: TrySendError<Command>,
    },
    #[error("The channel you're trying to connect is closed")]
    ChannelClosed{
        source: RecvError
    }
}

impl From<TrySendError<Command>> for StoreError {
    fn from(value: TrySendError<Command>) -> Self {
        Self::ChannelBusy{source:value}
    }
}
impl From<RecvError> for StoreError {
    fn from(value: RecvError) -> Self {
        Self::ChannelClosed{source:value}
    }
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, StoreError> {
        let (response_channel,receiver) = std::sync::mpsc::sync_channel(1);
        self.sender.try_send(Command::Insert { draft, response_channel })?;
        receiver.recv().map_err(|e|e.into())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, StoreError> {
        let (response_channel,receiver) = std::sync::mpsc::sync_channel(1);
        self.sender.try_send(Command::Get { id, response_channel })?;
        receiver.recv().map_err(|e|e.into())
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let(sender,receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

pub enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}

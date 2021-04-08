mod dispatcher;
mod handlers;

use self::dispatcher::{NotificationDispatcher, RequestDispatcher};
use crate::error::ProtocolError;
use crate::protocol::{Message, Notification, Request};
use crate::state::State;
use crate::Result;
use flume::Receiver;

/// The uninitialized server side of the language server connection.
pub struct Server<'a> {
    receiver: Receiver<Message>,
    state: &'a mut State,
}

impl<'a> Server<'a> {
    /// Constructs a new `Server` with the given receiver channel and state.
    pub fn new(receiver: Receiver<Message>, state: &'a mut State) -> Self {
        Self { receiver, state }
    }

    /// Constructs a new [`InitializedServer`] that establishes a successful
    /// connection between the server and client. This method will return an
    /// error if it doesn't receive an `initialize` request from the client or
    /// the server fails to send an `initialized` request.
    pub fn initialize(self) -> Result<InitializedServer<'a>> {
        match self.receiver.recv()? {
            Message::Request(request) if request.is_initialize() => {
                use lsp_types::request::Initialize;
                RequestDispatcher::new(request, self.state)
                    .on::<Initialize>(handlers::initialize)?
                    .finish();
            }
            message => {
                let message = format!(
                    "expected initialize request, but found {:?}",
                    message
                );
                return Err(ProtocolError(message).into());
            }
        }

        Ok(InitializedServer {
            receiver: self.receiver,
            state: self.state,
        })
    }
}

/// The initialized server side of the language server connection.
pub struct InitializedServer<'a> {
    receiver: Receiver<Message>,
    state: &'a mut State,
}

impl<'a> InitializedServer<'a> {
    /// Starts the main loop of the server.
    pub fn run(mut self) -> Result<()> {
        while let Ok(message) = self.receiver.recv() {
            match message {
                Message::Request(r) => self.handle_request(r)?,
                Message::Notification(n) if n.is_exit() => {
                    log::trace!("Exiting...");
                    break;
                }
                Message::Notification(n) => self.handle_notification(n)?,
                _ => log::info!("Unhandled message: {:?}", message),
            }
        }

        Ok(())
    }

    fn handle_request(&mut self, req: Request) -> Result<()> {
        use lsp_types::request::*;
        RequestDispatcher::new(req, self.state)
            // .on::<Initialize>(handlers::initialize)?
            .on::<Shutdown>(handlers::shutdown)?
            .on::<Completion>(handlers::completion)?
            .on::<HoverRequest>(handlers::hover)?
            .finish();

        Ok(())
    }

    fn handle_notification(&mut self, not: Notification) -> Result<()> {
        use lsp_types::notification::*;
        NotificationDispatcher::new(not, self.state)
            .on::<Initialized>(handlers::initialized)
            .on::<DidOpenTextDocument>(handlers::did_open_text_document)
            .on::<DidChangeTextDocument>(handlers::did_change_text_document)
            .on::<DidSaveTextDocument>(handlers::did_save_text_document)
            .on::<DidChangeConfiguration>(handlers::did_change_configuration)
            .finish();

        Ok(())
    }
}

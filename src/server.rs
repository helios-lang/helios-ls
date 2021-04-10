//! The orchestrator of the Helios language server responsible for handling
//! requests and notifications sent between the client and server.

mod dispatcher;
mod handlers;

use self::dispatcher::{NotificationDispatcher, RequestDispatcher};
use crate::protocol::{ErrorCode, Message, Notification, Request, Response};
use crate::state::State;
use crate::Result;
use flume::Receiver;

/// The server side of the language server connection.
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
    /// connection between the server and client.
    ///
    /// # Error handling
    ///
    /// As per the [Language Server Protocol Specification], this method will
    /// send an error response in the case that the server receives some message
    /// other than an `initialize` request. Specifically, it will send an error
    /// with `code: -32002`, which the client can handle appropriately. In the
    /// meantime, this method will continue listening for the `initialize`
    /// request.
    ///
    /// Notifications, on the other hand, will be ignored completely. A warning
    /// message may be printed notifying the client of this.
    ///
    /// [Language Server Protocol Specification]: https://microsoft.github.io/language-server-protocol/specifications/specification-current/#initialize
    pub fn initialize(self) -> Result<InitializedServer<'a>> {
        while let Ok(message) = self.receiver.recv() {
            match message {
                Message::Request(req) if req.is_initialize() => {
                    // Building the initialize response
                    let snapshot = self.state.snapshot();
                    let params = serde_json::from_value(req.params)?;
                    let result = handlers::initialize(snapshot, params)?;
                    let response = Response::new_ok(req.id, result);

                    // Send the initialize response
                    self.state.send(response);
                }
                Message::Notification(not) if not.is_initialized() => {
                    let params = serde_json::from_value(not.params)?;
                    handlers::initialized(self.state, params);
                    // The server has been initialized so we can exit the loop
                    break;
                }
                Message::Request(req) => {
                    let code = ErrorCode::ServerNotInitialized;
                    let message = format!(
                        "expected an `initialize` request but received `{}`",
                        req.method
                    );
                    let response = Response::new_error(req.id, code, message);
                    self.state.send(response);
                }
                _ => {
                    log::warn!(
                        "Helios-LS expected an `initialize` request but \
                         instead received the following message: {:?}. This \
                         message will be ignored.",
                        message
                    );
                }
            }
        }

        Ok(InitializedServer {
            receiver: self.receiver,
            state: self.state,
        })
    }
}

/// An initialized server that guarantees that the connection between the client
/// and server has been successfully initialized.
pub struct InitializedServer<'a> {
    receiver: Receiver<Message>,
    state: &'a mut State,
}

impl<'a> InitializedServer<'a> {
    /// Starts the main loop of the server.
    pub fn run(mut self) -> Result<()> {
        while let Ok(message) = self.receiver.recv() {
            match message {
                Message::Request(req) => self.handle_request(req)?,
                Message::Notification(not) if not.is_exit() => break,
                Message::Notification(not) => self.handle_notification(not)?,
                _ => log::warn!("Unhandled message: {:?}", message),
            }
        }

        Ok(())
    }

    fn handle_request(&mut self, req: Request) -> Result<()> {
        use lsp_types::request::*;
        RequestDispatcher::new(req, self.state)
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

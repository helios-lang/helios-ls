//! The Helios Language Server is an intermediary between an editor and the
//! Helios compiler. It implements the [Language Server Protocol], which allows
//! us to provide Helios with common editor functionality such as autocomplete,
//! go-to definitions and find-all-references in one centralised location.
//!
//! [Language Server Protocol]: https://microsoft.github.io/language-server-protocol

pub mod connection;
pub mod protocol;
pub mod server;
pub mod state;

use server::Server;
use state::State;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Responsible for the server's main loop logic.
fn __start() -> Result<()> {
    let (connection, threads) = connection::stdio();

    let mut state = State::new(connection.sender);
    Server::new(connection.receiver, &mut state)
        .initialize()?
        .run()?;

    threads.join()?;
    log::info!("Connection to client has closed");

    Ok(())
}

/// Establishes a connection to the client and starts the server.
pub fn start() {
    if let Err(error) = __start() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0")]
struct HeliosLsOpts {}

fn main() {
    env_logger::init();
    let _ = HeliosLsOpts::parse();

    log::trace!("Starting Helios-LS...");
    helios_ls::start();
}

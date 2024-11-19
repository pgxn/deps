use pgxn_deps::{error::Result, operating_system::OperatingSystem, resolve_package};

use argh::FromArgs;

#[derive(FromArgs)]
/// Obtain the installation command for a given purl package, according to PGXNv2 specs
struct Command {
    /// operating system to install the package on
    #[argh(option)]
    os: Option<OperatingSystem>,

    /// a purl string
    #[argh(positional)]
    purl: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let command: Command = argh::from_env();

    let operating_system = match command.os {
        Some(os) => os,
        None => OperatingSystem::detect()?,
    };

    let installation_command = resolve_package(&command.purl, operating_system).await?;

    println!("{installation_command}");

    Ok(())
}

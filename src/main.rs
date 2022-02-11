// std lib
use std::io::{stdin, BufRead};
use std::process;

// 3rd-party
use clap::Parser;
use simple_steam_totp;


#[derive(Debug, Parser)]
/// A small utility meant to receive Steam Guard's base64 encoded "Shared Secret"(s),
/// and print the corresponding Steam 2FA TOTP(s) -
/// can receive inputs as variadics arguments or passed through stdin.
///
/// This tool is not intended to do anything else than translate Steam 2FA seeds -> Steam 2FA TOTP
/// codes.
///
/// See below link for how to extract your Steam Guard's "Shared Secret":
/// https://github.com/SteamTimeIdler/stidler/wiki/Getting-your-%27shared_secret%27-code-for-use-with-Auto-Restarter-on-Mobile-Authentication
struct Cli {
    /// One or more (space separated) Steam Guard's Base64 encoded Shared Secret(s)
    /// (read: seed(s) for Steam 2FA TOTP)
    shared_secrets: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get args passed via cli
    let cli = Cli::parse();
    let mut shared_secrets = cli.shared_secrets;

    // Read inputs from stdin
    if !atty::is(atty::Stream::Stdin) {
        shared_secrets.extend(stdin().lock().lines().map(|l| l.unwrap()).collect::<Vec<String>>())
    }

    if shared_secrets.is_empty() {
        eprintln!("A base64 encoded string seed (or multiple) is required! Check usage through --help.");
        process::exit(1);
    }

    for seed in shared_secrets {
        let code = match simple_steam_totp::generate(&seed) {
            Ok(code) => code,
            Err(e) => panic!("{}", e),
        };
        println!("{}", code);
    }

    Ok(())
}

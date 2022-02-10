use std::process;

use anyhow;
use simple_steam_totp;

fn main() -> anyhow::Result<()> {
    // Use example shared_secret given by simple_steam_totp library's README.md
    let totp_code: String = match simple_steam_totp::generate("V59i4SUqNiuYDrssYyMz62RSI9k=") {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    println!("Hello, world!\n\t{}", totp_code);
    Ok(())
}

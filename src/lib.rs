pub mod config;
pub mod meow_file;

use config::{Args, Config};

pub fn lolcat() -> Result<(), String> {
    let args = Args::parse_args();
    let c = Config::new(args)?;

    c.read_all_files()?;
    Ok(())
}

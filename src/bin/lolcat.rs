use lolcat::config::{Args, Config};

fn main() -> Result<(), String> {
    let args = Args::parse_args();
    let c = Config::new(args)?;

    c.read_all_files()?;
    Ok(())
}

use anyhow::Result;
use clap::Clap;
use rgrep::*;

fn main() -> Result<()> {
  let config: GrepConfig = GrepConfig::parse();
  println!("cfg: {:?}", config);
  config.match_with_default_strategy()?;

  Ok(())
}

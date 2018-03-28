extern crate gitio;
#[macro_use]
extern crate structopt;
extern crate url;
extern crate failure;

use structopt::StructOpt;
use url::Url;

use std::io::{self, Read};
use std::str::FromStr;

type Result<T> = std::result::Result<T, failure::Error>;

fn main() -> Result<()> {
  let cli = Cli::from_args();

  let url = match cli.url {
    Some(u) => u,
    None => stdin_url()?
  };

  let location = gitio::shorten(&url, cli.key)?;

  println!("{}", location);

  Ok(())
}

#[derive(StructOpt)]
struct Cli {
  #[structopt(
    short = "k",
    long = "key",
    help = "key to attempt to use instead of a randomly-generated one",
  )]
  key: Option<String>,
  #[structopt(
    help = "the url to shorten",
    parse(try_from_str),
  )]
  url: Option<Url>,
}

fn stdin_url() -> Result<Url> {
  let mut content = String::new();
  io::stdin().read_to_string(&mut content)?;
  let url = Url::from_str(&content)?;
  Ok(url)
}

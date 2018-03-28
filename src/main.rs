#[macro_use]
extern crate structopt;
extern crate reqwest;
extern crate url;
#[macro_use]
extern crate failure;

use reqwest::Client;
use reqwest::header::Location;
use structopt::StructOpt;
use url::Url;

use std::io::{self, Read};
use std::str::FromStr;

type Result<T> = std::result::Result<T, failure::Error>;

fn main() -> Result<()> {
  let cli = Cli::from_args();
  let client = Client::new();
  let mut builder = client.post("https://git.io");
  let mut options = Vec::with_capacity(2);
  let url = match cli.url {
    Some(u) => u,
    None => stdin_url()?
  };
  options.push(("url", url.into_string()));
  if let Some(key) = cli.key {
    options.push(("code", key));
  }
  builder.form(&options);
  let resp = builder.send()?;

  let location = match resp.headers().get::<Location>() {
    Some(l) => l,
    None => bail!("git.io had no Location header in its response")
  };

  println!("{}", location);

  Ok(())
}

#[derive(StructOpt)]
struct Cli {
  #[structopt(
    short = "k",
    long = "key",
    help = "key to attempt to use instead of a randomly-generated one"
  )]
  key: Option<String>,
  #[structopt(
    help = "the url to shorten",
    parse(try_from_str)
  )]
  url: Option<Url>
}

fn stdin_url() -> Result<Url> {
  let mut content = String::new();
  io::stdin().read_to_string(&mut content)?;
  let url = Url::from_str(&content)?;
  Ok(url)
}

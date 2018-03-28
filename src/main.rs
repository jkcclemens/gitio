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

fn main() -> Result<(), failure::Error> {
  let cli = Cli::from_args();
  let client = Client::new();
  let mut builder = client.post("https://git.io");
  let mut options = Vec::with_capacity(2);
  options.push(("url", cli.url.into_string()));
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
  url: Url
}

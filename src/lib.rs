extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate url;

use reqwest::Client;
use reqwest::header::Location;
use url::Url;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error>;

lazy_static! {
  static ref CLIENT: Client = Client::new();
}

pub fn shorten<S: AsRef<str>>(url: &Url, key: Option<S>) -> Result<Url> {
  let mut builder = CLIENT.post("https://git.io");

  let mut options = Vec::with_capacity(2);
  options.push(("url", url.as_str().to_string()));
  if let Some(key) = key {
    options.push(("code", key.as_ref().to_string()));
  }

  builder.form(&options);

  let mut resp = builder.send().map_err(Error::Reqwest)?;
  let text = resp.text().map_err(Error::Reqwest)?;

  let location = match resp.headers().get::<Location>() {
    Some(l) => l,
    None => return Err(Error::GitHub(text))
  };

  Ok(Url::from_str(&*location).map_err(Error::Url)?)
}

#[derive(Debug, Fail)]
pub enum Error {
  Reqwest(reqwest::Error),
  Url(url::ParseError),
  GitHub(String),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match *self {
      Error::Reqwest(ref e) => write!(f, "{}", e),
      Error::Url(ref e) => write!(f, "{}", e),
      Error::GitHub(ref e) => write!(f, "{}", e),
    }
  }
}

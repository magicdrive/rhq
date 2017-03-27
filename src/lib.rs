//!
//! rhq is an alternative of `ghq`
//!

extern crate regex;
extern crate shellexpand;
extern crate shlex;
extern crate toml;
extern crate url;
extern crate walkdir;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

error_chain!{
  foreign_links {
    Io(::std::io::Error);
    TomlDe(::toml::de::Error);
    ShellExpand(::shellexpand::LookupError<::std::env::VarError>);
    UrlParse(::url::ParseError);
  }
}

pub mod app;
pub mod core;
pub mod util;
pub mod vcs;

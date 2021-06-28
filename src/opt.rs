use crate::common::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "zk")]
pub enum Opt {
  #[structopt(name = "new")]
  /// Create a new Zettelkasten note
  New { name: String },

  #[structopt(name = "open")]
  /// Open an existing Zettelkasten note
  Open { name: String },

  #[structopt(name = "link")]
  /// Link two existing Zettelkasten notes
  Link { left: String, right: String },

  #[structopt(name = "find")]
  /// Find Zettelkasten notes by tag
  Find { tag: String },

  #[structopt(name = "search")]
  /// Fuzzy search Zettelkasten notes
  Search,
}

impl Opt {
  pub fn run(self) -> Result<(), Error> {
    let handler = Handler {
      config: Config::load()?,
    };

    match self {
      Opt::New { name } => handler.new(&name)?,
      Opt::Open { name } => handler.open(&name)?,
      Opt::Link { left, right } => handler.link(&left, &right)?,
      Opt::Find { tag } => handler.find(&tag)?,
      Opt::Search => handler.search()?,
    }

    Ok(())
  }
}

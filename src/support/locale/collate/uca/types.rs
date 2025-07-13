use hashbrown::HashMap;

pub type SinglesTable = HashMap<u32, Box<[u32]>>;
pub type MultisTable = HashMap<u64, Box<[u32]>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Tailoring {
  Cldr(Locale),
  Ducet
}

impl Default for Tailoring {
  fn default() -> Self {
    Self::Cldr(Locale::default())
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub enum Locale {
  ArabicScript,
  ArabicInterleaved,
  #[default]
  Root
}

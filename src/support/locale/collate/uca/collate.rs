use {
  super::{
    Tailoring,
    ascii::fill_and_check,
    cea::generate_cea,
    first_weight::try_initial,
    normalize::make_nfd,
    prefix::find_prefix,
    sort_key::compare_incremental
  },
  bstr::{B, ByteSlice},
  core::cmp::Ordering
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Collator {
  pub tailoring: Tailoring,
  pub shifting: bool,
  pub tiebreak: bool,
  a_chars: Vec<u32>,
  b_chars: Vec<u32>,
  a_cea: Vec<u32>,
  b_cea: Vec<u32>
}

impl Default for Collator {
  fn default() -> Self {
    Self::new(Tailoring::default(), true, true)
  }
}

impl Collator {
  pub fn new(
    tailoring: Tailoring,
    shifting: bool,
    tiebreak: bool
  ) -> Self {
    Self {
      tailoring,
      shifting,
      tiebreak,
      a_chars: Vec::new(),
      b_chars: Vec::new(),
      a_cea: vec![0; 64],
      b_cea: vec![0; 64]
    }
  }

  pub fn collate_u8(
    &mut self,
    a: &[u8],
    b: &[u8]
  ) -> Ordering {
    if a == b {
      return Ordering::Equal;
    }

    let mut a_iter = B(a).chars().map(|c| c as u32);
    let mut b_iter = B(b).chars().map(|c| c as u32);

    self.collate_inner(&mut a_iter, &mut b_iter)
  }

  pub fn collate_u32(
    &mut self,
    a: &[u32],
    b: &[u32]
  ) -> Ordering {
    if a == b {
      return Ordering::Equal;
    }

    let mut a_iter = a.iter().cloned();
    let mut b_iter = b.iter().cloned();

    self.collate_inner(&mut a_iter, &mut b_iter)
  }

  fn collate_inner(
    &mut self,
    a_iter: &mut impl Iterator<Item = u32>,
    b_iter: &mut impl Iterator<Item = u32>
  ) -> Ordering {
    self.a_chars.clear();
    self.b_chars.clear();

    if let Some(o) =
      fill_and_check(a_iter, b_iter, &mut self.a_chars, &mut self.b_chars)
    {
      return o;
    }

    make_nfd(&mut self.a_chars);
    make_nfd(&mut self.b_chars);

    if self.a_chars == self.b_chars {
      if self.tiebreak {
        let a = a_iter.next().unwrap();
        let b = b_iter.next().unwrap();

        return a.cmp(&b);
      }
      return Ordering::Equal;
    }

    let offset = find_prefix(&self.a_chars, &self.b_chars, self.shifting);

    if self.a_chars[offset..].is_empty() || self.b_chars[offset..].is_empty() {
      return self.a_chars.len().cmp(&self.b_chars.len());
    }
    if let Some(o) =
      try_initial(self, &self.a_chars[offset..], &self.b_chars[offset..])
    {
      return o;
    }

    generate_cea(
      &mut self.a_cea,
      &mut self.a_chars,
      self.shifting,
      self.tailoring,
      offset
    );
    generate_cea(
      &mut self.b_cea,
      &mut self.b_chars,
      self.shifting,
      self.tailoring,
      offset
    );

    let comparison =
      compare_incremental(&self.a_cea, &self.b_cea, self.shifting);
    if comparison == Ordering::Equal && self.tiebreak {
      let a = a_iter.next().unwrap();
      let b = b_iter.next().unwrap();

      return a.cmp(&b);
    }

    comparison
  }
}

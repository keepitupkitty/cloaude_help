use {
  super::{Tailoring, ascii::fill, cea::generate_cea, normalize::make_nfd},
  crate::allocation::{vec, vec::Vec},
  allocation::boxed,
  bstr::{B, ByteSlice}
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct SortKey {
  pub tailoring: Tailoring,
  pub shifting: bool,
  pub tiebreak: bool,
  s_chars: Vec<u32>,
  s_cea: Vec<u32>,
  prev_cea: Vec<u32>
}

impl Default for SortKey {
  fn default() -> Self {
    Self::new(Tailoring::default(), true, true)
  }
}

impl SortKey {
  pub fn new(
    tailoring: Tailoring,
    shifting: bool,
    tiebreak: bool
  ) -> Self {
    Self {
      tailoring,
      shifting,
      tiebreak,
      s_chars: Vec::new(),
      s_cea: vec![0; 64],
      prev_cea: vec![0; 64]
    }
  }

  pub fn get_sortkey_u8(
    &mut self,
    s: &[u8]
  ) -> &[u8] {
    let mut s_iter = B(s).chars().map(|c| c as u32);

    let cea = self.get_sortkey_inner(&mut s_iter);
    let u8_cea: Vec<u8> = cea.iter().map(|x| *x as u8).collect();

    boxed::Box::leak(u8_cea.into_boxed_slice())
  }

  pub fn get_sortkey_u32(
    &mut self,
    s: &[u32]
  ) -> &[u32] {
    let mut s_iter = s.iter().cloned();

    self.get_sortkey_inner(&mut s_iter)
  }

  fn get_sortkey_inner(
    &mut self,
    s_iter: &mut impl Iterator<Item = u32>
  ) -> &[u32] {
    self.s_chars.clear();

    fill(s_iter, &mut self.s_chars);
    make_nfd(&mut self.s_chars);
    generate_cea(
      &mut self.s_cea,
      &mut self.s_chars,
      self.shifting,
      self.tailoring,
      0
    );

    if let Some(idx) = self.s_cea.iter().rposition(|&x| x != 0) {
      self.s_cea.truncate(idx + 1);
    } else {
      self.s_cea.clear();
    }

    &self.s_cea
  }
}

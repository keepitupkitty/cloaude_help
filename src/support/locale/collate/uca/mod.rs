use {
  super::LCCollate,
  crate::{
    c_char,
    c_int,
    size_t,
    std::{string, wchar},
    wchar_t
  },
  core::{cmp::Ordering, slice}
};

mod types;
pub use types::{Locale, Tailoring};

mod ascii;
mod cea;
mod cea_utils;
mod consts;
mod first_weight;
mod normalize;
mod prefix;
mod sort_key;
mod tailor;
mod weights;

mod collate;
pub use collate::Collator;
mod xfrm;
pub use xfrm::SortKey;

fn strcoll(
  lhs: *const c_char,
  rhs: *const c_char
) -> c_int {
  let lhs: &[u8] =
    unsafe { slice::from_raw_parts(lhs as *const u8, string::rs_strlen(lhs)) };
  let rhs: &[u8] =
    unsafe { slice::from_raw_parts(rhs as *const u8, string::rs_strlen(rhs)) };

  let mut c = Collator::default();
  match c.collate_u8(lhs, rhs) {
    | Ordering::Less => return -1,
    | Ordering::Equal => return 0,
    | Ordering::Greater => return 1
  };
}

pub fn strxfrm(
  dest: *mut c_char,
  src: *const c_char,
  dlen: size_t
) -> size_t {
  let slen = string::rs_strlen(src);
  if dlen >= slen && slen != 0 {
    let source: &[u8] =
      unsafe { slice::from_raw_parts(src as *const u8, slen) };
    let destination: &mut [u8] =
      unsafe { slice::from_raw_parts_mut(dest as *mut u8, dlen) };

    let mut x = SortKey::default();
    let sk = x.get_sortkey_u8(source);

    for (i, &val) in sk.iter().enumerate().take(dlen) {
      destination[i] = val;
    }
  }
  slen
}

fn wcscoll(
  lhs: *const wchar_t,
  rhs: *const wchar_t
) -> c_int {
  let lhs: &[u32] =
    unsafe { slice::from_raw_parts(lhs as *const u32, wchar::rs_wcslen(lhs)) };
  let rhs: &[u32] =
    unsafe { slice::from_raw_parts(rhs as *const u32, wchar::rs_wcslen(rhs)) };

  let mut c = Collator::default();
  match c.collate_u32(lhs, rhs) {
    | Ordering::Less => return -1,
    | Ordering::Equal => return 0,
    | Ordering::Greater => return 1
  };
}

fn wcsxfrm(
  dest: *mut wchar_t,
  src: *const wchar_t,
  dlen: size_t
) -> size_t {
  let slen = wchar::rs_wcslen(src);
  println!("wcsxfrm_debug: slen={}, dlen={}", slen, dlen);

  if slen == 0 {
    if dlen > 0 {
      unsafe { *dest = 0 };
    }
    return 1;
  }

  let source: &[u32] =
    unsafe { slice::from_raw_parts(src as *const u32, slen) };

  println!("wcsxfrm_debug: source = {:?}", source);

  let mut x = SortKey::default();
  let sk = x.get_sortkey_u32(source);

  println!("wcsxfrm_debug: sort key = {:?}", sk);

  let required_size = sk.len() + 1;

  if dlen > 0 {
    let destination: &mut [u32] =
      unsafe { slice::from_raw_parts_mut(dest as *mut u32, dlen) };

    let copy_len = core::cmp::min(sk.len(), dlen.saturating_sub(1));

    for i in 0..copy_len {
      destination[i] = sk[i];
    }

    if copy_len < dlen {
      destination[copy_len] = 0;
    }

    println!(
      "wcsxfrm_debug: destination = {:?}",
      &destination[..core::cmp::min(copy_len + 1, dlen)]
    );
  }

  required_size
}

pub const UCA_COLLATE: LCCollate = LCCollate {
  name: c"".as_ptr(),
  strcoll: strcoll,
  strxfrm: strxfrm,
  wcscoll: wcscoll,
  wcsxfrm: wcsxfrm
};

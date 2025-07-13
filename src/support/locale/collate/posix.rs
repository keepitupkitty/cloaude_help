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

fn strcoll(
  lhs: *const c_char,
  rhs: *const c_char
) -> c_int {
  let lhs: &[u8] =
    unsafe { slice::from_raw_parts(lhs as *const u8, string::rs_strlen(lhs)) };
  let rhs: &[u8] =
    unsafe { slice::from_raw_parts(rhs as *const u8, string::rs_strlen(rhs)) };

  match lhs.cmp(rhs) {
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

    for (i, &val) in source.iter().enumerate().take(dlen) {
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

  match lhs.cmp(rhs) {
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
  if dlen >= slen && slen != 0 {
    let source: &[u32] =
      unsafe { slice::from_raw_parts(src as *const u32, slen) };
    let destination: &mut [u32] =
      unsafe { slice::from_raw_parts_mut(dest as *mut u32, dlen) };

    for (i, &val) in source.iter().enumerate().take(dlen) {
      destination[i] = val;
    }
  }
  slen
}

pub const POSIX_COLLATE: LCCollate = LCCollate {
  name: c"".as_ptr(),
  strcoll: strcoll,
  strxfrm: strxfrm,
  wcscoll: wcscoll,
  wcsxfrm: wcsxfrm
};

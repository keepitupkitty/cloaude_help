use super::consts::{NEED_THREE, NEED_TWO, VARIABLE};

pub fn find_prefix(
  a: &[u32],
  b: &[u32],
  shifting: bool
) -> usize {
  let prefix_len = a
    .iter()
    .zip(b.iter())
    .take_while(|(x, y)| {
      x == y && !NEED_TWO.contains(x) && !NEED_THREE.contains(x)
    })
    .count();

  if prefix_len > 0 {
    if shifting && VARIABLE.contains(&a[prefix_len - 1]) {
      if prefix_len > 1 {
        if VARIABLE.contains(&a[prefix_len - 2]) {
          return 0;
        }

        return prefix_len - 1;
      }

      return 0;
    }

    return prefix_len;
  }

  0
}

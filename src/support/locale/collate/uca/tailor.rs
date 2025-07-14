use {
  super::{
    consts::{BINCODE_CONF, MULT_CLDR_DATA, SING_CLDR_DATA},
    types::{MultisTable, SinglesTable}
  },
  bincode::serde::decode_from_slice,
  once_cell::sync::Lazy
};

const SING_AR_DATA: &[u8; 13_588] =
  include_bytes!("bincode/tailoring/arabic_script_sing");
pub static SING_AR: Lazy<SinglesTable> = Lazy::new(|| {
  let mut sing: SinglesTable =
    decode_from_slice(SING_CLDR_DATA, BINCODE_CONF).unwrap().0;
  let extension: SinglesTable =
    decode_from_slice(SING_AR_DATA, BINCODE_CONF).unwrap().0;

  sing.extend(extension);
  sing
});

const MULT_AR_DATA: &[u8; 56] =
  include_bytes!("bincode/tailoring/arabic_script_multi");
pub static MULT_AR: Lazy<MultisTable> = Lazy::new(|| {
  let mut mult: MultisTable =
    decode_from_slice(MULT_CLDR_DATA, BINCODE_CONF).unwrap().0;
  let extension: MultisTable =
    decode_from_slice(MULT_AR_DATA, BINCODE_CONF).unwrap().0;

  mult.extend(extension);
  mult
});

const SING_AR_I_DATA: &[u8; 10_157] =
  include_bytes!("bincode/tailoring/arabic_interleaved_sing");
pub static SING_AR_I: Lazy<SinglesTable> = Lazy::new(|| {
  let mut sing: SinglesTable =
    decode_from_slice(SING_CLDR_DATA, BINCODE_CONF).unwrap().0;
  let extension: SinglesTable =
    decode_from_slice(SING_AR_I_DATA, BINCODE_CONF).unwrap().0;

  sing.extend(extension);
  sing
});

const MULT_AR_I_DATA: &[u8; 34] =
  include_bytes!("bincode/tailoring/arabic_interleaved_multi");
pub static MULT_AR_I: Lazy<MultisTable> = Lazy::new(|| {
  let mut mult: MultisTable =
    decode_from_slice(MULT_CLDR_DATA, BINCODE_CONF).unwrap().0;
  let extension: MultisTable =
    decode_from_slice(MULT_AR_I_DATA, BINCODE_CONF).unwrap().0;

  mult.extend(extension);
  mult
});

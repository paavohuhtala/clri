
// Ported from http://stackoverflow.com/a/3407254
pub fn to_next_multiple(x: u64, multiple: u64) -> u64 {
  if multiple == 0 {
    return x
  }

  let remainder = x % multiple;
  if remainder == 0 {
    return x
  }

  x + multiple - remainder
}

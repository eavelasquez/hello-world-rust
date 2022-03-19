fn main() {
  let decimal = 65.4321_f32;

  // Error! No implicit conversion
  let integer: u8 = decimal;
  // FIXME ^ Comment out this line

  // Explicit conversion
  let integer = decimal as u8;
  let character = integer as char;

  // Error! There are limitations in conversion rules.
  // A float cannot be directly conversion to a char.
  let character = decimal as char;
  // FIXME ^ Comment out this line

  println!("Casting: {} -> {} -> {}", decimal, integer, character);

  // when casting any value to an unsigned type, T,
  // T::Max + 1 is added or subtracted until the value
  // fits into the new type

  // 1000 already fits in a u16
  println!("1000 as a u16 is: {}", 1000 as u16);

  // 1000 - 256 - 256 - 256 = 232
  // Under the hood, the first 8 least significant bits (LSB) are kept,
  // while the rest towards the most significant bit (MSB) get truncated.
  println!("1000 as a u8 is: {}", 1000 as u8);
  
}
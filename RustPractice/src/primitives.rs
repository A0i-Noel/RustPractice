fn main() {
  // boolean
  let logical : bool = true;

  // number]
  let a_float: f64 = 1.0; // Regular annotation
  let an_integer = 5i32; // Suffix annotation

  // default for float is f64
  // default for integer is i32

  // ====upper code is immutable code (cannot change value)======

  let mut inffered_type = 12;
  inffered_type = 123456789;

  let mut mutable = 12;
  mutable = 21;

  // mutable = true; // error cause because data type changed from intger to boolean

  let mutable = true; 
  // it does not cause error. variables can be overwritten with shadowing
}
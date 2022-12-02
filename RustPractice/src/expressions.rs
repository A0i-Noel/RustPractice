fn main() {
  let x = 5u32;

  let y = {
    let x_squared = x * x;
    let x_cube = x_squared * x;

    x_cube + x_squared + x
  };

  let z = {
    2 * x;
  };

  println!("x is {:?}",x);
  println!("y is {:?}",y);
  println!("z is {:?}",z);




  let n = 5;

  if n < 0 {
    print!("{} is negative", n);
  } else if n > 0 {
    print!("{} is positive", n);
  } else {
    print!("{} is zero", n);
  }
}
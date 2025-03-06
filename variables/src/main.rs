fn main() {


  let mut x: u64 = 0;
  loop {

    x = x+1;
    println!("The value of x is: {x}");

    if x >= 1000 { break; };

  };
  println!("The value of x is: {x}");
}

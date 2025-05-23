//comment

fn main() {
  //1.
  //ln add newline
  print!("Hello world!\n");
  println!("I'm a Rustacean!");

  //1.1
  //can block comment inside definition
  let x = [5 + /*6 +*/ 5, 5 + 6 + 5];
  println!("x_0 is {} and x_1 = {}", x[0],x[1]);
}

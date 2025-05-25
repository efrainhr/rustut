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

  //zero indexing
  println!("{0}, this is {1}. {1}, this is {0}", "Sponge", "Bob");

  //notice that these are out of order
  println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

  println!("Base 10:               {}",   69420); // 69420
  println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
  println!("Base 8 (octal):        {:o}", 69420); // 207454
  println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

  //1st has a 1 w/ 4 padding
  //2nd has 4 padding w/ a 1
  println!("{number:<5}", number=1);
  println!("{number:>5}", number=1);
  println!("{number:0<5}{number:0>5}", number=1);
  println!("{number}{number:0>9}", number=1);
  println!("{number}{number}", number=1);

  println!("{number:0>width$}", number=1, width=5);

  println!("My name is {0}, {1} {0}", "Bond", "James");
  // FIXME ^ Add the missing argument: "James"

  //#[allow(dead_code)]
  //struct Structure(i32);


  // This will not compile because `Structure` does not implement
  // fmt::Display.
  //println!("This struct `{}` won't print...", Structure(3));
  // TODO ^ Try uncommenting this line

  let pi = 3.141592;
  print!("pi is approx {pi:.0}\n");
  println!("pi is approx {pi:.3}");
}

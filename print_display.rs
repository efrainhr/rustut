use std::fmt::{self, Formatter, Display}; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Bin2D {
    x: i64,
    y: i64,
}

impl fmt::Binary for Bin2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {:b}, y: {:b}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //extract using tuple indexing
    //and create reference to `vec`
    let vec = &self.0;

    //open `vec` braket
    write!(f, "[")?;

    //iterate over `vec` while enumerating the iteration
    //`count`
    for (count, v) in vec.iter().enumerate() {
      //add comma-space for each element except 0
      if count != 0 {
        write!(f, ", ")?;
      }
      write!(f, "{1}: {0}", v, count)?;

    }

    //close `vec` braket
    write!(f, "]")
  }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32
}

impl Display for City {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      let latc = if self.lat >= 0.0 {'N'} else {'S'};
      let lonc = if self.lat >= 0.0 {'E'} else {'W'};

      write!(f, "{}: {:.3}deg{} {:.3}deg{}",
             self.name, self.lat.abs(), latc, self.lon.abs(), lonc)
  }
}

//#[derive(Debug)]
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

impl Display for Color {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    //needed to change RGB to u32 for calc (no mix type operations?)
    //I guess I can use `as type` ...
    let col = (self.red as u32*65536)+(self.green as u16*256) as u32 +self.blue as u32;

    //tutorial suggests using :0>2 for padding, but needed to needed
    //to use #08
    write!(f, "RGB ({}, {}, {}) {:#08X}",self.red, self.green, self.blue, col)
  }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };
    let binpoint = Bin2D { x: 3, y: 2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("Binary: {:b}", binpoint);
    println!("Binary: {:?}", binpoint);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);

    let cplx = Complex {real: 3.3, imag: 7.2};
    println!("Display: {}",cplx);
    println!("Debug: {:?}",cplx);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 } ] {
            println!( "{}", city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 }
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        //println!("{:?}", color);
        println!("{}", color);
    }

}

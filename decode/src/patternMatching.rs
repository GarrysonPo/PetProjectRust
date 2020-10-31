fn main() {
  let number = 15;

  match number {
    1 => println!("It is one!"),
    2 ..= 20 => println!("It is greate than one!"),
    // 2 => println!("There is two of them"),
    // 10 | 11 => println!("It is either 10 or 11"),
    _ => println!("It doesn't match!")
  }
}
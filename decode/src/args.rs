use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  println!("{}", args[1]);
  // for argument in args.iter() {
  //   println!("{}", argument);
  // }
}
fn main() {
  demo1();
  demo2();
}

fn demo1() {
  // can change
  let mut _a: i32 = 1111;
  _a = 12334;
  // cannot change
  const A: i32 = 123;
  //  can Shadowing
  let _b:i64 = 123213;
  let _b = 1;

  println!("value:{}", _a);
  println!("value2:{}", A);
  println!("value3:{}", _b);
}

fn demo2() {
  let x = 5;
  let x = x + 1;
  let x = x * 2;

  println!("The value of x is: {}", x);
}

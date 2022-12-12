fn _mutable_concern() {
  let x = 1;
  println!("x = {}", x);

  let mut y = 2;
  y = y + 1;
  println!("y = {}", y);
}

fn _copy_semantics() {
  let s = 1;
  let t = s;
  println!("{}", t);
  println!("{}", s);
}

fn _move_semantics1() {
  let s = "Hello".to_string();
  let t = s;
  println!("{}", t);
  // ERR println!("{}", s);
}

#[warn(dead_code)]
fn myprint<T: std::fmt::Display>(msg: T) {
  println!("{}", msg);
}

fn _move_semantics2() {
  let s = "Hello".to_string();
  myprint(s);
  // ERR myprint(s);
}

fn _move_semantics3() {
  let s = "Hello".to_string();
  let ss = s.clone();
  myprint(s);
  myprint(ss);
}

#[warn(dead_code)]
fn myprint_byreference<T: std::fmt::Display>(msg: &T) {
  println!("{}", *msg); // dereference
}

fn _reference() {
  let s = "Hello".to_string();
  myprint_byreference(&s);
  myprint_byreference(&s);
}

fn _immutable_reference() {
  let s = "hello".to_string();
  let s_ref = &s;
  myprint_byreference(s_ref);
  myprint_byreference(s_ref);
}

fn main() {
  let mut s = "hello".to_string();
  println!("s = {}", s);
  s.clear();
  println!("s = {}", s);
}










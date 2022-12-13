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

fn _mutable_reference1() {
  let mut s = "hello".to_string();
  println!("s = {}", s);
  s.clear();
  println!("s = {}", s);
}

#[warn(dead_code)]
fn myclear(x: &mut String) {
  x.clear();
}

fn _mutable_mutablereference() {
  let mut s = "hello".to_string();
  println!("s = {}", s);

  let s_ref = &mut s; // 元がmutableでないとmutableな参照がとれない
  myclear(s_ref); // 関数内で変更が起きる
  println!("s = {}", s);
}

#[warn(dead_code)]
fn pick1(x: &[i32], end: usize) -> &[i32] {
  &x[..end]
}

fn _retref1() {
  let v1 = [1, 2, 3, 4, 5];
  let p = pick1(&v1, 2);
  for ss in p {
    println!("{}", ss);
  };
}

#[warn(dead_code)]
fn pick2<'a, 'b>(x: &'a [i32], y: &'b [i32], end: usize) -> (&'a [i32], &'b [i32]) {
  (&x[..end], &y[..end])
}

fn _checkretref() {
  let v1 = [1, 2, 3, 4, 5];
  let v2 = [6, 7, 8];

  let p = pick2(&v1, &v2, 2);
  for ss in p.0 {
    println!("{}", ss);
  };
  for ss in p.1 {
    println!("{}", ss);
  }
}

enum EnumExample {
  TupleTypeExample1(String),
  TupleTypeExample2(i32, bool),
  StructTypeExample{name: String, age: u8}
}

fn main() {
  let mut v: Vec<EnumExample> = Vec::new();
  v.push(EnumExample::TupleTypeExample1(String::from("Hello"))); 
  v.push(EnumExample::TupleTypeExample2(10, true));
  v.push(EnumExample::StructTypeExample{
    name: String::from("taro"),
    age: 10,
  });

  for e in &v {
    if let EnumExample::StructTypeExample {name: n, age: a} = e {
      println!("StructTypeExample_iflet: name = {}, age = {}", n, a);
    }
  }

  for e in &v {
    match e {
      EnumExample::TupleTypeExample1(s) => {
        println!("TupleTypeExample1: s = {}", s);
      }
      EnumExample::TupleTypeExample2(n, b) => {
        println!("TupleTypeExample2: n = {}, b = {}", n, b);
      }
      EnumExample::StructTypeExample{name: n, ..} => {
        println!("TupleTypeExample1: name = {}", n);
      }
    }
  }
}










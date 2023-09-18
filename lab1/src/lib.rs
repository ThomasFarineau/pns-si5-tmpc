#![allow(dead_code, unused_imports, unused_variables)]

use std::mem;

struct Vector3 {
  x: f64,
  y: f64,
  z: f64,
}

fn test_point(p: Vector3) {
  p.x;
  p.y;
  p.z;
}


impl Vector3 {
  const ZERO: Self = Self {
    x: 0.0,
    y: 0.0,
    z: 0.0,
  };

  fn new_normed(x: f64, y: f64, z: f64) -> Self {
    let norm = (x * x + y * y + z * z).sqrt();
    if norm == 0.0 {
      panic!("Norm is 0");
    }
    Self {
      x: x / norm,
      y: y / norm,
      z: z / norm,
    }
  }

  fn dot(&self, other: &Self) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  fn cross(&self, other: &Self) -> Self {
    Self {
      x: self.y * other.z - self.z * other.y,
      y: -(self.x * other.z - self.z * other.x),
      z: self.x * other.y - self.y * other.x,
    }
  }
}

#[test]
fn test_zero() {
  let z = Vector3::ZERO;
  assert_eq!(z.x, 0.0);
  assert_eq!(z.y, 0.0);
  assert_eq!(z.z, 0.0);

}


#[test]
fn test_normed() {
  let p = Vector3::new_normed(0.0, 3.5, 0.0);
  assert_eq!(p.x, 0.0);
  assert_eq!(p.y, 1.0);
  assert_eq!(p.z, 0.0);
}


fn test_dot_cross(p1: Vector3, p2: Vector3) {
  p1.dot(&p2);
  let p3: Vector3 = p1.cross(&p2);
}


fn concat(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
  let mut v = Vec::new();
  for i in v1 {
    v.push(i);
  }
  for i in v2 {
    v.push(i);
  }
  v
}


#[test]
fn test_concat() {
  let v1 = vec![1, 2, 3, 4];
  let v2 = vec![5, 6, 7, 8];

  assert_eq!(concat(v1, v2), vec![1, 2, 3, 4, 5, 6, 7, 8]);
}


fn concat2(v1: &mut Vec<i32>, v2: &Vec<i32>) {
  for i in v2 {
    v1.push(*i)
  }
}

#[test]
fn test_concat2() {
  let mut v1 = vec![1, 2, 3, 4];
  let v2 = vec![5, 6, 7, 8];
  concat2(&mut v1, &v2);
  assert_eq!(v1, vec![1, 2, 3, 4, 5, 6, 7, 8]);
  assert_eq!(v2, vec![5, 6, 7, 8]);
}

/*
Question 7:

La signature fn bad<'a, T>(x: T) -> &'a mut T suggère de renvoyer une référence à x, qui est une
variable locale stockée sur la pile et sera supprimée à la fin de la fonction. C'est impossible en
Rust car cela entraînerait une référence pendante, ce qui est dangereux et interdit par le
vérificateur d'emprunts.

Question 8:

La signature fn dup<T>(x: T) -> (T, T) est problématique en Rust car elle implique la duplication
d'une valeur T sans restrictions sur son type. Pour implémenter dup, T devrait implémenter le trait
Clone pour permettre une copie sûre.
 */
fn insertion_sort(v: &mut Vec<i32>) {
  for i in 1..v.len() {
    let mut j = i;
    while j > 0 && v[j - 1] > v[j] {
      v.swap(j, j - 1);
      j -= 1;
    }
  }
}

#[test]
fn test_sort() {
  let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
  insertion_sort(&mut v);
  assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}


// Arbres
//
// Un type opaque de valeur, avec suivi de la possession mod private {
#[derive(PartialEq, Eq, Debug)]
pub struct Value(u64);

impl Value {
  pub fn new(v: u64) -> Self {
    Value(v)
  }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Node {
  left: BST,
  key: u64,
  val: Value,
  right: BST,
}

#[derive(PartialEq, Eq, Debug)]
pub enum BST {
  Leaf,
  Node(Box<Node>),
}

impl BST {
  fn new() -> Self {
    BST::Leaf
  }

  pub fn find<'a>(&'a self, k: u64) -> Option<&'a Value> {
    match self {
      BST::Leaf => None,
      BST::Node(node) => {
        if k < node.key {
          node.left.find(k)
        } else if k > node.key {
          node.right.find(k)
        } else {
          Some(&node.val)
        }
      }
    }
  }
  pub fn find_mut<'a>(&'a mut self, k: u64) -> Option<&'a mut Value> {
    match self {
      BST::Leaf => None,
      BST::Node(node) => {
        if k < node.key {
          node.left.find_mut(k)
        } else if k > node.key {
          node.right.find_mut(k)
        } else {
          Some(&mut node.val)
        }
      }
    }
  }

  pub fn insert(&mut self, k: u64, v: Value) -> Option<Value> {
    match self {
      BST::Leaf => {
        *self = BST::Node(Box::new(Node {
          left: BST::Leaf,
          key: k,
          val: v,
          right: BST::Leaf,
        }));
        None
      }
      BST::Node(node) => {
        if k < node.key {
          node.left.insert(k, v)
        } else if k > node.key {
          node.right.insert(k, v)
        } else {
          Some(mem::replace(&mut node.val, v))
        }
      }
    }
  }
}

#[test]
fn test_new() {
  assert_eq!(BST::new(), BST::Leaf)
}

#[test]
fn test_find() {
  let b1 = BST::Node(Box::new(Node {
    left: BST::Node(Box::new(Node {
      left: BST::Leaf,
      key: 3,
      val: Value::new(2),
      right: BST::Leaf,
    })),
    key: 4,
    val: Value::new(6),
    right: BST::Leaf,
  }));
  assert_eq!(b1.find(5), None);
  assert_eq!(b1.find(4), Some(Value::new(6)).as_ref());
}

//
// Exercise 12
#[test]
fn test_find_mut() {
  let mut b1 = BST::Node(Box::new(Node {
    left: BST::Node(Box::new(Node {
      left: BST::Leaf,
      key: 3,
      val: Value::new(2),
      right: BST::Leaf,
    })),
    key: 4,
    val: Value::new(6),
    right: BST::Leaf,
  }));

  let v = b1.find_mut(4).unwrap();

  *v = Value::new(10);
  assert_eq!(b1.find(4), Some(Value::new(10)).as_ref());
}

// Exercise 13
#[test]
fn test_insert() {
  let mut b1 = BST::Leaf;

  b1.insert(3, Value::new(2));
  b1.insert(2, Value::new(5));

  assert_eq!(b1.find(3), Some(Value::new(2)).as_ref());
  assert_eq!(b1.find(2), Some(Value::new(5)).as_ref());

}

fn main() {}
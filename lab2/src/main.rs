// EXERCISE 0.1

use std::ops::Add;

fn sum<T: Default + Add<Output=T>, U: IntoIterator<Item=T>>(container: U) -> T {
    let mut res = Default::default();
    for x in container {
        res = res + x;
    }
    res
}

#[test]
fn test_sum() {
    assert_eq!(sum(vec![1.0, 2.0, 3.0]), 6.0);
    assert_eq!(sum(1..4), 6);
    let empty_usize_vec: Vec<usize> = vec![];
    let empty_f64_vec: Vec<f64> = vec![];
    assert_eq!(sum(empty_usize_vec), 0);
    assert_eq!(sum(empty_f64_vec), 0.0);
}

// EXERCISE 0.2


fn primes(n: usize) -> Vec<usize> {
    let mut res = vec![2];
    for i in (3..n).step_by(2) {
        for &j in &res {
            if i % j == 0 {
                break;
            }
            if j * j > i {
                res.push(i);
                break;
            }
        }
    }
    res
}

#[test]
fn test_primes() {
    assert_eq!(primes(10), vec![2, 3, 5, 7])
}


// EXERCISE 1

struct Circle {
    max: usize,
    current: usize,
}

fn circle(n: usize) -> Circle {
    Circle { max: n, current: 0 }
}

// Exercise 1
// Implementer `Iterator` pour `Circle`

impl Iterator for Circle {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.max {
            self.current = 0;
        }
        let res = Some(self.current);
        self.current += 1;
        res
    }
}

#[test]
fn test_circle() {
    assert_eq!(circle(3).take(6).collect::<Vec<_>>(), [0, 1, 2, 0, 1, 2]);
}

// Exercise 2

fn is_evens<I: IntoIterator<Item=i32>>(numbers: I) -> bool {
    numbers.into_iter().all(|x| x % 2 == 0)
}

#[test]
fn test_is_even() {
    assert_eq!(is_evens([2, 4, 6, 10].into_iter()), true);
    assert_eq!(is_evens(1..5), false);
}


// Exercise 3

// fn is_sorted<I: Iterator<Item=u16>>(list: &mut I) -> bool {
//     let mut iter = list.into_iter();
//     let mut prev = iter.next().unwrap();
//     for x in iter {
//         if x < prev {
//             return false;
//         }
//         prev = x;
//     }
//     true
// }

// ChatGPT GOESSS BRRRRRRRRR
fn is_sorted<T: Ord, I:  Iterator<Item = T>>(iter: I) -> bool
{
    let vec: Vec<T> = iter.collect();
    vec.windows(2).all(|window| window[0] <= window[1])
}

#[test]
fn test_is_ordered() {
    assert_eq!(is_sorted(&mut [5u16, 10u16, 3].into_iter()), false);
    assert_eq!(is_sorted(&mut (4..13).into_iter()), true);
}

////////////////////////////////////////////////////

// PART 2 : TENSORS

#[derive(Debug)]
struct Tensor<E, D>(Vec<E>, D);
// E : type of elements of the tensor
// D : type of indices ("dimension")

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Dim<I>(I);

type Dim0 = Dim<[usize; 0]>;
type Dim1 = Dim<[usize; 1]>;
type Dim2 = Dim<[usize; 2]>;
type Dim3 = Dim<[usize; 3]>;

type Vector<E> = Tensor<E, Dim1>;
type Matrix<E> = Tensor<E, Dim2>;

// EXERCISE 4

// impl<'a> Add<&'a Vector<u64>> for &'a Vector<u64> {
//    type Output = Vector<u64>;
//    fn add(self, rhs: &'a Vector<u64>) -> Vector<u64> {
//        assert_eq!(self.1, rhs.1);
//        ...
//    }
// }


// #[test]
// fn vec_u64_add() {
//     let v: &Vector<u64> = &Tensor(vec![1,2,3,4,5], Dim([5]));
//     let w: &Vector<u64> = &Tensor(vec![1,1,1,1,1], Dim([5]));
//     assert_eq!((v + w).0, vec![2,3,4,5,6]);
// }


// EXERCISE 5

// trait Scalar ...

// impl Add<...> for ... { ... }


// impl Scalar for i32 { }
// #[test]
// fn vec_add() {
//     let v = Tensor(vec![1i32,2,3,4,5], Dim([5]));
//     let w = Tensor(vec![1i32,1,1,1,1], Dim([5]));

//     assert_eq!((v + w).0, vec![2,3,4,5,6]);
// }

// EXERCISE 6


// #[test]
// fn tensor_add() {
//     let v = &Tensor(vec![1,2,3,4,5,6,7,8,9], Dim([3, 3]));
//     let w = &Tensor(vec![1,1,1,1,1,1,1,1,1], Dim([3, 3]));
//     assert_eq!(&(v + w).0[..], &[2,3,4,5,6,7,8,9,10]);

// }


// #[test]
// fn test_ext_mul(){
//     let v: &Tensor<i32,Dim2> = &Tensor(vec![1,2,3,4,5,6,7,8,9], Dim([3, 3]));
//     let k: &Tensor<i32,Dim0> = &Tensor(vec![2], Dim([]));
//     assert_eq!( &(k * v).0[..], &[2,4,6,8,10,12,14,16,18]);
//     assert_eq!( (k * v).1, v.1);

// }


// EXERCISE 7

trait NdIndex<D> {
    fn offset(&self, dim: D) -> usize;
}


// #[test]
// fn test_indexing() {
//     let v0: &Tensor<i32, Dim0>  = &Tensor(vec![42], Dim([]));
//     assert_eq!(v0[()], 42);
//     let v1: &Tensor<i32, Dim1>  = &Tensor(vec![42, 43, 44], Dim([3]));
//     assert_eq!(v1[1], 43);
//     let v2: &Tensor<i32,Dim2> = &Tensor(vec![1,2,3,4,5,6,7,8], Dim([2, 4]));
//     assert_eq!(v2[(1,2)], 6);
// }


// EXERCISE 8


// EXERCISE 9

struct View<'arr, T, D> {
    data: &'arr [T],
    dim: D,
    stride: D,
}

trait Dimension {
    type Smaller: Dimension;

    // Produces a Dimension with axis `ax` removed fn remove_axis(&self, ax: usize) -> Self::Smaller;

    // Produces a Dimension where each dimension represents
    // the row-major stride needed to traverse `self`.
    //
    // For example given `Dim([5, 5, 1])` we expect `Dim([25, 5, 1])` as an output fn strides(&self) -> Self;

    // Views the dimension as a slice of usize fn as_slice(&self) -> &[usize];
}

// #[test]
// fn test_remove_axis() {
//     assert_eq!(Dim([5,4,2]).remove_axis(1), Dim([5, 2]));
//     assert_eq!(Dim([2]).remove_axis(0), Dim([]));
//     assert_eq!(Dim([7, 100]).remove_axis(0), Dim([100]));
// }

// #[test]
// fn test_strides() {
//     assert_eq!(Dim([7, 10, 5]).strides(), Dim([50, 5, 1]));
//     assert_eq!(Dim([0, 5]).strides(), Dim([0, 0, 0]));
// }

impl<T, D: Dimension> Tensor<T, D> {
    // Returns a view of the `ax` selected at `ix`.
    //
    // For example in the matrix
    // 1 2 3
    // 4 5 6
    //
    // If we restrict Axis 0 to index 1, our view corresponds to [2, 5] fn view_axis_at_index(&self, ax: usize, ix: usize) -> View<T, D::Smaller> {
    //todo!()
    //}
}

// #[test]
// fn test_restrict_axis() {
//     let base = Tensor(vec![1,2,3,4,5,6,7,8,9], Dim([3,3]));
//     {
//         let view = base.view_axis_at_index(0, 2);

//         assert_eq!(view[0], 7);
//         assert_eq!(view[1], 8);
//         assert_eq!(view[2], 9);
//     }

//     {
//         let view = base.view_axis_at_index(1, 1);

//         assert_eq!(view[0], 2);
//         assert_eq!(view[1], 5);
//         assert_eq!(view[2], 8);
//     }
// }

fn main() {}
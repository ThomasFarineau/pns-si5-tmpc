trait Stack<T> {
    fn new() -> Self;
    fn push(&mut self, x: T);
    fn pop(&mut self) -> Option<T>;
}

////////////////////////////////////////////////////////////////////////////////////////////
// EXERCISES 1 AND 2

mod exercises_1_and_2 {
    use crate::Stack;

    enum List<T> {
        Empty,
        Cons { hd: T, tl: Box<List<T>> },
    }

    use List::*;

    impl<T> Default for List<T> {
        fn default() -> Self { Empty }
    }

    impl<T: Copy> Stack<T> for List<T> {
        fn new() -> Self {
            Empty
        }

        fn push(&mut self, x: T) {
            let mut new_tail = Box::new(Empty);
            std::mem::swap(self, &mut *new_tail);
            *self = Cons { hd: x, tl: new_tail };
        }

        fn pop(&mut self) -> Option<T> {
            let current_list = std::mem::take(self);
            match current_list {
                Empty => None,
                Cons { hd, tl } => {
                    *self = *tl;
                    Some(hd)
                }
            }
        }
    }

    #[test]
    fn test_stack() {
        let mut s = List::<char>::new();
        s.push('a');
        s.push('b');
        assert_eq!(s.pop(), Some('b'));
        s.push('c');
        assert_eq!(s.pop(), Some('c'));
        assert_eq!(s.pop(), Some('a'));
        assert_eq!(s.pop(), None);
    }
}


////////////////////////////////////////////////////////////////////////////////////////////
// EXERCISE 3

mod exercise_3 {
    use crate::Stack;
    use std::cell::Cell;

    enum ListCell<T> {
        Empty,
        Cons { hd: T, tl: List<T> },
    }

    impl<T> Default for ListCell<T> {
        fn default() -> Self { Empty }
    }

    struct List<T> {
        cell: Box<Cell<ListCell<T>>>,
    }

    impl<T> Default for List<T> {
        fn default() -> Self { Self { cell: Default::default() } }
    }

    use ListCell::*;

    impl<T> Stack<T> for List<T> {
        fn new() -> Self {
            List { cell: Box::new(Cell::new(Empty)) }
        }

        fn push(&mut self, x: T) {
            let new_list = Cons {
                hd: x,
                tl: List {
                    cell: Box::new(Cell::from(self.cell.take()))
                },
            };
            self.cell.set(new_list)
        }

        fn pop(&mut self) -> Option<T> {
            match self.cell.take() {
                Empty => {
                    None
                }
                Cons { hd, tl } => {
                    self.cell.set(tl.cell.into_inner());
                    Some(hd)
                }
            }
        }
    }

    #[test]
    fn test_stack() {
        let mut s = List::<char>::new();
        s.push('a');
        s.push('b');
        assert_eq!(s.pop(), Some('b'));
        s.push('c');
        assert_eq!(s.pop(), Some('c'));
        assert_eq!(s.pop(), Some('a'));
        assert_eq!(s.pop(), None);
    }
}


////////////////////////////////////////////////////////////////////////////////////////////
// EXERCISE 4


#[cfg(test)]
use std::cell::RefCell;

#[cfg(test)]
fn crashes_with_borrow_mut() {
    let cell = RefCell::new(());
    let _borrow1 = cell.borrow();
    let _borrow2 = cell.borrow(); // <- still ok
    let _borrow3 = cell.borrow_mut(); // <- runtime error
}

#[test]
#[should_panic]
fn test_crashes_with_borrow_mut() {
    crashes_with_borrow_mut()
}

#[cfg(test)]
fn crashes_with_borrow() {
    let cell = RefCell::new(());
    let _borrow1 = cell.borrow_mut();
    let _borrow2 = cell.borrow(); // <- runtime error
}

#[test]
#[should_panic]
fn test_crashes_with_borrow() {
    crashes_with_borrow()
}


//////////////////////////////////////////////////////////////////////////////////////////// PERSISTANT ARRAY

trait PArray<T> {
    fn from_vec(vec: Vec<T>) -> Self;
    fn get(&self, idx: usize) -> T;
    fn set(&self, idx: usize, val: T) -> Self;
}


#[cfg(test)]
fn test_parray<Arr: PArray<char>>() {
    let parr1 = Arr::from_vec(vec!['a'; 3]);
    let parr2 = parr1.set(0, 'b');
    let parr3 = parr1.set(0, 'c');
    let parr4 = parr3.set(1, 'd');
    assert_eq!(parr1.get(0), 'a');
    assert_eq!(parr2.get(0), 'b');
    assert_eq!(parr3.get(0), 'c');
    assert_eq!(parr4.get(0), 'c');
    assert_eq!(parr1.get(1), 'a');
    assert_eq!(parr2.get(1), 'a');
    assert_eq!(parr3.get(1), 'a');
    assert_eq!(parr4.get(1), 'd');
    assert_eq!(parr1.get(0), 'a');
    assert_eq!(parr2.get(0), 'b');
    assert_eq!(parr3.get(0), 'c');
    assert_eq!(parr4.get(0), 'c');
    assert_eq!(parr1.get(2), 'a');
    assert_eq!(parr2.get(2), 'a');
    assert_eq!(parr3.get(2), 'a');
    assert_eq!(parr4.get(2), 'a');
}

////////////////////////////////////////////////////////////////////////////////////////////
// EXERCISE 5

mod exercise_5 {
    use crate::PArray;
    use std::rc::Rc;

    enum ArrayNode<T> {
        RootNode(Vec<T>),
        DiffNode { idx: usize, val: T, array: Array<T> },
    }

    #[derive(Clone)]
    struct Array<T> {
        rc: Rc<ArrayNode<T>>,
    }

    use ArrayNode::*;

    impl<T: Copy> PArray<T> for Array<T> {
        fn from_vec(vec: Vec<T>) -> Self {
            Array {
                rc: Rc::new(RootNode(vec))
            }
        }

        fn get(&self, idx: usize) -> T {
            match &*self.rc {
                RootNode(vec) => vec[idx].clone(),
                DiffNode { idx: diff_idx, val, array } => {
                    if *diff_idx == idx {
                        val.clone()
                    } else {
                        array.get(idx)
                    }
                }
            }
        }

        fn set(&self, idx: usize, val: T) -> Self {
            Array {
                rc: Rc::new(DiffNode { idx, val, array: self.clone() })
            }
        }
    }

    #[test]
    fn test1() {
        crate::test_parray::<Array<char>>();
    }
}


////////////////////////////////////////////////////////////////////////////////////////////
// EXERCISE 6

mod exercise_6 {
    use crate::PArray;
    use std::rc::Rc;
    use std::cell::RefCell;

    enum ArrayNode<T> {
        RootNode(Vec<T>),
        DiffNode { idx: usize, val: T, array: Array<T> },
    }

    #[derive(Clone)]
    struct Array<T> {
        rc: Rc<RefCell<ArrayNode<T>>>,
    }

    use ArrayNode::*;

    impl<T> ArrayNode<T> {
        fn into_vec(  self) -> Vec<T> {
            match self {
                RootNode(vec) => vec,
                DiffNode { .. } => panic!(),
            }
        }
        fn into_vec_borrow(&self) -> &Vec<T> {
            match self {
                RootNode(vec) => vec,
                DiffNode { .. } => panic!(),
            }
        }
    }

    impl<T> Default for ArrayNode<T> {
        fn default() -> Self {
            RootNode(vec![])
        }
    }

    impl<T: Copy> Array<T> {
        #[cfg(test)]
        fn is_root(&self) -> bool {
            match *self.rc.borrow() {
                RootNode(_) => true,
                DiffNode { .. } => false,
            }
        }

        fn reroot(&self) {
            // todo();
        }


    }

    impl<T: Copy> PArray<T> for Array<T> {
        fn from_vec(vec: Vec<T>) -> Self {
            Array {
                rc: Rc::new(RefCell::new(RootNode(vec)))
            }
        }

        fn get(&self, idx: usize) -> T {
            self.reroot();
            self.rc.borrow().into_vec_borrow()[idx]
        }

        fn set(&self, idx: usize, val: T) -> Self {
            let res = Array {
                rc: Rc::new(RefCell::new(DiffNode { idx, val, array: self.clone() }))
            };
            res.reroot();
            res
        }
    }

    #[test]
    fn test1() {
        crate::test_parray::<Array<char>>();
    }


    #[test]
    fn test_random_with_rerooting() {
        use rand::prelude::IteratorRandom;
        let mut parray = vec![Array::<usize>::from_vec(vec![0, 1, 2, 3])];
        let mut vec = vec![vec![0, 1, 2, 3]];
        for i in 1..1000 {
            let idx = (0..4).choose(&mut rand::thread_rng()).unwrap();
            let val = (0..42).choose(&mut rand::thread_rng()).unwrap();
            let version = (0..i).choose(&mut rand::thread_rng()).unwrap();
            let mut v = vec[version].clone();
            v[idx] = val;
            let p = parray[version].set(idx, val);
            assert!(p.is_root());
            vec.push(v);
            parray.push(p);
            let version = (0..i).choose(&mut rand::thread_rng()).unwrap();
            for idx in 0..vec[version].len() {
                assert_eq!(vec[version][idx], parray[version].get(idx));
                assert!(parray[version].is_root())
            }
        }
    }
}


fn main() {
    println!("Hello, world!");
}

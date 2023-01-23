use std::cell::RefCell;

use crate::{value::NumLike, Value};

type ParentsPartials<T> = (usize, T);

#[derive(Clone, Copy)]
pub struct Node<T> {
    pub parents_partials: [ParentsPartials<T>; 2],
}

pub struct Scope<T> {
    pub nodes: RefCell<Vec<Node<T>>>,
}

impl<T: NumLike> Scope<T> {
    pub fn new() -> Scope<T> {
        Scope {
            nodes: RefCell::new(Vec::<Node<T>>::new()),
        }
    }

    pub fn value(&self, value: T) -> Value<T> {
        let len = self.nodes.borrow().len();
        self.nodes.borrow_mut().push(Node {
            parents_partials: [(len, T::zero()), (len, T::zero())],
        });
        Value {
            scope: self,
            idx: len,
            val: value,
        }
    }

    pub fn op(
        &self,
        lhs_partial: T,
        rhs_partial: T,
        lhs_idx: usize,
        rhs_idx: usize,
        new_value: T,
    ) -> Value<T> {
        let len = self.nodes.borrow().len();
        self.nodes.borrow_mut().push(Node {
            parents_partials: [(lhs_idx, lhs_partial), (rhs_idx, rhs_partial)],
        });
        Value {
            scope: self,
            idx: len,
            val: new_value,
        }
    }

    pub fn constant_op(&self, partial: T, idx: usize, new_value: T) -> Value<T> {
        let len = self.nodes.borrow().len();
        self.nodes.borrow_mut().push(Node {
            parents_partials: [(idx, partial), (len, T::zero())],
        });
        Value {
            scope: self,
            idx: len,
            val: new_value,
        }
    }
}

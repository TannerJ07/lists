use std::ptr;

struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

impl<T> LinkedList<T> {
    fn new() -> Self{
        LinkedList { front: ptr::null_mut(), back: ptr::null_mut(), len: 0 }
    }
}
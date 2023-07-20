#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Formatter;
use core::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    #[inline]
    fn new(data: T) -> Box<Self> {
        Box::new(Self { data, next: None })
    }

    #[inline]
    fn as_ref(&self) -> &T {
        &self.data
    }

    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.data
    }

    #[inline]
    fn swap(&mut self, other: &mut Node<T>) {
        core::mem::swap(&mut self.data, &mut other.data);
    }
}

impl<T> LinkedList<T> {
    #[inline]
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn append(&mut self, data: T) -> &mut Self {
        let new_node = Node::new(data);
        let mut ptr = &mut self.head;
        while let Some(node) = ptr {
            ptr = &mut node.next;
        }
        *ptr = Some(new_node);
        self.len += 1;
        self
    }

    pub fn prepend(&mut self, data: T) -> &mut Self {
        let mut new_node = Node::new(data);
        if self.head.is_some() {
            let old = self.head.take();
            new_node.next = old;
        }
        self.head = Some(new_node);
        self.len += 1;
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.head = None;
        self.len = 0;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn first(&self) -> Option<&T> {
        return if let Some(ref node) = self.head {
            Some(node.as_ref().as_ref())
        } else {
            None
        };
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        return if let Some(ref mut node) = self.head {
            Some(node.as_mut().as_mut())
        } else {
            None
        };
    }

    pub fn last(&self) -> Option<&T> {
        let mut ptr = &self.head;
        if ptr.is_none() {
            return None;
        }
        while let Some(node) = ptr {
            if node.next.is_none() {
                break;
            } else {
                ptr = &node.next;
            }
        }
        Some(ptr.as_deref().unwrap().as_ref())
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        let mut ptr = &mut self.head;
        if ptr.is_none() {
            return None;
        }
        for _ in 0..self.len - 1 {
            if let Some(node) = ptr {
                ptr = &mut node.next;
            } else {
                break;
            }
        }
        Some(ptr.as_deref_mut().unwrap().as_mut())
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        let ptr = self.head.take().unwrap();
        self.head = ptr.next;
        self.len -= 1;
        Some(ptr.data)
    }

    pub fn pop_last(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        if self.len == 1 {
            return self.pop_front();
        }
        let mut ptr = &mut self.head;
        for _ in 0..self.len - 1 {
            if let Some(node) = ptr {
                ptr = &mut node.next;
            } else {
                break;
            }
        }
        let ptr = ptr.take().unwrap();
        self.len -= 1;
        Some(ptr.data)
    }

    pub fn insert(&mut self, data: T, index: usize) -> &mut Self {
        if index >= self.len {
            self.append(data);
            return self;
        }
        if index == 0 {
            self.prepend(data);
            return self;
        }
        let mut new_node = Node::new(data);
        let mut ptr = &mut self.head;
        for _ in 0..index - 1 {
            if let Some(node) = ptr {
                ptr = &mut node.next;
            } else {
                return self;
            }
        }
        new_node.next = ptr.as_deref_mut().unwrap().next.take();
        ptr.as_deref_mut().unwrap().next = Some(new_node);
        self.len += 1;
        self
    }

    pub fn remove(&mut self, mut index: usize) -> &mut Self {
        if self.head.is_none() {
            return self;
        }
        if index == 0 {
            self.pop_front();
            return self;
        }
        if index >= self.len {
            index = self.len - 1;
        }
        let mut ptr = &mut self.head;
        for _ in 0..index - 1 {
            if let Some(node) = ptr {
                ptr = &mut node.next;
            } else {
                break;
            }
        }
        ptr.as_deref_mut().unwrap().next = ptr.as_deref_mut().unwrap().next.as_deref_mut().unwrap().next.take();
        self
    }

    pub fn reverse(&mut self) -> &mut Self {
        if self.len <= 1 {
            return self;
        }
        let mut ptr = self.head.take();
        while let Some(mut node) = ptr {
            ptr = node.next.take();
            node.next = self.head.take();
            self.head = Some(node);
        }
        self
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            ptr: self.head.as_ref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            ptr: self.head.as_mut(),
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            ptr: self
        }
    }
}

pub struct Iter<'a, T> {
    ptr: Option<&'a Box<Node<T>>>,
}

pub struct IterMut<'a, T> {
    ptr: Option<&'a mut Box<Node<T>>>,
}

pub struct IntoIter<T> {
    ptr: LinkedList<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.ptr {
            self.ptr = node.next.as_ref();
            return Some(&node.data);
        }
        None
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.ptr.take() {
            self.ptr = node.next.as_mut();
            return Some(&mut node.data);
        }
        None
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.ptr.head.take() {
            self.ptr.head = node.next;
            return Some(node.data);
        }
        None
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut ptr = &self.head;
        for _ in 0..index {
            if let Some(node) = ptr {
                ptr = &node.next;
            } else {
                break;
            }
        }
        ptr.as_deref().unwrap().as_ref()
    }
}

impl<T> IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut ptr = &mut self.head;
        for _ in 0..index {
            if let Some(node) = ptr {
                ptr = &mut node.next;
            } else {
                break;
            }
        }
        ptr.as_deref_mut().unwrap().as_mut()
    }
}

impl<T: core::fmt::Display> core::fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if self.head.is_none() {
            write!(f, "None")?;
        } else {
            let mut ptr = &self.head;
            while let Some(node) = ptr {
                write!(
                    f,
                    "{} {}",
                    node.data,
                    match node.next.is_some() {
                        true => "-> ",
                        false => "",
                    }
                )?;
                ptr = &node.next;
            }
        }
        Ok(())
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(value: Vec<T>) -> Self {
        let mut list = LinkedList::new();
        for data in value.into_iter() {
            list.append(data);
        }
        list
    }
}

impl<T> Into<Vec<T>> for LinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut list = vec![];
        for data in self.into_iter() {
            list.push(data);
        }
        list
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;
    use super::LinkedList;

    #[test]
    fn test() {
        let mut list = LinkedList::from(vec![1, 2, 3]);

        list.append(1)
            .append(2)
            .append(3)
            .append(4)
            .prepend(0)
            .reverse();

        assert_eq!(list.pop_last(), Some(0));
        assert_eq!(list.pop_front(), Some(4));

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        list.remove(2);

        let list: Vec<i32> = list.into();
        assert_eq!(vec![3, 2, 3, 2, 1], list);
    }
}

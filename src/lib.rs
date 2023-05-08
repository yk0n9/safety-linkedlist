use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

type Data<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Data<T>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Self { data, next: None }))
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Data<T>>,
    last: Option<Data<T>>,
    length: usize,
}

impl<T: Clone + Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
            last: None,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn clear(&mut self) -> &mut Self {
        self.head = None;
        self.length = 0;
        self.last = None;

        self
    }

    pub fn reverse(&mut self) -> &mut Self {
        if self.length <= 1 {
            return self;
        }

        let mut now = self.head.take();
        while let Some(n) = now.as_mut() {
            let next = n.borrow_mut().next.take();
            self.prepend(n.borrow().data.clone());
            self.length -= 1;
            now = next;
        }

        self
    }

    pub fn front(&self) -> Option<T> {
        match self.head.as_ref() {
            None => None,
            Some(n) => Some(n.borrow().data.clone()),
        }
    }

    pub fn last(&self) -> Option<T> {
        match self.last.as_ref() {
            None => None,
            Some(n) => Some(n.borrow().data.clone()),
        }
    }

    pub fn rm_front(&mut self) -> &mut Self {
        let tmp = match self.head.as_ref() {
            None => return self,
            Some(n) => n.borrow_mut().next.take(),
        };
        self.head = tmp;

        self.length -= 1;
        self
    }

    pub fn rm_last(&mut self) -> &mut Self {
        if self.last.is_none() {
            return self;
        }

        if self.length == 1 {
            self.rm_front();
            return self;
        }

        let next = match self.head.as_ref() {
            None => return self,
            Some(n) => range(n.clone(), self.length - 2),
        };
        next.borrow_mut().next = None;
        self.last = Some(next.clone());

        self.length -= 1;
        self
    }

    pub fn append(&mut self, data: T) -> &mut Self {
        let new_node = Node::new(data);
        match self.last.take() {
            Some(node) => node.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        }
        self.length += 1;
        self.last = Some(new_node);

        self
    }

    pub fn prepend(&mut self, data: T) -> &mut Self {
        let new_node = Node::new(data);
        match self.head.take() {
            Some(node) => {
                new_node.borrow_mut().next = Some(node);
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.last = Some(new_node)
            }
        }
        self.length += 1;

        self
    }

    pub fn rm(&mut self, index: usize) -> &mut Self {
        if index >= self.length {
            return self;
        }
        if index == 0 {
            self.rm_front();
            return self;
        }
        if index == self.length - 1 {
            self.rm_last();
            return self;
        }

        let next = match self.head.as_ref() {
            None => return self,
            Some(n) => range(n.clone(), index - 1),
        };
        let tmp = match next.borrow().next.as_ref() {
            None => return self,
            Some(n) => n.clone(),
        };
        next.borrow_mut().next = tmp.borrow_mut().next.take();

        self.length -= 1;
        self
    }

    pub fn set(&mut self, index: usize, data: T) -> &mut Self {
        if index >= self.length {
            return self;
        }

        let next = match self.head.as_ref() {
            None => return self,
            Some(n) => range(n.clone(), index),
        };
        next.borrow_mut().data = data;

        self
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }

        let next = match self.head.as_ref() {
            None => return None,
            Some(n) => range(n.clone(), index),
        };

        let res = Some(next.borrow().data.clone());
        res
    }

    pub fn insert(&mut self, index: usize, data: T) -> &mut Self {
        if index > self.length {
            return self;
        }

        if index == 0 {
            self.prepend(data);
            return self;
        }

        let new_node = Node::new(data);

        let next = match self.head.as_ref() {
            None => return self,
            Some(n) => range(n.clone(), index - 1),
        };

        let tmp = match next.borrow().next.as_ref() {
            None => return self,
            Some(n) => n.clone(),
        };
        new_node.borrow_mut().next = Some(tmp.clone());
        next.borrow_mut().next = Some(new_node.clone());

        self.length += 1;
        self
    }

    pub fn is_empty(&self) -> bool {
        if self.length > 0 {
            return false;
        }
        true
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head.clone(),
            length: self.length,
            index: 0,
        }
    }
}

fn range<T>(item: Data<T>, index: usize) -> Data<T> {
    let mut next = item.clone();
    for _ in 0..index {
        let tmp = match next.borrow().next.as_ref() {
            None => return item,
            Some(n) => n.clone(),
        };
        next = tmp;
    }
    next
}

impl<T: std::fmt::Display> std::fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.head.is_none() {
            write!(f, "None")?;
        } else {
            let mut next = self.head.clone();
            while let Some(node) = next {
                write!(
                    f,
                    "{} {}",
                    node.borrow().data,
                    match node.borrow().next.is_some() {
                        true => "-> ",
                        false => "",
                    }
                )?;
                next = node.borrow().next.clone();
            }
        }
        Ok(())
    }
}

impl<T: Debug + Clone> From<Vec<T>> for LinkedList<T> {
    fn from(value: Vec<T>) -> Self {
        let mut head = LinkedList::<T>::new();
        for i in value {
            head.append(i);
        }
        head
    }
}

impl<T: Debug + Clone> Into<Vec<T>> for LinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut list: Vec<T> = vec![];
        let len = self.length;
        if len > 0 {
            let mut next = self.head.clone();
            while let Some(n) = next {
                list.push(n.borrow().data.clone());
                next = n.borrow().next.clone();
            }
            return list;
        }
        list
    }
}

pub struct Iter<T> {
    head: Option<Data<T>>,
    length: usize,
    index: usize,
}

impl<T: Debug + Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.length {
            None
        } else {
            let next = match self.head.as_ref() {
                None => return None,
                Some(n) => range(n.clone(), self.index)
            };
            self.index += 1;
            let res = Some(next.borrow().data.clone());
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test() {
        let mut link = LinkedList::from(vec![1, 2, 3]);

        link.append(1)
            .append(2)
            .append(3)
            .append(4)
            .prepend(0)
            .reverse()
            .rm_front()
            .rm_last();
        assert_eq!(Some(3), link.get(3));

        let mut iter = link.iter();
        assert_eq!(iter.next(), Some(3));

        let list: Vec<i32> = link.into();
        assert_eq!(vec![3, 2, 1, 3, 2, 1], list);
    }
}

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
    list: Option<Data<T>>,
    length: usize,
    last_node: Option<Data<T>>,
}

impl<T: Clone + Debug + ?Sized> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            list: None,
            length: 0,
            last_node: None,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn clear(&mut self) -> &mut Self {
        self.list = None;
        self.length = 0;
        self.last_node = None;

        self
    }

    pub fn reverse(&mut self) -> &mut Self {
        if self.length <= 1 {
            return self;
        }

        let mut now = self.list.take();
        while let Some(n) = now.as_mut() {
            let next = n.borrow_mut().next.take();
            self.prepend(n.borrow().data.clone());
            self.length -= 1;
            now = next;
        }

        self
    }

    pub fn front(&self) -> Option<T> {
        match self.list.as_ref() {
            None => None,
            Some(n) => Some(n.borrow().data.clone()),
        }
    }

    pub fn last(&self) -> Option<T> {
        match self.last_node.as_ref() {
            None => None,
            Some(n) => Some(n.borrow().data.clone()),
        }
    }

    pub fn rm_front(&mut self) -> &mut Self {
        let tmp = match self.list.as_ref() {
            None => return self,
            Some(n) => n.borrow_mut().next.take(),
        };
        self.list = tmp;

        self.length -= 1;
        self
    }

    pub fn rm_last(&mut self) -> &mut Self {
        if self.last_node.is_none() {
            return self;
        }

        if self.length == 1 {
            self.rm_front();
            return self;
        }

        let mut next = match self.list.as_ref() {
            None => return self,
            Some(n) => n.clone(),
        };
        for _ in 0..self.length - 2 {
            let tmp = match next.borrow().next.as_ref() {
                None => return self,
                Some(n) => n.clone(),
            };
            next = tmp;
        }
        next.borrow_mut().next = None;
        self.last_node = Some(next.clone());

        self.length -= 1;
        self
    }

    pub fn append(&mut self, data: T) -> &mut Self {
        let new_node = Node::new(data);
        match self.last_node.take() {
            Some(node) => node.borrow_mut().next = Some(new_node.clone()),
            None => self.list = Some(new_node.clone()),
        }
        self.length += 1;
        self.last_node = Some(new_node);

        self
    }

    pub fn prepend(&mut self, data: T) -> &mut Self {
        let new_node = Node::new(data);
        match self.list.take() {
            Some(node) => {
                new_node.borrow_mut().next = Some(node);
                self.list = Some(new_node);
            }
            None => {
                self.list = Some(new_node.clone());
                self.last_node = Some(new_node)
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

        let mut next = match self.list.as_ref() {
            None => return self,
            Some(n) => n.clone(),
        };
        for _ in 0..index - 1 {
            let tmp = match next.borrow().next.as_ref() {
                None => return self,
                Some(n) => n.clone(),
            };
            next = tmp;
        }
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

        let mut next = match self.list.as_ref() {
            None => return self,
            Some(n) => n.clone(),
        };
        for _ in 0..index {
            let tmp = match next.borrow().next.as_ref() {
                None => return self,
                Some(n) => n.clone(),
            };
            next = tmp;
        }
        next.borrow_mut().data = data;

        self
    }

    pub fn get(&mut self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }

        let mut next = match self.list.as_ref() {
            None => return None,
            Some(n) => n.clone(),
        };
        for _ in 0..index {
            let tmp = match next.borrow().next.as_ref() {
                None => return None,
                Some(n) => n.clone(),
            };
            next = tmp;
        }

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

        let mut next = match self.list.as_ref() {
            None => return self,
            Some(n) => n.clone(),
        };

        for _ in 0..index - 1 {
            let tmp = match next.borrow().next.as_ref() {
                None => return self,
                Some(n) => n.clone(),
            };
            next = tmp;
        }
        let tmp = match next.borrow().next.as_ref() {
            None => return self,
            Some(n) => n.clone(),
        };
        new_node.borrow_mut().next = Some(tmp.clone());
        next.borrow_mut().next = Some(new_node.clone());

        self.length += 1;
        self
    }
}

impl<T: std::fmt::Display> std::fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.list.is_none() {
            write!(f, "None")?;
        } else {
            let mut next = self.list.clone();
            while let Some(node) = next {
                write!(
                    f,
                    "{} {} ",
                    node.borrow().data,
                    match node.borrow().next.is_some() {
                        true => "->",
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
        let mut list = LinkedList::<T>::new();
        for i in value {
            list.append(i);
        }
        list
    }
}

#[test]
fn test() {
    let mut list = LinkedList::<i32>::new();

    list.append(1).append(2).append(3).prepend(40);
    println!("{}", list);

    list.set(1, 10).insert(2, 70).rm(4).rm_front();
    println!("{}", list);

    list.reverse();
    println!("{}", list);

    println!("len: {}", list.length());

    let mut list = LinkedList::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    list.prepend(9).reverse();

    println!("{}", list);
}

use std::{mem::take, iter::Iterator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<T> {
    Next(T, Box<Self>),
    Tail(T),
}

impl<T> Node<T> {
    pub fn new(data: T, next: Self) -> Self {
        Self::Next(data, Box::new(next))
    }

    pub fn unwrap(self) -> T {
        match self {
            Self::Next(data, _) => data,
            Self::Tail(data) => data,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LinkedList<T> {
    head: Option<Node<T>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push_front(&mut self, data: T) {
        match self.head {
            None => self.head = Some(Node::Tail(data)),
            ref mut node => {
                let next = node.take().unwrap();

                self.head = Some(Node::new(data, next));
            }
        }

        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let data = match self.head {
            Some(Node::Tail(_)) => take(&mut self.head).map(|node| node.unwrap()),
            Some(Node::Next(_, _)) => {
                if let Node::Next(data, next) = take(&mut self.head).unwrap() {
                    self.head = Some(*next);
                    Some(data)
                } else {
                    unreachable!()
                }
            }
            None => return None,
        };

        self.len -= 1;

        data
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn push_front() {
        let mut list = LinkedList::new();

        list.push_front(10);
        list.push_front(23);

        assert_eq!(list.len(), 2);
    }

    #[test]
    fn pop_front() {
        let mut list = LinkedList::new();

        list.push_front(10);
        list.push_front(23);

        assert_eq!(list.pop_front(), Some(23));
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), None);

        assert_eq!(list.len(), 0);
    }
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

//Iterator stuff
pub struct IntoIter<T>(List<T>);
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    // should this be &self?
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr_node = self.head.take();

        while let Some(mut boxed_node) = curr_node {
            curr_node = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(3);
        list.push(2);
        list.push(1);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        list.pop();

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(3);
        list.push(2);
        list.push(1);

        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek_mut(), Some(&mut 1));
        list.peek_mut().map(|value| *value = 66);
        assert_eq!(list.peek(), Some(&66));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(3);
        list.push(6);
        list.push(9);

        let mut itr = list.into_iter();
        assert_eq!(itr.next(), Some(9));
        assert_eq!(itr.next(), Some(6));
        assert_eq!(itr.next(), Some(3));
        assert_eq!(itr.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(3);
        list.push(6);
        list.push(9);

        let mut itr = list.iter();
        assert_eq!(itr.next(), Some(&9));
        assert_eq!(itr.next(), Some(&6));
        assert_eq!(itr.next(), Some(&3));
        assert_eq!(itr.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(3);
        list.push(6);
        list.push(9);

        let mut itr = list.iter_mut();
        itr.next().map(|node| *node = 4);
        assert_eq!(itr.next(), Some(&mut 6));
        assert_eq!(itr.next(), Some(&mut 3));
        assert_eq!(itr.next(), None);
        assert_eq!(list.peek(), Some(&4));

        let mut itr2 = list.iter_mut();
        itr2.next().map(|node| *node = 9);
        assert_eq!(itr2.next(), Some(&mut 6));
        assert_eq!(itr2.next(), Some(&mut 3));
        assert_eq!(itr2.next(), None);

        assert_eq!(list.peek(), Some(&9));
    }
}

use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = *mut Node<T>; 

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut() }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        unsafe {
            Iter { next: self.head.as_ref() }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        unsafe {
            IterMut { next: self.head.as_mut() }
        }
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            let new_tail = Box::into_raw(
                Box::new(
                    Node {elem: elem, next: ptr::null_mut()}
                )
            );

            if self.tail.is_null() {
                self.head = new_tail;
            } else {
                (*(self.tail)).next = new_tail;
            }

            self.tail = new_tail;
        }

    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                return None;
            }
            else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                return Some(head.elem);
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| &node.elem)
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| &mut node.elem)
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
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() { }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn push_pop() {
        let mut list = List::new();
        list.push(3);
        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn miri_food() {
        let mut list = List::new();
        list.push(3);
        list.push(4);
        list.push(5);

        assert_eq!(list.peek(), Some(&3));
        list.peek_mut().map(|x| *x = 42);
        assert_eq!(list.peek(), Some(&42));

        let mut iter_mut = list.iter_mut();
        iter_mut.next();
        iter_mut.next().map(|x| *x = 52);
        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.pop(), Some(52));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}
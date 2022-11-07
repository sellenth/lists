use std::sync::Arc;

type Link<T> = Option<Arc<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

pub struct Node<T> {
    val: T,
    next: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Arc::new(Node {
                val: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn prepend() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(3).prepend(4).prepend(5);

        assert_eq!(list.head(), Some(&5));
    }

    #[test]
    fn itr() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(3).prepend(4).prepend(5);
        assert_eq!(list.head(), Some(&5));

        let mut itr = list.iter();

        assert_eq!(itr.next(), Some(&5));
        assert_eq!(itr.next(), Some(&4));
        assert_eq!(itr.next(), Some(&3));
        assert_eq!(itr.next(), None);
    }
}

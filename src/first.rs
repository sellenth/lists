use std::mem;

pub struct List {
    head: Link
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty}
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            val: elem,
            next: mem::replace(&mut self.head, Link::Empty )
        });

        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.val)
            }
        }
    }

}

impl Drop for List {
    fn drop(&mut self){
        let mut curr_node = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = curr_node {
            curr_node = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

struct Node {
    val: i32,
    next: Link
}

enum Link {
    Empty,
    More(Box<Node>)
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
}
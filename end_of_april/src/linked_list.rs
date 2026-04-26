pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let old_head = self.head.take();
        let new_node = Node {
            value,
            next: old_head,
        };
        self.head = Some(Box::new(new_node));
        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut current = &self.head;
        for _ in 0..index {
            match current {
                Some(node) => current = &node.next,
                None => return None,
            }
        }
        current.as_ref().map(|node| &node.value)
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

impl<T: std::fmt::Debug> LinkedList<T> {
    pub fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{:?}", node.value);
            current = &node.next;
        }
    }
}

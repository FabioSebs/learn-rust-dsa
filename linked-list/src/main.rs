struct Node {
    value: u8,
    next: Option<Box<Node>>,
    index: Option<u8>,
}

impl Node {
    fn new(val: u8, idx: Option<u8>) -> Node {
        Node {
            value: val,
            next: None,
            index: idx,
        }
    }
}

struct LinkedList {
    head: Option<Box<Node>>,
    tail: *mut Node,
    length: u8,
}

impl LinkedList {
    fn new(val: u8) -> LinkedList {
        let mut head = Box::new(Node::new(val, Option::Some((0 as u8))));
        let raw_tail: *mut Node = &mut *head;

        LinkedList {
            head: Some(head),
            tail: raw_tail,
            length: 1,
        }
    }

    fn print(&self) {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = node.next.as_ref();
        }
        println!("END")
    }

    fn add(&mut self, val: u8) {
        let mut new_node = Box::new(Node::new(val, Some(self.length)));
        let raw_node: *mut Node = &mut *new_node;

        unsafe {
            (*self.tail).next = Some(new_node);
        }

        self.tail = raw_node;
        self.length += 1;
    }

    fn get_length(&self) -> u8 {
        self.length
    }
}

fn main() {
    let mut ll = LinkedList::new(10);
    ll.add(20);
    ll.add(30);
    ll.add(40);
    ll.print();
    println!("length : {}", ll.get_length());
}

mod linked_list;
mod task;
mod todo;
mod traits;
mod types;

use linked_list::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    println!("length: {}", list.length());
    println!("get(0): {:?}", list.get(0));
    println!("get(1): {:?}", list.get(1));
    println!("get(2): {:?}", list.get(2));
    println!("get(3): {:?}", list.get(3));

    list.print();
}

// ===============================
// Rust Singly Linked List Program
// ===============================
// This file implements common linked list operations in Rust,
// useful for coding interviews and DSA practice.
// -------------------------------

#[derive(Debug)]
// Node represents a single element in the linked list
struct Node {
    data: i32,
    next: Option<Box<Node>>, // Box for heap allocation, Option for nullable
}

#[derive(Debug)]
// LinkedList is a wrapper around the head node
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    // Constructor: Creates an empty linked list
    fn new() -> Self {
        Self { head: None }
    }

    // 1. Insert: Adds a node with given data at the end
    fn insert(&mut self, data: i32) {
        let new_node = Box::new(Node { data, next: None });
        match self.head.as_mut() {
            Some(mut node) => {
                while let Some(ref mut next) = node.next {
                    node = next;
                }
                node.next = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }

    // 2. Print: Traverses and prints the list
    fn print(&self) {
        let mut temp = &self.head;
        while let Some(node) = temp {
            print!("{} -> ", node.data);
            temp = &node.next;
        }
        println!("None");
    }

    // 3. Length: Returns the number of nodes
    fn length(&self) -> usize {
        let mut count = 0;
        let mut temp = &self.head;
        while let Some(node) = temp {
            count += 1;
            temp = &node.next;
        }
        count
    }

    // 4. Search: Returns true if key exists
    fn search(&self, key: i32) -> bool {
        let mut temp = &self.head;
        while let Some(node) = temp {
            if node.data == key {
                return true;
            }
            temp = &node.next;
        }
        false
    }

    // 5. Delete by Key: Removes the first node with matching value
    fn delete_by_key(&mut self, key: i32) {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if node.data == key {
                *current = node.next.take(); // Re-link and remove node
                return;
            }
            current = &mut node.next;
        }
    }

    // 6. Delete by Position: Removes node at a specific index (0-based)
    fn delete_at_position(&mut self, pos: usize) {
        let mut current = &mut self.head;
        let mut index = 0;
        while let Some(ref mut node) = current {
            if index == pos {
                *current = node.next.take();
                return;
            }
            index += 1;
            current = &mut node.next;
        }
    }

    // 7. Delete All: Clears the entire list
    fn delete_all(&mut self) {
        self.head = None;
    }

    // 8. Nth from Start: Returns nth node's value (0-based)
    fn nth_from_start(&self, n: usize) -> Option<i32> {
        let mut index = 0;
        let mut temp = &self.head;
        while let Some(node) = temp {
            if index == n {
                return Some(node.data);
            }
            index += 1;
            temp = &node.next;
        }
        None
    }

    // 9. Nth from End: Uses length - n - 1 to get the node from start
    fn nth_from_end(&self, n: usize) -> Option<i32> {
        let len = self.length();
        if n >= len {
            return None;
        }
        self.nth_from_start(len - n - 1)
    }
}

// ====================
// Driver Code for Test
// ====================
fn main() {
    let mut list = LinkedList::new();

    println!("== Creating Linked List ==");
    list.insert(10);
    list.insert(20);
    list.insert(30);
    list.insert(40);
    list.insert(50);

    println!("== Print List ==");
    list.print();

    println!("== Length of List ==");
    println!("{}", list.length());

    println!("== Search Nodes ==");
    println!("Search 30: {}", list.search(30));
    println!("Search 99: {}", list.search(99));

    println!("== Nth Node from Start (3rd) ==");
    println!("{:?}", list.nth_from_start(2));

    println!("== Nth Node from End (2nd) ==");
    println!("{:?}", list.nth_from_end(1));

    println!("== Delete Key 30 ==");
    list.delete_by_key(30);
    list.print();

    println!("== Delete Node at Position 1 ==");
    list.delete_at_position(1);
    list.print();

    println!("== Delete Entire List ==");
    list.delete_all();
    list.print();
}
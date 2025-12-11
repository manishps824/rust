#[derive(Debug)]
// The Node struct holds the data and the pointer to the next node
struct Node {
    val: String,
    // We use Box because the size of Node is recursive
    // We use Option because the next node might not exist (null equivalent)
    next: Option<Box<Node>>,
}

// The List struct simply holds the pointer to the first node
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    // Create a new, empty list
    pub fn new() -> Self {
        LinkedList { head: None }
    }
}

impl LinkedList {
    pub fn push(&mut self, data: String) {
        // 1. Create the new node
        let new_node = Box::new(Node {
            val: data,
            // 2. We 'take' the current head value, leaving 'None' in its place momentarily.
            //    This transfers ownership of the old list to the new node.
            next: self.head.take(), 
        });

        // 3. Set the head to point to the new node (wrapped in Some)
        self.head = Some(new_node);
    }
}

impl LinkedList {
    pub fn pop(&mut self) -> Option<String> {
        // 1. Take the head. If it is None, return None (list is empty)
        //    map() allows us to work on the value inside the Option if it exists
        self.head.take().map(|node| {
            // 'node' is now a Box<Node> that we own.
            
            // 2. Set the list's head to be the next node
            self.head = node.next;

            // 3. Return the data from the popped node
            node.val
        })
    }
}

impl LinkedList {
    // A simple function to print the list state
    pub fn print(&self) {
        // Start with a reference to the head
        let mut current = &self.head;

        // While 'current' is Some(node), loop
        while let Some(node) = current {
            print!("{} -> ", node.val);
            // Move the reference to the next node
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    // Push strings
    list.push("Rust".to_string());
    list.push("is".to_string());
    list.push("Awesome".to_string());

    println!("Current list:");
    list.print(); // Output: Awesome -> is -> Rust -> None

    // Pop a string
    println!("\nPopped: {:?}", list.pop()); // Output: Some("Awesome")

    println!("List after pop:");
    list.print(); // Output: is -> Rust -> None
}
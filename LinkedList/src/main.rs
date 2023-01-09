use std::fmt::Display;

struct Node {
    id: i32,
    value: i32,
    next: Option<Box<Node>>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, value: {}", self.id, self.value)?;
        Ok(())
    }
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            id: 0,
            value,
            next: None,
        }
    }

    fn insert_value(&mut self, value: i32) {
        match self.next {
            Some(ref mut val) => val.insert_value(value),
            None => {
                self.next = Some(Box::new(Node {
                    id: self.id + 1,
                    value,
                    next: None,
                }));
            }
        }
    }
    fn print_value(&self) {
        match self.next {
            Some(ref value) => {
                println!("{self}");
                value.print_value();
            }
            None => {
                println!("{self}");
            }
        }
    }
    fn search_value(&self, value: i32) {
        match self.next {
            _ if self.value == value => {
                println!("Found {self}");
            }
            Some(ref val) => val.search_value(value),
            None => {
                println!("Value not found");
            }
        }
    }
}

fn main() {
    let mut list = Node::new(12);
    list.insert_value(66);
    list.insert_value(45);
    list.print_value();
    list.search_value(12);
    list.search_value(66);
}

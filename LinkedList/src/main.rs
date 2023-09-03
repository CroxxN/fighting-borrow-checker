use std::{fmt::Display, mem};

#[derive(Clone)]
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
    // Universal, all-pervading, all-encompassing, all-powerful, omnipotent, omniscient, omnipresent, primordial
    // pre-historical, data: 0
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
        if self.value == value {
            println!("Found {self}");
            return;
        }
        match self.next {
            Some(ref val) => val.search_value(value),
            None => {
                println!("Value not found");
            }
        }
    }
    fn delete_id(&mut self, id: i32) {
        // Currently ignore if the value is the first
        if self.value == id {
            if let Some(ref n) = self.next {
                _ = mem::replace(self, n.as_ref().to_owned());
            } else {
                *self = Self::new(0);
            }
        }
        match self.next {
            Some(ref mut val) if val.value == id => {
                if let Some(ref t) = val.next {
                    _ = mem::replace(val, t.to_owned());
                } else {
                    self.next = None;
                }
                println!("DONE");
            }
            Some(ref mut val) => val.delete_id(id),
            None => {
                println!("Such value doesn't exist");
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
    // Works. First we check if 45 exists(we know it does), then we delete it
    // and then we search again, which returns null this time. Neat
    list.search_value(45);
    list.delete_id(45);
    list.search_value(45);
    list.delete_id(12);
    list.delete_id(66);
    list.print_value();
}


#[derive(Debug, PartialEq, Eq)]

struct LinkedList{
    
    value: i32,
    next: Option<Box<LinkedList>>
}


impl LinkedList{
    fn new(value: i32)-> Self{
        Self{
            value,
            next: None
        }
    }
    
    fn add(&mut self, value: i32) {
            self.next = Some(Box::new(LinkedList{
                value,
                next: None
        }));
    }
    
    fn print_list(&self) {
        println!("{:?}", self.value);
        if let Some(next) = &self.next {
            next.print_list();
        } else {}
    }
}
fn main(){
    let mut a = LinkedList::new(32);
    a.add(33);
    a.add(5);
    a.add(10);
    a.print_list();
}

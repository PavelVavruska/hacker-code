#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[test]
    fn test_scopes() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
    
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), // 1
            Rc::weak_count(&leaf), // 0
        );
    
        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });
    
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch), // 1
                Rc::weak_count(&branch),  // 1
            );
    
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf), // 2
                Rc::weak_count(&leaf), // 0
            );
        }
    
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),  // 1
            Rc::weak_count(&leaf),  // 0
        );
    }
}
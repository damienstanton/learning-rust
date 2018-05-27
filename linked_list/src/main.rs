use List::*;

// List: a linked-list
enum List {
    // Cons: an element and a pointer to the next node in the list
    Cons(u32, Box<List>),
    // Nil: the null-termination at the end of the list
    Nil,
}

impl List {
    // new: a constructor for the linked list type
    fn new() -> List {
        Nil
    }
    // add: prepend an element to the front of the list
    fn add(self, element: u32) -> List {
        Cons(element, Box::new(self))
    }

    // len: get the length of the list
    fn len(&self) -> u32 {
        match *self {
            // Note: interesting point from the book:
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn to_string(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.to_string()),
            Nil => format!("∅"),
        }
    }
}

#[cfg(test)]
mod tests {
    use List;
    fn gen_list() -> List {
        let mut list = List::new();
        list = list.add(1);
        list = list.add(2);
        list = list.add(3);
        list
    }

    #[test]
    fn check_length() {
        let list = gen_list();
        assert_eq!(3, list.len());
    }

    #[test]
    fn check_printed_val() {
        let list = gen_list();
        assert_eq!("3, 2, 1, ∅", list.to_string());
    }
}

use std::ops::Deref;
use std::rc::Rc;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum OtherList {
    Cons(i32, Rc<OtherList>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello ,{}", name);
}

#[cfg(test)]
mod tests {
    use crate::List::{Cons, Nil};
    use crate::MyBox;
    use crate::hello;
    use std::rc::Rc;
    use crate::OtherList;

    #[test]
    fn multi_ref() {
        let a = Rc::new(OtherList::Cons(5, Rc::new(OtherList::Nil)));
        let b = OtherList::Cons(3, Rc::clone(&a));
        let c = OtherList::Cons(4, Rc::clone(&a));
    }

    #[test]
    fn deref_coercion() {
        let m = MyBox::new(String::from("Man"));
        hello(&m);
    }

    #[test]
    fn recursive_type() {
        let _list = Cons(
            1,
        Box::new(Cons(
            2,
            Box::new(Cons(
                3,
                Box::new(Nil)))))
        );

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn smart_pointer() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}

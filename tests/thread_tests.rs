use std::thread;
use rust_dsa::doubly_linked_list::DubLinkedList;
use rust_dsa::stack::Stack;

#[test]
fn thread_tests() {
    let mut s = Stack::<usize>::new();
    s.push(1);
    s.push(5);
    s.push(10);

    let handle = thread::spawn(move || {
        println!("{:?}", s.pop());
        s.pop()
    });

   // println!("{x:?}");

/*
    let handle2 = thread::spawn(move || {
        let e = s.as_ref().pop();
        println!("pop that {e:?} off");
    });
*/
//    println!("stack {s:?}");
    println!("{:?}", handle.join().unwrap());
 //   handle2.join().unwrap();

    let mut dl = DubLinkedList::<usize>::new();

    dl.push_front(5);

    // yay, rust compiler doesn't let us move an Rc, Rust W
    // thread::spawn(move || {
        // dl.push_back(10);
        // println!("{:?}", dl.pop_front());
    // });
    assert!(true);
}
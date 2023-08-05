use std::fmt;


#[derive(Debug)]
enum BinaryTree<T>{
    Nil,
    Node(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>),
}

impl<T> BinaryTree<T> where T: std::cmp::PartialEq + 'static{
    fn find(&self, x: &T) -> bool{
        match self {
            BinaryTree::<T>::Nil => false,
            BinaryTree::<T>::Node(t, g, d) => (*t == *x) || g.find(x) || d.find(x), 
        }
    }
}

impl<T> BinaryTree<T> where T: fmt::Display + 'static{
    fn prefix_search(&self) {
        match self {
            BinaryTree::<T>::Nil => (),
            BinaryTree::<T>::Node(t, g, d) => {
                print!("{}", t);
                g.prefix_search(); d.prefix_search()
            }
        }
    }
}

impl<T> BinaryTree<T> where T: fmt::Display + 'static{
    fn infix_search(&self) {
        match self {
            BinaryTree::<T>::Nil => (),
            BinaryTree::<T>::Node(t, g, d) => {
                g.infix_search(); 
                print!("{}", t);
                d.infix_search()
            }
        }
    }
}

impl<T> BinaryTree<T> where T: fmt::Display + 'static{
    fn postfix_search(&self) {
        match self {
            BinaryTree::<T>::Nil => (),
            BinaryTree::<T>::Node(t, g, d) => {
                g.postfix_search(); 
                print!("{}", t);
                d.postfix_search()
            }
        }
    }
}

fn main() {
    
    let t: BinaryTree<i64> = BinaryTree::<i64>::Node(1, Box::new(BinaryTree::<i64>::Node(0, Box::new(BinaryTree::<i64>::Nil), Box::new(BinaryTree::<i64>::Nil))), Box::new(BinaryTree::<i64>::Nil));

    println!("t is {:?}", t);

    println!("{}", t.find(&2));
    println!("{}", t.find(&0));

    t.prefix_search();
}

use std::collections::VecDeque;
use std::fmt;
use std::cmp;


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



impl<T> BinaryTree<T>{
    fn height(&self) -> u64{
        match self {
            BinaryTree::<T>::Nil => 0,
            BinaryTree::<T>::Node(t, g, d) => 1 + cmp::max(g.height(), d.height()),
        }
    }

    fn size(&self) -> u64{
        match self{
            BinaryTree::<T>::Nil => 0,
            BinaryTree::<T>::Node(t, g, d) => 1 + g.size() + d.size(),
        }
    }

    fn dfs(&self){
        match self {
            BinaryTree::<T>::Nil => (),
            BinaryTree::<T>::Node(t, g, d) => {
                // Do thing on t
                dfs(g); dfs(d)
            }
        }
    }

    fn bfs(&self){
        let mut q: VecDeque<BinaryTree<T>> = VecDeque::new();
        
        q.push_back(self);

        while q.len() != 1{
            let t = q.pop_front();

            match t {
                BinaryTree::<T>::Nil => (),
                BinaryTree::<T>::Node(x, g, d) => {
                    // Do thing on x
                    q.push_back(g);
                    q.push_back(d);
                }
            }
        }
    }
}
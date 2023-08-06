use std::collections::VecDeque;
use std::fmt;
use std::cmp;

use rand::Rng;

#[derive(Debug)]
pub enum BinaryTree<T>{
    Nil,
    Node(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>),
}

impl<T> BinaryTree<T> where T: std::cmp::PartialEq + 'static{
    pub fn find(&self, x: &T) -> bool{
        match self {
            BinaryTree::<T>::Nil => false,
            BinaryTree::<T>::Node(t, g, d) => (*t == *x) || g.find(x) || d.find(x), 
        }
    }
}

impl<T> BinaryTree<T> where T: fmt::Display + 'static{
    pub fn prefix_search(&self) {
        match self {
            BinaryTree::<T>::Nil => (),
            BinaryTree::<T>::Node(t, g, d) => {
                print!("{}", t);
                g.prefix_search(); d.prefix_search()
            }
        }
    }
    pub fn infix_search(&self) {
        match self {
            BinaryTree::<T>::Nil => (),
            BinaryTree::<T>::Node(t, g, d) => {
                g.infix_search(); 
                print!("{}", t);
                d.infix_search()
            }
        }
    }
    pub fn postfix_search(&self) {
        match self {
            BinaryTree::<T>::Nil => (),
            BinaryTree::<T>::Node(t, g, d) => {
                g.postfix_search(); 
                print!("{}", t);
                d.postfix_search()
            }
        }
    }

    pub fn print_bfs(&self){
        let mut q: VecDeque<&BinaryTree<T>> = VecDeque::new();
        
        q.push_back(self);

        while q.len() != 0{
            let t = q.pop_front().unwrap();

            match t {
                BinaryTree::<T>::Nil => (),
                BinaryTree::<T>::Node(x, g, d) => {
                    print!("{} ", x);
                    q.push_back(&g);
                    q.push_back(&d);
                }
            }
        }
        println!("");
    }
}



impl<T> BinaryTree<T>{
    pub fn height(&self) -> u64{
        match self {
            BinaryTree::<T>::Nil => 0,
            BinaryTree::<T>::Node(t, g, d) => 1 + cmp::max(g.height(), d.height()),
        }
    }

    pub fn size(&self) -> u64{
        match self{
            BinaryTree::<T>::Nil => 0,
            BinaryTree::<T>::Node(t, g, d) => 1 + g.size() + d.size(),
        }
    }

    pub fn dfs(&self){
        match self {
            BinaryTree::<T>::Nil => (),
            BinaryTree::<T>::Node(t, g, d) => {
                // Do thing on t
                g.dfs(); d.dfs()
            }
        }
    }

    pub fn bfs(&self){
        let mut q: VecDeque<&BinaryTree<T>> = VecDeque::new();
        
        q.push_back(self);

        while q.len() != 1{
            let t = q.pop_front().unwrap();

            match t {
                BinaryTree::<T>::Nil => (),
                BinaryTree::<T>::Node(x, g, d) => {
                    // Do thing on x
                    q.push_back(&g);
                    q.push_back(&d);
                }
            }
        }
    }

    pub fn add_simple(&mut self, x: T){
        match self {
            BinaryTree::<T>::Nil => {
                let sub_t = BinaryTree::<T>::Node(x, Box::new(BinaryTree::<T>::Nil), Box::new(BinaryTree::<T>::Nil));
                *self = sub_t;
            } 
            BinaryTree::<T>::Node(y, g, d) => g.add_simple(x)
        }
    }

    pub fn add_random(&mut self, x: T){
        match self {
            BinaryTree::<T>::Nil => {
                let sub_t = BinaryTree::<T>::Node(x, Box::new(BinaryTree::<T>::Nil), Box::new(BinaryTree::<T>::Nil));
                *self = sub_t;
            } 
            BinaryTree::<T>::Node(_, g, d) => {
                let r = rand::random::<u8>() % 2;
                if r == 0{
                    g.add_random(x);
                }
                else{
                    d.add_random(x);
                }
            }
        }

    }
}

impl<T> BinaryTree<T>{
    pub fn reverse(&mut self){
        match self {
            BinaryTree::<T>::Nil => (),
            BinaryTree::<T>::Node(x, g, d) => {
                g.reverse();
                d.reverse();
                std::mem::swap(g, d);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_find() {
        let t: BinaryTree<i64> = BinaryTree::<i64>::Node(1, Box::new(BinaryTree::<i64>::Node(0, Box::new(BinaryTree::<i64>::Nil), Box::new(BinaryTree::<i64>::Nil))), Box::new(BinaryTree::<i64>::Nil));

        assert!(!t.find(&2));
        assert!(t.find(&0));
    }

    #[test]
    fn test_height() {
        let mut t: BinaryTree<i64> = BinaryTree::<i64>::Node(1, Box::new(BinaryTree::<i64>::Node(0, Box::new(BinaryTree::<i64>::Nil), Box::new(BinaryTree::<i64>::Nil))), Box::new(BinaryTree::<i64>::Nil));

        assert_eq!(t.height(), 2);
        
        t.add_simple(1);
        assert_eq!(t.size(), 3);
    }

    #[test]
    fn test_size() {
        let mut t: BinaryTree<i64> = BinaryTree::<i64>::Node(1, Box::new(BinaryTree::<i64>::Node(0, Box::new(BinaryTree::<i64>::Nil), Box::new(BinaryTree::<i64>::Nil))), Box::new(BinaryTree::<i64>::Nil));

        assert_eq!(t.size(), 2);

        t.add_simple(1);
        assert_eq!(t.size(), 3);
    }
}
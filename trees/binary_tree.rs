use std::Boxed::Box;

/* Two styles of binary tree, 
it can either be an enum of tree and nil
or it can be enum of node and leaf
We use the former */

enum BinaryTree<T>{
    Nil,
    Node(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>),
}



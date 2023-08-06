
use binaryt::BinaryTree;

fn main() {
    
    let t: BinaryTree<i64> = BinaryTree::<i64>::Node(1, Box::new(BinaryTree::<i64>::Node(0, Box::new(BinaryTree::<i64>::Nil), Box::new(BinaryTree::<i64>::Nil))), Box::new(BinaryTree::<i64>::Nil));

    println!("t is {:?}", t);

    t.print_bfs();
}

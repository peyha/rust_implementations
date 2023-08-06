
use binary_tree::BinaryTree;

fn main() {
    
    let t: BinaryTree<i64> = BinaryTree::<i64>::Node(1, Box::new(BinaryTree::<i64>::Node(0, Box::new(BinaryTree::<i64>::Nil), Box::new(BinaryTree::<i64>::Nil))), Box::new(BinaryTree::<i64>::Nil));

    println!("t is {:?}", t);

    println!("{}", t.find(&2));
    println!("{}", t.find(&0));

    t.prefix_search();
    println!("");

    println!("{}", t.height());
    println!("{}", t.size());
}

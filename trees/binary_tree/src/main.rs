
use binaryt::BinaryTree;

fn main() {
    
    let mut t: BinaryTree<i64> = BinaryTree::<i64>::Nil;

    for i in 0..5{
        t.add_random(i);
    }
    
    println!("t is {:?}", t);

    t.print_bfs();
    t.reverse();
    t.print_bfs();
    
}

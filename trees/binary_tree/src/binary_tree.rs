
#[derive(Debug)]
enum BinaryTree<T>{
    Nil,
    Node(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>),
}


fn main() {
    
    let t: BinaryTree<i64> = BinaryTree<i64>::Node(0, Box::new(Nil), Box::new(Nil));

    println!("t is {:?}", t);

}

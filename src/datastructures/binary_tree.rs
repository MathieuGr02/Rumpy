pub struct BST<T>{
    root: Box<Node<T>>,
    height: usize
}

struct Item<T>{
    value: T,
    next: Link<T>
}

enum Link<T>{
    Empty,
    More(Box<Item<T>>)
}

struct Node<T>{
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: T
}

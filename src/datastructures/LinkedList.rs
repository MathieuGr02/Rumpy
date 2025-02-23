struct LinkedList {
    head: Node
}

struct Node<T> {
    link: Option<Box<Node>>,
    value: T
}

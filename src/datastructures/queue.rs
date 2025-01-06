pub struct Queue<T>{
    current: Link<T>,
    length: usize
}

enum Link<T>{
    Empty,
    More(Box<Item<T>>)
}

struct Item<T>{
    value: T,
    next: Link<T>
}

impl<T> Queue<T>{
    pub fn new(){

    }

    pub fn push(){}
}
pub struct BST<T> {
    root: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy> BST<T> {
    pub fn new() -> BST<T> {
        BST { root: None }
    }

    pub fn insert(&mut self, item: T) {
        let new_node = Box::new(Node {
            value: item,
            left: None,
            right: None,
        });

        let mut current_node = &mut self.root;
        // Move down tree and search for location of empty child
        while let Some(ref mut node) = current_node {
            if node.value >= item {
                current_node = &mut node.left;
            } else {
                current_node = &mut node.right;
            }
        }
        println!("{:?}", current_node.unwrap().value);
    }

    pub fn delete(&mut self, item: T) {}

    pub fn max(&mut self) {}

    pub fn min(&mut self) {}

    pub fn size(&self) -> i32 {
        let mut height: i32 = 0;

        height
    }
}

#[cfg(test)]
mod tests {
    use crate::datastructures::binary_tree::BST;
    #[test]
    fn bst_test() {
        let mut bst = BST::<i32>::new();
    }
}

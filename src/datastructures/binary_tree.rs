use std::fmt::Debug;

pub struct BST<T> {
    root: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy + Debug> BST<T> {
    pub fn new() -> BST<T> {
        BST { root: None }
    }

    /// Insert new node into tree
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
        // Set new node into child
        *current_node = Some(new_node);
    }

    /// Check if item exists in tree
    pub fn exists(&self, item: T) -> bool {
        let mut current_node = &self.root;

        while let Some(ref node) = current_node {
            if node.value == item {
                return true;
            } else if node.value > item {
                current_node = &node.left;
            } else {
                current_node = &node.right;
            }
        }
        false
    }

    /// Delete node from tree
    pub fn delete(&mut self, item: T) {
        let mut current_node = &mut self.root;

        while let Some(ref mut node) = current_node {
            if node.value == item {
                break;
            } else if node.value > item {
                current_node = &mut node.left;
            } else {
                current_node = &mut node.right;
            }
        }
    }

    /// Find max value of tree
    pub fn max(&self) -> T {
        let mut current_node = &self.root;

        while let Some(ref node) = current_node {
            current_node = &node.right;
        }

        current_node.as_ref().unwrap().value
    }

    /// Find min value of tree
    pub fn min(&self) -> T {
        let mut current_node = &self.root;

        while let Some(ref node) = current_node {
            current_node = &node.left;
        }

        current_node.as_ref().unwrap().value
    }

    /// Get height of tree
    pub fn size(&self) -> i32 {
        let mut height: i32 = 0;

        height
    }
}

#[cfg(test)]
mod tests {
    use crate::datastructures::binary_tree::BST;

    #[test]
    fn bst_insert_test() {
        let mut bst = BST::<i32>::new();
        bst.insert(2);
        bst.insert(3);
        bst.insert(5);
        bst.insert(1000);
        assert!(bst.exists(2));
        assert!(bst.exists(3));
        assert!(bst.exists(5));
        assert!(bst.exists(1000));
    }

    #[test]
    fn bst_get_max_test() {
        let mut bst = BST::<i32>::new();
        bst.insert(2);
        bst.insert(3);
        bst.insert(5);
        assert_eq!(5, bst.max());
    }
}

/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 插入新值到BST中
    fn insert(&mut self, value: T) {
        self.root = Self::add(self.root.take(), value);
    }

    // 递归帮助函数，将值插入树中
    fn add(node: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut cur) => {
                if cur.value < value {
                    // 当前节点的值较小，递归到右子树
                    cur.right = Self::add(cur.right.take(), value);
                } else if cur.value > value {
                    // 当前节点的值较大，递归到左子树
                    cur.left = Self::add(cur.left.take(), value);
                }
                Some(cur) // 返回更新后的节点
            }
            None => Some(Box::new(TreeNode::new(value))), // 创建新节点并返回
        }
    }
    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut current = &self.root;

        while let Some(ref node) = current {
            if node.value == value {
                return true;
            } else if node.value > value {
                // If the current node's value is greater, move to the left subtree
                current = &node.left;
            } else {
                // If the current node's value is smaller, move to the right subtree
                current = &node.right;
            }
        }

        false // If we reach a None (leaf node), return false
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();


        assert_eq!(bst.search(1), false);


        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);


        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);


        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();


        bst.insert(1);
        bst.insert(1);


        assert_eq!(bst.search(1), true);


        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}    



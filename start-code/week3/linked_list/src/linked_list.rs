use std::fmt;
use std::option::Option;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { value, next }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, size: 0 }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.value
        })
    }
}

impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        let mut first = true;
        write!(f, "[")?;
        while let Some(node) = current {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", node.value)?;
            current = &node.next;
            first = false;
        }
        write!(f, "]")
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut values = Vec::new();
        let mut current = &self.head;
        while let Some(node) = current {
            values.push(node.value.clone());
            current = &node.next;
        }
        let mut new_list = LinkedList::new();
        for value in values.into_iter().rev() {
            new_list.push_front(value);
        }
        new_list
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        let mut a = &self.head;
        let mut b = &other.head;
        while let (Some(node_a), Some(node_b)) = (a, b) {
            if node_a.value != node_b.value {
                return false;
            }
            a = &node_a.next;
            b = &node_b.next;
        }
        a.is_none() && b.is_none()
    }
}

// 由于ComputeNorm trait未定义，这里注释掉相关实现
// impl<T> ComputeNorm for LinkedList<T> {
//     fn compute_norm(&self) -> T {
//         let mut current = self.head.clone();
//         let mut norm = T::default();
//         while let Some(node) = current {
//             norm += node.value.clone();
//             current = node.next.clone();
//         }
//         norm
//     }
// }

// 不要为LinkedList实现Iterator trait，否则会与标准库的IntoIterator实现冲突
// 可以实现一个专用的迭代器
pub struct LinkedListIntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for LinkedListIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        LinkedListIntoIter { list: self }
    }
}
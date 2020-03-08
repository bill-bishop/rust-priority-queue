use std::fmt::Debug;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Node<T> {
    pub elem: T,
    pub count: usize,
}

impl<T> Eq for Node<T> {}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl<T> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.count.cmp(&other.count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let n1 = Node{ elem: 5, count: 10 };
        let n2 = Node{ elem: 6, count: 10 };
        let n3 = Node{ elem: 7, count: 19 };
        let n4 = Node{ elem: 1, count: 1 };
        let n5 = Node{ elem: 2, count: 2 };

        assert_eq!(n1.cmp(&n2), Ordering::Equal);
        assert_eq!(n2.cmp(&n3), Ordering::Less);
        assert_eq!(n3.cmp(&n4), Ordering::Greater);
        assert_eq!(n4.cmp(&n5), Ordering::Less);
    }
}

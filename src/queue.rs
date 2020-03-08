use std::cmp::Ordering;

pub struct PriorityQueue<T>(pub Vec<T>);

impl<T> PriorityQueue<T> where T: Ord + Eq + PartialEq {
    pub fn new() -> Self {
        PriorityQueue(Vec::new())
    }

    pub fn remove_max(&mut self) -> Option<T> {
        let len = self.0.len();
        if len == 0 { return None; }

        let vec = &mut self.0;

        vec.swap(0, len - 1);

        let max = vec.pop();

        let len = len - 1;
        let mut parent_idx = 1;

        loop {
            let a_idx = parent_idx * 2;
            let b_idx = a_idx + 1;

            if a_idx >= len {
                break;
            }

            let parent_val = &vec[parent_idx - 1];
            let a_val = &vec[a_idx - 1];
            let b_val = if len > b_idx { &vec[b_idx - 1] } else { a_val };

            let (max_child_idx, max_child_val) = match a_val.cmp(b_val) {
                Ordering::Less => {
                    (b_idx, b_val)
                }
                _ => (a_idx, a_val)
            };

            if let Ordering::Greater = max_child_val.cmp(parent_val) {
                println!("Swapping {} -> {}", parent_idx, max_child_idx);

                vec.swap(parent_idx - 1, max_child_idx - 1);
                parent_idx = max_child_idx;
                continue;
            }
            else {
                break;
            }
        }

        max
    }

    pub fn insert(&mut self, elem: T) {
        // Equivalent to `let PriorityQueue(ref mut vec) = self;`
        // Equivalent to `let PriorityQueue(vec) = self;`
        let vec = &mut self.0;

        vec.push(elem);

        // leave these 1-indexed. subtract when accessing
        let mut inserted_idx = vec.len();
        let mut parent_idx = inserted_idx / 2;

        while parent_idx > 0 {
            let inserted_val = &vec[inserted_idx - 1];
            let parent_val = &vec[parent_idx - 1];

            match inserted_val.cmp(parent_val) {
                Ordering::Greater => {
                    println!("Swapping {} -> {}", inserted_idx, parent_idx);

                    vec.swap(inserted_idx - 1, parent_idx - 1);

                    inserted_idx = inserted_idx / 2;
                    parent_idx = parent_idx / 2;

                    continue;
                }
                _ => { break; }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::*;

    #[test]
    fn basics() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();

        assert_eq!(pq.remove_max(), None);

        pq.insert(4);
        pq.insert(500);
        pq.insert(6);

        assert_eq!(pq.remove_max(), Some(500));
        assert_eq!(pq.remove_max(), Some(6));
        assert_eq!(pq.remove_max(), Some(4));
        assert_eq!(pq.remove_max(), None);

        pq.insert(-999);
        pq.insert(0);
        pq.insert(999);

        assert_eq!(pq.remove_max(), Some(999));
        assert_eq!(pq.remove_max(), Some(0));
        assert_eq!(pq.remove_max(), Some(-999));
        assert_eq!(pq.remove_max(), None);

        pq.insert(2);
        pq.insert(1);
        pq.insert(5);
        pq.insert(3);
        pq.insert(6);

        assert_eq!(pq.remove_max(), Some(6));
        assert_eq!(pq.remove_max(), Some(5));
        assert_eq!(pq.remove_max(), Some(3));
        assert_eq!(pq.remove_max(), Some(2));
        assert_eq!(pq.remove_max(), Some(1));
    }

    #[test]
    fn numbers() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();

        pq.insert(2);
        pq.insert(5);
        pq.insert(1);
        pq.insert(3);
        pq.insert(6);

        assert_eq!(pq.0[0], 6);
        assert_eq!(pq.0[1], 5);
        assert_eq!(pq.0[2], 1);
        assert_eq!(pq.0[3], 2);
        assert_eq!(pq.0[4], 3);

        assert_eq!(pq.remove_max(), Some(6));
        assert_eq!(pq.remove_max(), Some(5));
        assert_eq!(pq.remove_max(), Some(3));
        assert_eq!(pq.remove_max(), Some(2)); // broken
        assert_eq!(pq.remove_max(), Some(1));
    }

    #[test]
    fn nodes() {
        let mut pq: PriorityQueue<Node<i32>> = PriorityQueue::new();

        pq.insert(Node { elem: 4, count: 2 });
        pq.insert(Node { elem: 2, count: 5 });
        pq.insert(Node { elem: 5, count: 1 });
        pq.insert(Node { elem: 3, count: 3 });
        pq.insert(Node { elem: 1, count: 6 });

        assert_eq!(pq.remove_max(), Some(Node { elem:1, count: 6 }));
        assert_eq!(pq.remove_max(), Some(Node { elem:2, count: 5 }));
        assert_eq!(pq.remove_max(), Some(Node { elem:3, count: 3 }));
        assert_eq!(pq.remove_max(), Some(Node { elem:4, count: 2 })); // broken
        assert_eq!(pq.remove_max(), Some(Node { elem:5, count: 1 }));
    }

    #[test]
    fn insert() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();

        pq.insert(4);
        pq.insert(5);
        pq.insert(6);

        // Structure should now look like this:
        //        6
        //      /   \
        //     4     5
        //
        // Represented in memory by a Vec<i32>:
        //  [6, 4, 5]

        assert_eq!(pq.0.get(0), Some(&6));
        assert_eq!(pq.0.get(1), Some(&4));
        assert_eq!(pq.0.get(2), Some(&5));
    }
}


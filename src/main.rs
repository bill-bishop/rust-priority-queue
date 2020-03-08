use std::collections::HashMap;

mod queue;
use queue::*;
mod node;
use node::*;


fn main() {
    let mut counts: HashMap<i32, usize> = HashMap::new();

    let barcodes = vec![1, 1, 1, 1,1,1, 2, 2,2,2,2, 3,3,3,4,4,5];

    for barcode in barcodes {
        let count = match counts.get(&barcode) {
            Some(count) => count + 1,
            _ => 1
        };
        counts.insert(barcode, count);
    }

    let mut pq = PriorityQueue::new();

    for (n, &count) in counts.iter() {
        let node = Node { elem: n, count: count };
        println!("Inserting {:?}", node);
        pq.insert(node);
    }

    while let Some(Node { elem, .. }) = pq.remove_max() {
        if pq.0.len() > 0 {
            println!("Removed {}, greatest: {}", elem, &pq.0[0].elem);
        }
        else {
            println!("Removed {}", elem);
        }
    }
}

#[cfg(test)]
mod tests {
}

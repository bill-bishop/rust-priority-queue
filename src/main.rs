use std::collections::HashMap;

mod queue;
use queue::*;
mod node;
use node::*;

fn spread_numbers(numbers: &mut Vec<i32>) {
    let mut counts: HashMap<i32, usize> = HashMap::new();

    for &number in numbers.iter() {
        counts.insert(number, match counts.get(&number) {
            Some(count) => count + 1,
            None => 1
        });
    }

    let mut pq = PriorityQueue::new();

    for (&n, &count) in counts.iter() {
        let node = Node { elem: n, count: count };
        println!("Insert {:?}", node);
        pq.insert(node);
    }

    numbers.clear();

    while let Some(node) = pq.remove_max() {
        println!("Remove {:?}", node);
        numbers.push(node.elem);
    }
}

fn main() {
    let mut barcodes = vec![1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 4, 4, 5];
    spread_numbers(&mut barcodes);

    for &number in barcodes.iter() {
        println!("{}", number);
    }
}

#[cfg(test)]
mod tests {
}

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

    let mut max;
    let mut next = None;

    loop {
        max = match pq.remove_max() {
            Some(node) => node,
            None => { break; }
        };

        numbers.push(max.elem);

        next.map(|node| {
            pq.insert(node);
        });

        next = if max.count > 1 {
            max.count = max.count - 1;
            Some(max)
        } else { None }
    }
}

fn main() {
    let mut barcodes = vec![50, 60, 60, 60, 70];
    spread_numbers(&mut barcodes);

    for &number in barcodes.iter() {
        println!("{}", number);
    }
}

#[cfg(test)]
mod tests {
}

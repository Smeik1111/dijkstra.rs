// priority queue based on binary heap
// for efficient access to id with the lowest cost
#[derive(Debug)]
pub struct Heap {
    nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    id: usize,
    cost: f64,
}

impl Heap {
    pub fn new() -> Self {
        Heap { nodes: Vec::new() }
    }
    pub fn put(&mut self, id: usize, cost: f64) {
        self.nodes.push(Node { id, cost });
        self.promote(self.nodes.len() - 1);
    }
    pub fn take(&mut self) -> Option<(usize, f64)> {
        match self.nodes.len() {
            0 => None,
            1 => {
                let node = self.nodes[0].clone();
                self.nodes.pop();
                Some((node.id, node.cost))
            }
            _ => {
                let node = self.nodes[0].clone();
                self.nodes[0] = self.nodes.pop().unwrap();
                self.demote(0);
                Some((node.id, node.cost))
            }
        }
    }
    // demote more expensive parent towards the bottom of the heap
    fn demote(&mut self, mut parent: usize) {
        loop {
            match self.children(parent) {
                (Some(left), Some(right))
                    if self.nodes[right].cost < self.nodes[left].cost
                        && self.nodes[parent].cost > self.nodes[right].cost =>
                {
                    self.nodes.swap(parent, right);
                    parent = right;
                }
                (Some(left), _) if self.nodes[parent].cost > self.nodes[left].cost => {
                    self.nodes.swap(parent, left);
                    parent = left;
                }
                _ => {
                    return;
                }
            }
        }
    }
    // promote less expensive child towards the top of the heap
    fn promote(&mut self, mut child: usize) {
        loop {
            match self.parent(child) {
                Some(parent) if self.nodes[child].cost < self.nodes[parent].cost => {
                    self.nodes.swap(child, parent);
                    child = parent;
                }
                _ => {
                    return;
                }
            }
        }
    }
    fn parent(&self, child: usize) -> Option<usize> {
        if child == 0 {
            None
        } else {
            Some((child - 1) / 2)
        }
    }
    fn children(&self, parent: usize) -> (Option<usize>, Option<usize>) {
        let left = 2 * parent + 1;
        let right = left + 1;
        if left < self.nodes.len() {
            if right < self.nodes.len() {
                (Some(left), Some(right))
            } else {
                (Some(left), None)
            }
        } else {
            (None, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut heap = Heap::new();
        heap.put(1, 0.3);
        heap.put(2, 0.5);
        heap.put(3, 0.7);
        heap.put(4, 0.9);
        heap.put(5, 0.4);
        heap.put(6, 0.8);
        heap.put(7, 0.6);
        heap.put(8, 0.2);
        heap.put(9, 0.1);
        
        assert_eq!(heap.take(), Some((9, 0.1)));
        assert_eq!(heap.take(), Some((8, 0.2)));
        assert_eq!(heap.take(), Some((1, 0.3)));
        assert_eq!(heap.take(), Some((5, 0.4)));
        assert_eq!(heap.take(), Some((2, 0.5)));
        assert_eq!(heap.take(), Some((7, 0.6)));
        assert_eq!(heap.take(), Some((3, 0.7)));
        assert_eq!(heap.take(), Some((6, 0.8)));
        assert_eq!(heap.take(), Some((4, 0.9)));
        assert_eq!(heap.take(), None);
    }
}

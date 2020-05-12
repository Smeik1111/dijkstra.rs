// priority queue based on binary heap for efficient access to id with the lowest cost
#[derive(Debug)]
pub struct Heap {
    nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    cost: f64,
}

impl Node {
    fn tuple(&self) -> (usize, f64) {
        (self.id, self.cost)
    }
}

impl Heap {
    pub fn new() -> Self {
        Heap { nodes: Vec::new() }
    }
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    pub fn insert(&mut self, id: usize, cost: f64) {
        self.nodes.push(Node { id, cost });
        self.promote(self.nodes.len() - 1);
    }
    pub fn extract_min(&mut self) -> Option<(usize, f64)> {
        match self.nodes.len() {
            0 => None,
            1 => {
                let node = self.nodes.pop().unwrap();
                Some(node.tuple())
            }
            _ => {
                let node = self.nodes[0].clone();
                self.nodes[0] = self.nodes.pop().unwrap();
                self.demote(0);
                Some(node.tuple())
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
        heap.insert(1, 0.3);
        heap.insert(2, 0.5);
        heap.insert(3, 0.7);
        heap.insert(4, 0.9);
        heap.insert(5, 0.4);
        heap.insert(6, 0.8);
        heap.insert(7, 0.6);
        heap.insert(8, 0.2);
        heap.insert(9, 0.1);

        assert_eq!(heap.extract_min(), Some((9, 0.1)));
        assert_eq!(heap.extract_min(), Some((8, 0.2)));
        assert_eq!(heap.extract_min(), Some((1, 0.3)));
        assert_eq!(heap.extract_min(), Some((5, 0.4)));
        assert_eq!(heap.extract_min(), Some((2, 0.5)));
        assert_eq!(heap.extract_min(), Some((7, 0.6)));
        assert_eq!(heap.extract_min(), Some((3, 0.7)));
        assert_eq!(heap.extract_min(), Some((6, 0.8)));
        assert_eq!(heap.extract_min(), Some((4, 0.9)));
        assert_eq!(heap.extract_min(), None);
    }
}

// priority queue based on binary heap for efficient access to id with the lowest cost
#[derive(Debug)]
pub struct Heap {
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
struct Item {
    id: Id,
    cost: Cost,
}

type Id = usize;
type Cost = f64;

impl Heap {
    pub fn new() -> Self {
        Heap { items: Vec::new() }
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn insert(&mut self, id: Id, cost: Cost) {
        self.items.push(Item { id, cost });
        self.promote(self.items.len() - 1);
    }
    pub fn extract_min(&mut self) -> Option<(Id, Cost)> {
        match self.items.len() {
            0 => None,
            1 => {
                let item = self.items.pop().unwrap();
                Some((item.id, item.cost))
            }
            _ => {
                let item = self.items[0].clone();
                self.items[0] = self.items.pop().unwrap();
                self.demote(0);
                Some((item.id, item.cost))
            }
        }
    }
    // demote more expensive parent towards the bottom of the heap
    fn demote(&mut self, mut parent: Id) {
        loop {
            match self.children(parent) {
                (Some(left), Some(right))
                    if self.items[right].cost < self.items[left].cost
                        && self.items[parent].cost > self.items[right].cost =>
                {
                    self.items.swap(parent, right);
                    parent = right;
                }
                (Some(left), _) if self.items[parent].cost > self.items[left].cost => {
                    self.items.swap(parent, left);
                    parent = left;
                }
                _ => {
                    return;
                }
            }
        }
    }
    // promote less expensive child towards the top of the heap
    fn promote(&mut self, mut child: Id) {
        loop {
            match self.parent(child) {
                Some(parent) if self.items[child].cost < self.items[parent].cost => {
                    self.items.swap(child, parent);
                    child = parent;
                }
                _ => {
                    return;
                }
            }
        }
    }
    fn parent(&self, child: Id) -> Option<Id> {
        if child == 0 {
            None
        } else {
            Some((child - 1) / 2)
        }
    }
    fn children(&self, parent: Id) -> (Option<Id>, Option<Id>) {
        let left = 2 * parent + 1;
        let right = left + 1;
        if left < self.items.len() {
            if right < self.items.len() {
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

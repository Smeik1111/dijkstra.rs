// priority queue based on binary heap for efficient access to id with the lowest cost
#[derive(Debug, Default)]
pub struct Heap<Cost> {
    items: Vec<Item<Cost>>,
}

#[derive(Debug, Clone)]
struct Item<Cost> {
    id: Id,
    cost: Cost,
}

type Id = usize;

impl<Cost: Clone + PartialOrd> Heap<Cost> {
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
        if right < self.items.len() {
            (Some(left), Some(right))
        } else if left < self.items.len() {
            (Some(left), None)
        } else {
            (None, None)
        }
    }
}

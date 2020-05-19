use dijkstra::priority_queue::Heap;

#[test]
fn test() {
    let mut heap: Heap<f64> = Heap::new();
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

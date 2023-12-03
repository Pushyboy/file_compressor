pub struct Heap<T: PartialOrd> {
    items: Vec<T>,
}

impl<T: PartialOrd> Heap<T> {
    /// Instantiates a new `Heap`
    pub fn new(items: Vec<T>) -> Self {
        let mut heap = Self {items};
        heap.heapify();
        heap
    }

    fn heapify(&mut self) {
        if self.size() != 0 {
            for i in (0..=(self.size()/2 - 1)).rev() {
                self.sift_down(i)
            }
        } 
    }

    /// Insert an item into the heap
    pub fn insert(&mut self, item: T) {
        self.items.push(item);
        self.sift_up()
    }

    /// Remove an item from the heap
    pub fn remove(&mut self) -> Option<T> {
        match self.size() {
            0 => None,
            1 => self.items.pop(),
            _ => {
                let item = self.items.swap_remove(0);
                self.sift_down(0);
                Some(item)
            }
        }
     }

    // Sifts an item down the heap until the priority is correct
    fn sift_down(&mut self, mut pos: usize) {
        while pos < self.size() / 2 {
            let left = 2 * pos + 1;
            let right = left + 1;
            let mut largest = pos;

            if self.items[left] > self.items[largest] {
                largest = left;
            }

            if right < self.size() && self.items[right] > self.items[largest] {
                largest = right;
            }

            if largest == pos {
                return;
            }

            self.items.swap(pos, largest);
            pos = largest;
        }
    }

    // Sifts an item up the heap until the priority is correct
    fn sift_up(&mut self) {
        let mut pos = self.size() - 1;
        while pos > 0 {
            let parent = (pos - 1) / 2;
            if self.items[pos] <= self.items[parent] {
                return;
            }
            self.items.swap(pos, parent);
            pos = parent;
        }
    }

    /// Returns the size of the heap
    pub fn size(&self) -> usize {
        self.items.len()
    }
 }

 #[test]
 fn test_1() {
    let mut heap = Heap::new(vec![4,7,2,3,4]);
    assert_eq!(heap.remove(), Some(7));
    assert_eq!(heap.remove(), Some(4));
    assert_eq!(heap.remove(), Some(4));
    assert_eq!(heap.remove(), Some(3));
    assert_eq!(heap.remove(), Some(2));
 }

 #[test]
 fn test_empty() {
    let mut heap: Heap<i32> = Heap::new(vec![]);
    assert_eq!(heap.remove(), None);
 }

 
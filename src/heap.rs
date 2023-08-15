pub struct Heap<T: PartialOrd> {
    items: Vec<T>,
}

// All of these methods have a problem if the number is nonnegative
impl<T: PartialOrd> Heap<T> {
    pub fn new(items: Vec<T>) -> Self {
        let mut heap = Self {items};
        heap.heapify();
        heap
    }

    fn heapify(&mut self) {
        // Might break if there are 0 elements?
        for i in (0..=(self.size()/2 - 1)).rev() {
            self.sift_down(i)
        }
    }

    pub fn insert(&mut self, item: T) {
        self.items.push(item);
        self.sift_up()
    }

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

    fn sift_down(&mut self, pos: usize) {
        let mut swap_idx;
        let mut curr_idx = pos;

        while (!self.is_leaf(curr_idx)) {
            let current = &self.items[curr_idx];
            let left_child = &self.items[self.get_left_child(curr_idx).unwrap()];

            swap_idx = self.get_left_child(curr_idx).unwrap();
            if let Ok(right_pos) = self.get_right_child(curr_idx) {
                let right_child = &self.items[right_pos];
                if (right_child > left_child) {
                    swap_idx = right_pos;
                }
            }

            if(*current >= self.items[swap_idx]) {
                return;
            }

            self.items.swap(curr_idx, swap_idx);
            curr_idx = swap_idx;       
        }
    }

    fn sift_up(&mut self) {
        let mut curr_idx = self.size() - 1;
        let mut parent_index = self.get_parent(curr_idx).unwrap();

        while(parent_index >= 0) {
            let current = &self.items[curr_idx];
            let parent = &self.items[self.get_parent(curr_idx).unwrap()];

            if(current <= parent) {
                return
            } else {
                self.items.swap(curr_idx, parent_index);
                curr_idx = parent_index;
                parent_index = self.get_parent(curr_idx).unwrap();
            }
        }
    }

    fn get_parent(&self, pos: usize) -> Result<usize, &'static str> {
        if (pos > 0 && pos < self.size()) {
            Ok((pos - 1) / 2)
        } else {
            Err("Position does not have a parent")
        }
    }

    fn get_left_child(&self, pos: usize) -> Result<usize, &'static str> {
        if (pos < self.size() / 2) {
            Ok(2 * pos + 1)
        } else {
            Err("Position does not have a left child")
        }
    }

    fn get_right_child(&self, pos: usize) -> Result<usize, &'static str> {
        if (pos < (self.size() - 1) / 2) {
            Ok(2 * pos + 2)
        } else {
            Err("Position does not have right child")
        }
    }

    fn is_leaf(&self, pos: usize) -> bool {
        (pos >= self.size()/2 && pos < self.size())
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
 }
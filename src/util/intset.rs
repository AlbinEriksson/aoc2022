pub struct IntSet {
    min: usize,
    max: usize,
    items: Vec<usize>
}

impl IntSet {
    pub fn new(min: usize, max: usize) -> IntSet {
        let num_items = max - min + 1;
        let mut items: Vec<usize> = vec!();
        items.resize((num_items + usize::BITS as usize - 1) / usize::BITS as usize, 0);
        IntSet {
            min,
            max,
            items
        }
    }

    pub fn add(&mut self, mut item: usize) {
        item -= self.min;
        let item_index = item / usize::BITS as usize;
        let bit_offset = item % usize::BITS as usize;
        self.items[item_index as usize] |= 1 << bit_offset;
    }

    pub fn remove(&mut self, mut item: usize) {
        item -= self.min;
        let item_index = item / usize::BITS as usize;
        let bit_offset = item % usize::BITS as usize;
        self.items[item_index as usize] &= !(1 << bit_offset);
    }

    pub fn clear(&mut self) {
        self.items.fill(0);
    }

    pub fn contains(&self, mut item: usize) -> bool {
        item -= self.min;
        let item_index = item / usize::BITS as usize;
        let bit_offset = item % usize::BITS as usize;
        (self.items[item_index as usize] & (1 << bit_offset)) != 0
    }

    pub fn intersect(&mut self, other: &IntSet) {
        if self.min != other.min || self.max != other.max {
            panic!("Other IntSet must have the same min/max range");
        }
        for (item, other) in self.items.iter_mut().zip(other.items.iter()) {
            *item &= other;
        }
    }

    pub fn get_min(&self) -> usize {
        self.min
    }

    pub fn get_max(&self) -> usize {
        self.max
    }

    pub fn count(&self) -> usize {
        self.items
            .iter()
            .map(|item| item.count_ones() as usize)
            .sum()
    }
}

pub struct IntSet {
    min: u32,
    max: u32,
    items: Vec<usize>
}

impl IntSet {
    pub fn new(min: u32, max: u32) -> IntSet {
        let num_items = max - min + 1;
        let mut items: Vec<usize> = vec!();
        items.resize(((num_items + usize::BITS - 1) / usize::BITS) as usize, 0);
        IntSet {
            min,
            max,
            items
        }
    }

    pub fn add(&mut self, mut item: u32) {
        item -= self.min;
        let item_index = item / usize::BITS;
        let bit_offset = item % usize::BITS;
        self.items[item_index as usize] |= 1 << bit_offset;
    }

    pub fn remove(&mut self, mut item: u32) {
        item -= self.min;
        let item_index = item / usize::BITS;
        let bit_offset = item % usize::BITS;
        self.items[item_index as usize] &= !(1 << bit_offset);
    }

    pub fn clear(&mut self) {
        self.items.fill(0);
    }

    pub fn contains(&self, mut item: u32) -> bool {
        item -= self.min;
        let item_index = item / usize::BITS;
        let bit_offset = item % usize::BITS;
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

    pub fn get_min(&self) -> u32 {
        self.min
    }

    pub fn get_max(&self) -> u32 {
        self.max
    }
}

pub struct IntSet {
    min: isize,
    max: isize,
    items: Vec<usize>
}

impl IntSet
{
    pub fn new(min: isize, max: isize) -> IntSet
    {
        let num_items: usize = (max - min).try_into().unwrap();
        let num_items = (num_items + usize::BITS as usize - 1) / usize::BITS as usize;
        IntSet {
            min,
            max,
            items: vec![0; num_items]
        }
    }

    fn bit_position(&self, item: isize) -> (usize, usize) {
        let offset: usize = (item - self.min).try_into().unwrap();
        (offset / usize::BITS as usize, offset % usize::BITS as usize)
    }

    pub fn add(&mut self, item: isize) {
        let (item_index, bit_offset) = self.bit_position(item);
        self.items[item_index as usize] |= 1 << bit_offset;
    }

    pub fn remove(&mut self, item: isize) {
        if item < self.min || item >= self.max {
            return;
        }
        let (item_index, bit_offset) = self.bit_position(item);
        self.items[item_index as usize] &= !(1 << bit_offset);
    }

    pub fn clear(&mut self) {
        self.items.fill(0);
    }

    pub fn contains(&self, item: isize) -> bool {
        if item < self.min || item >= self.max {
            return false;
        }
        let (item_index, bit_offset) = self.bit_position(item);
        (self.items[item_index as usize] & (1 << bit_offset)) != 0
    }

    pub fn intersect(&mut self, other: &IntSet) {
        if self.min != other.min || self.max != other.max {
            panic!("Other IntSet must have the same min/max ranges");
        }
        for (item, other) in self.items.iter_mut().zip(other.items.iter()) {
            *item &= other;
        }
    }

    pub fn get_min(&self) -> isize {
        self.min
    }

    pub fn get_max(&self) -> isize {
        self.max
    }

    pub fn count(&self) -> usize {
        self.items
            .iter()
            .map(|item| item.count_ones() as usize)
            .sum()
    }
}

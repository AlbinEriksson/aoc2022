pub struct IntSet<const D: usize = 1> {
    min: [usize; D],
    max: [usize; D],
    strides: [usize; D],
    items: Vec<usize>
}

impl<const D: usize> IntSet<D> {
    pub fn new(min: [usize; D], max: [usize; D]) -> IntSet<D> {
        let num_items: usize = max.iter().zip(min.iter()).map(|(max, min)| max - min).product();
        let mut items: Vec<usize> = vec!();
        items.resize((num_items + usize::BITS as usize - 1) / usize::BITS as usize, 0);
        let mut strides = [1usize; D];
        for i in (1..D).rev() {
            strides[i - 1] *= strides[i] * (max[i] - min[i]);
        }
        IntSet {
            min,
            max,
            strides,
            items
        }
    }

    fn item_offset(&self, item: &[usize; D]) -> usize {
        item.iter()
            .zip(self.min.iter()).map(|(item, min)| item - min)
            .zip(self.strides).map(|(item, stride)| item * stride)
            .sum()
    }

    fn bit_position(&self, item: &[usize; D]) -> (usize, usize) {
        let offset = self.item_offset(item);
        (offset / usize::BITS as usize, offset % usize::BITS as usize)
    }

    pub fn add(&mut self, item: &[usize; D]) {
        let (item_index, bit_offset) = self.bit_position(item);
        self.items[item_index as usize] |= 1 << bit_offset;
    }

    pub fn remove(&mut self, item: &[usize; D]) {
        let (item_index, bit_offset) = self.bit_position(item);
        self.items[item_index as usize] &= !(1 << bit_offset);
    }

    pub fn clear(&mut self) {
        self.items.fill(0);
    }

    pub fn contains(&self, item: &[usize; D]) -> bool {
        let (item_index, bit_offset) = self.bit_position(item);
        (self.items[item_index as usize] & (1 << bit_offset)) != 0
    }

    pub fn intersect(&mut self, other: &IntSet) {
        if self.min.iter().zip(other.min.iter()).any(|(a, b)| a != b)
        || self.max.iter().zip(other.max.iter()).any(|(a, b)| a != b) {
            panic!("Other IntSet must have the same min/max ranges");
        }
        for (item, other) in self.items.iter_mut().zip(other.items.iter()) {
            *item &= other;
        }
    }

    pub fn get_min(&self) -> &[usize; D] {
        &self.min
    }

    pub fn get_max(&self) -> &[usize; D] {
        &self.max
    }

    pub fn count(&self) -> usize {
        self.items
            .iter()
            .map(|item| item.count_ones() as usize)
            .sum()
    }
}

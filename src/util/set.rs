pub trait Set<Item> {

    fn add(&mut self, item: Item);
    fn remove(&mut self, item: Item);
    fn clear(&mut self);
    fn contains(&self, item: Item) -> bool;
    fn intersect(&mut self, other: &Self);
}

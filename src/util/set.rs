pub trait Set {
    type Item;

    fn add(&mut self, item: Self::Item);
    fn remove(&mut self, item: Self::Item);
    fn clear(&mut self);
    fn contains(&mut self, item: Self::Item) -> bool;
    fn intersect(&mut self, other: &Self);
}

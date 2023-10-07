use super::*;

impl<'i, T> Iterator for NyarTupleView<'i, T>
where
    T: Clone,
{
    type Item = &'i T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rev { self.raw.next_back() } else { self.raw.next() }
    }
}

impl<'i, T> DoubleEndedIterator for NyarTupleView<'i, T>
where
    T: Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.rev { self.raw.next() } else { self.raw.next_back() }
    }
}
impl<'i, T> ExactSizeIterator for NyarTupleView<'i, T> where T: Clone {}

impl<'i, T> Iterator for NyarTupleEdit<'i, T>
where
    T: 'i + Clone,
{
    type Item = &'i mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rev { self.raw.next_back() } else { self.raw.next() }
    }
}
impl<'i, T> DoubleEndedIterator for NyarTupleEdit<'i, T>
where
    T: 'i + Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.rev { self.raw.next() } else { self.raw.next_back() }
    }
}
impl<'i, T> ExactSizeIterator for NyarTupleEdit<'i, T> where T: Clone {}

use super::*;

impl<'i, T> Iterator for NyarTupleView<'i, T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rev {
            if self.current < self.step {
                return None;
            }
            else {
                self.current -= self.step;
                if self.current < self.end {
                    return None;
                }
            }
        }
        else {
            self.current += self.step;
            if self.current > self.end {
                return None;
            }
        }
        self.raw.get(self.current).cloned()
    }
}

impl<'i, T> DoubleEndedIterator for NyarTupleView<'i, T>
where
    T: Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.rev {
            if self.current < self.step {
                return None;
            }
            else {
                self.current -= self.step;
                if self.current < self.start {
                    return None;
                }
            }
        }
        else {
            self.current += self.step;
            if self.current > self.start {
                return None;
            }
        }
        self.raw.get(self.current).cloned()
    }
}

impl<'i, T> Iterator for NyarTupleEdit<'i, T>
where
    T: 'i + Clone,
{
    type Item = &'i mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rev {
            if self.current < self.step {
                return None;
            }
            else {
                self.current -= self.step;
                if self.current < self.end {
                    return None;
                }
            }
        }
        else {
            self.current += self.step;
            if self.current > self.end {
                return None;
            }
        }
        self.raw.get_mut(self.current)
    }
}
impl<'i, T> DoubleEndedIterator for NyarTupleEdit<'i, T>
where
    T: 'i + Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.rev {
            if self.current < self.step {
                return None;
            }
            else {
                self.current -= self.step;
                if self.current < self.start {
                    return None;
                }
            }
        }
        else {
            self.current += self.step;
            if self.current > self.start {
                return None;
            }
        }
        self.raw.get_mut(self.current)
    }
}

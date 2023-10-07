use im::{
    vector::{Iter, IterMut},
    Vector,
};
use shredder::{
    marker::{GcDrop, GcSafe},
    Scan, Scanner,
};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
    iter::{Skip, StepBy, Take},
};

#[cfg(feature = "serde")]
mod der;
#[cfg(feature = "serde")]
mod ser;

mod iter;

#[derive(Clone)]
pub struct NyarTuple<T> {
    raw: Vector<T>,
    /// This is a compile time property
    map: BTreeMap<Box<str>, usize>,
}

pub struct NyarTupleView<'i, T> {
    raw: StepBy<Skip<Take<Iter<'i, T>>>>,
    rev: bool,
}

pub struct NyarTupleEdit<'i, T> {
    raw: StepBy<Skip<Take<IterMut<'i, T>>>>,
    rev: bool,
}

impl<T: Clone> Default for NyarTuple<T> {
    fn default() -> Self {
        Self { raw: Vector::new(), map: BTreeMap::default() }
    }
}

unsafe impl<T: GcSafe> GcSafe for NyarTuple<T> {}

unsafe impl<T: GcDrop> GcDrop for NyarTuple<T> {}

unsafe impl<T: Scan + Clone> Scan for NyarTuple<T> {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        self.raw.iter().for_each(|v| scanner.scan(v))
    }
}

impl<T: Clone + PartialEq> PartialEq<Self> for NyarTuple<T> {
    /// If the two Tuple names are different, but the value is the same, it is deemed to be equal
    fn eq(&self, other: &Self) -> bool {
        self.raw.eq(&other.raw)
    }
}

impl<T: Clone + Eq> Eq for NyarTuple<T> {}

impl<T: Clone + Hash> Hash for NyarTuple<T> {
    /// If the two Tuple names are different, but the value is the same, it will be deduplicated
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.iter().for_each(|v| v.hash(state));
    }
}

impl<T: Clone> Debug for NyarTuple<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.raw.iter()).finish()
    }
}

impl<T, U> FromIterator<U> for NyarTuple<T>
where
    U: Into<T>,
    T: Clone,
{
    fn from_iter<I>(items: I) -> Self
    where
        I: IntoIterator<Item = U>,
    {
        let mut empty = NyarTuple::default();
        for item in items.into_iter() {
            empty.raw.push_back(item.into());
        }
        empty
    }
}

impl<T: Clone> NyarTuple<T> {
    pub fn cast_offset(&self, ordinal: isize) -> Option<usize> {
        let offset = if ordinal == 0 {
            return None;
        }
        else if ordinal > 0 {
            ordinal - 1
        }
        else {
            let max = self.raw.len() as isize;
            if 0 > max + ordinal {
                return None;
            }
            max + ordinal
        };
        Some(offset as usize)
    }
    pub fn get_offset(&self, offset: usize) -> Option<&T> {
        self.raw.get(offset)
    }
    pub fn get_ordinal(&self, ordinal: isize) -> Option<&T> {
        self.get_offset(self.cast_offset(ordinal)?)
    }
    pub fn get_named(&self, name: &str) -> Option<&T> {
        let index = self.map.get(name)?;
        self.raw.get(*index)
    }
    /// Obtain an immutable view, the start and end are both closed ranges
    ///
    /// # Arguments
    ///
    /// * `head`: The ordinal number of the starting element
    /// * `tail`: The ordinal number of the ending element
    /// * `step`: Step between each element
    ///
    /// # Examples
    ///
    /// ```vk
    /// 0..=9[1: 1] = [0]
    /// 0..=9[1:-1] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    /// 0..=9[1: 2] = [0, 1]
    /// 0..=9[1:-2] = [0, 1, 2, 3, 4, 5, 6, 7, 8]
    /// 0..=9[1: 3] = [0, 1, 2]
    /// 0..=9[1:-3] = [0, 1, 2, 3, 4, 5, 6, 7]
    /// 0..=9[:: 1] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    /// 0..=9[::-1] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    /// 0..=9[:: 2] = [0, 2, 4, 6, 8]
    /// 0..=9[::-2] = [8, 6, 4, 2, 0]
    /// 0..=9[:: 3] = [0, 3, 6, 9]
    /// 0..=9[::-3] = [9, 6, 3, 0]
    /// ```
    pub fn get_range(&self, head: isize, tail: isize, step: isize) -> NyarTupleView<T> {
        let start = self.cast_offset(head).unwrap_or(self.raw.len() + 1);
        let end = self.cast_offset(tail).unwrap_or(0) + 1;
        // println!("{}: {} -> {}", self.raw.len(), start, end);
        if step > 0 {
            NyarTupleView { raw: self.raw.iter().take(end).skip(start).step_by(step.unsigned_abs()), rev: false }
        }
        else {
            NyarTupleView { raw: self.raw.iter().take(end).skip(start).step_by(step.unsigned_abs()), rev: true }
        }
    }
    pub fn append_named<I: Into<T>>(&mut self, name: &str, item: I) -> Result<(), String> {
        if self.map.contains_key(name) {
            return Err("KeyAlreadyExists".to_string());
        }
        self.raw.push_back(item.into());
        self.map.insert(Box::from(name), self.raw.len());
        Ok(())
    }
    pub fn append_one<I: Into<T>>(&mut self, item: I) {
        self.raw.push_back(item.into())
    }
    pub fn append_many<I: Iterator<Item = T>>(&mut self, items: I) {
        for item in items {
            self.append_one(item)
        }
    }
    pub fn prepend_named<I: Into<T>>(&mut self, name: &str, item: I) -> Result<(), String> {
        if self.map.contains_key(name) {
            return Err("KeyAlreadyExists".to_string());
        }
        self.raw.push_back(item.into());
        for value in self.map.values_mut() {
            *value += 1;
        }
        self.map.insert(Box::from(name), 0);
        Ok(())
    }
    pub fn prepend_one<I: Into<T>>(&mut self, item: I) {
        self.raw.push_front(item.into())
    }
    pub fn prepend_many<I: Iterator<Item = T>>(&mut self, items: I) {
        for item in items {
            self.prepend_one(item)
        }
    }
}

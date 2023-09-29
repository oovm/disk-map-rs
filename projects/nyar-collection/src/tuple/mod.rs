use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use im::Vector;
use shredder::marker::{GcDrop, GcSafe};
use shredder::{Scan, Scanner};

#[cfg(feature = "serde")]
mod ser;
#[cfg(feature = "serde")]
mod der;

#[derive(Clone)]
pub struct NyarTuple<T> {
    raw: Vector<T>,
    named: BTreeMap<Box<str>, usize>,
}

impl<T: Clone> Default for NyarTuple<T> {
    fn default() -> Self {
        Self {
            raw: Vector::new(),
            named: BTreeMap::default(),
        }
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
    fn eq(&self, other: &Self) -> bool {
        self.raw.eq(&other.raw)
    }
}

impl<T: Clone + Eq> Eq for NyarTuple<T> {}

impl<T: Clone + Hash> Hash for NyarTuple<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.iter().for_each(|v| v.hash(state))
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
        T: Clone
{
    fn from_iter<I>(items: I) -> Self
        where
            I: IntoIterator<Item=U>,
    {
        let mut empty = NyarTuple::default();
        for item in items.into_iter() {
            empty.raw.push_back(item.into());
        }
        empty
    }
}

impl<T: Clone> NyarTuple<T> {
    pub fn get_offset(&self, offset: usize) -> Option<T> {
        self.raw.get(offset).cloned()
    }
    pub fn get(&self, ordinal: isize) -> Option<T> {
        todo!()
    }
    pub fn get_range(&self, head: isize, tail: isize, step: isize) -> T {
        todo!()
    }

    pub fn append_one<I: Into<T>>(&mut self, item: I) {
        self.raw.push_back(item.into())
    }
    pub fn append_many<I: Iterator<Item=T>>(&mut self, items: I) {
        for item in items {
            self.raw.push_back(item)
        }
    }
    pub fn prepend_one<I: Into<T>>(&mut self, item: I) {
        self.raw.push_front(item.into())
    }
    pub fn prepend_many<I: Iterator<Item=T>>(&mut self, items: I) {
        for item in items {
            self.raw.push_front(item)
        }
    }
}

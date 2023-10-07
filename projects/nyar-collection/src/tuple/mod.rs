use im::Vector;
use shredder::{
    marker::{GcDrop, GcSafe},
    Scan, Scanner,
};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
};

#[cfg(feature = "serde")]
mod der;
#[cfg(feature = "serde")]
mod ser;

#[derive(Clone)]
pub struct NyarTuple<T> {
    raw: Vector<(Box<str>, T)>,
}

impl<T: Clone> Default for NyarTuple<T> {
    fn default() -> Self {
        Self { raw: Vector::new() }
    }
}

unsafe impl<T: GcSafe> GcSafe for NyarTuple<T> {}

unsafe impl<T: GcDrop> GcDrop for NyarTuple<T> {}

unsafe impl<T: Scan + Clone> Scan for NyarTuple<T> {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        self.raw.iter().for_each(|v| scanner.scan(&v.1))
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
    T: Clone,
{
    fn from_iter<I>(items: I) -> Self
    where
        I: IntoIterator<Item = U>,
    {
        let mut empty = NyarTuple::default();
        for item in items.into_iter() {
            empty.raw.push_back((Box::default(), item.into()));
        }
        empty
    }
}

impl<T: Clone> NyarTuple<T> {
    pub fn get_offset(&self, offset: isize) -> Option<T> {
        if offset < 0 { None } else { self.raw.get(offset as usize).map(|i| i.1.clone()) }
    }
    pub fn get_ordinal(&self, ordinal: isize) -> Option<T> {
        if ordinal == 0 {
            None
        }
        else if ordinal > 0 {
            self.get_offset(-ordinal)
        }
        else {
            let max = self.raw.len() as isize;
            self.get_offset(max + ordinal)
        }
    }
    pub fn get_named(&self, name: &str) -> Option<&T> {
        for (key, value) in self.raw.iter() {
            if name.eq(&**key) {
                return Some(value);
            }
        }
        return None;
    }
    pub fn get_range(&self, head: isize, tail: isize, step: isize) -> T {
        todo!()
    }
    pub fn append_named<I: Into<T>>(&mut self, name: &str, item: I) {
        for (key, value) in self.raw.iter_mut() {
            if name.eq(&**key) {
                *value = item.into();
                return;
            }
        }
        self.raw.push_back((Box::from(name), item.into()))
    }
    pub fn append_one<I: Into<T>>(&mut self, item: I) {
        self.raw.push_back((Box::default(), item.into()))
    }
    pub fn append_many<I: Iterator<Item = T>>(&mut self, items: I) {
        for item in items {
            self.append_one(item)
        }
    }
    pub fn prepend_one<I: Into<T>>(&mut self, item: I) {
        self.raw.push_front((Box::default(), item.into()))
    }
    pub fn prepend_many<I: Iterator<Item = T>>(&mut self, items: I) {
        for item in items {
            self.prepend_one(item)
        }
    }
}

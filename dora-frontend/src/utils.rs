use parking_lot::{Mutex, MutexGuard, RwLock};

use std::ops::Index;
use std::sync::Arc;

pub struct GrowableVec<T> {
    elements: Mutex<Vec<Arc<T>>>,
}

impl<T> GrowableVec<T> {
    pub fn new() -> GrowableVec<T> {
        GrowableVec {
            elements: Mutex::new(Vec::new()),
        }
    }

    pub fn lock(&self) -> MutexGuard<Vec<Arc<T>>> {
        self.elements.lock()
    }

    pub fn push(&self, val: T) -> usize {
        let mut elements = self.elements.lock();
        let idx = elements.len();
        elements.push(Arc::new(val));

        idx
    }

    pub fn len(&self) -> usize {
        let elements = self.elements.lock();
        elements.len()
    }

    pub fn iter(&self) -> GrowableVecIter<T> {
        GrowableVecIter { vec: self, idx: 0 }
    }

    pub fn idx_usize(&self, idx: usize) -> Arc<T> {
        let elements = self.elements.lock();
        elements[idx].clone()
    }
}

impl<T: Id> GrowableVec<RwLock<T>> {
    pub fn idx(&self, idx: T::IdType) -> Arc<RwLock<T>> {
        self.idx_usize(T::id_to_usize(idx))
    }
}

pub struct GrowableVecIter<'a, T>
where
    T: 'a,
{
    vec: &'a GrowableVec<T>,
    idx: usize,
}

impl<'a, T> Iterator for GrowableVecIter<'a, T> {
    type Item = Arc<T>;

    fn next(&mut self) -> Option<Arc<T>> {
        let length = self.vec.len();

        if self.idx < length {
            let idx = self.idx;
            self.idx += 1;
            Some(self.vec.idx_usize(idx))
        } else {
            None
        }
    }
}

pub trait Id {
    type IdType: Copy + Clone;

    fn usize_to_id(value: usize) -> Self::IdType;
    fn id_to_usize(value: Self::IdType) -> usize;
    fn store_id(value: &mut Self, id: Self::IdType);
}

type ElementType<T> = Arc<RwLock<T>>;

pub struct MutableVec<T: Id> {
    elements: Vec<Arc<RwLock<T>>>,
}

impl<T: Id> MutableVec<T> {
    pub fn new() -> MutableVec<T> {
        MutableVec {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, mut value: T) -> T::IdType {
        let id = T::usize_to_id(self.elements.len());
        T::store_id(&mut value, id);
        self.elements.push(Arc::new(RwLock::new(value)));
        id
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn idx(&self, idx: T::IdType) -> ElementType<T> {
        self.elements[T::id_to_usize(idx)].clone()
    }

    pub fn iter(&self) -> SharedVecIter<T> {
        SharedVecIter {
            vec: &self.elements,
            idx: 0,
            len: self.elements.len(),
        }
    }
}

impl<T: Id> Index<T::IdType> for MutableVec<T> {
    type Output = ElementType<T>;
    fn index(&self, index: T::IdType) -> &ElementType<T> {
        &self.elements[T::id_to_usize(index)]
    }
}

pub struct SharedVecIter<'a, T>
where
    T: 'a,
{
    vec: &'a Vec<ElementType<T>>,
    idx: usize,
    len: usize,
}

impl<'a, T> Iterator for SharedVecIter<'a, T> {
    type Item = ElementType<T>;

    fn next(&mut self) -> Option<ElementType<T>> {
        if self.idx < self.len {
            let idx = self.idx;
            self.idx += 1;
            Some(self.vec[idx].clone())
        } else {
            None
        }
    }
}

#[test]
fn test_push() {
    let vec: GrowableVec<Mutex<i32>> = GrowableVec::new();

    {
        vec.push(Mutex::new(1));
        vec.push(Mutex::new(2));

        let elem = vec.idx_usize(1);
        let mut elem = elem.lock();

        *elem = 10;
        for i in 3..8 {
            vec.push(Mutex::new(i));
        }
    }

    assert_eq!(7, vec.len());
}

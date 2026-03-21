use std::alloc::{Layout, alloc, dealloc};
use std::fmt;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::Index;
use std::ptr;

struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>, // _marker
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            ptr: ptr::null_mut(),
            len: 0,
            cap: 0,
            _marker: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Vec::new();
        }
        let layout = Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { alloc(layout) as *mut T };
        Vec {
            ptr,
            len: 0,
            cap: capacity,
            _marker: PhantomData,
        }
    }

    fn grow(&mut self) {
        let (new_cap, layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.cap * 2;
            (new_cap, Layout::array::<T>(new_cap).unwrap())
        };

        let new_ptr = unsafe { alloc(layout) as *mut T };

        if !self.ptr.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                dealloc(self.ptr as *mut u8, old_layout);
            }
        }

        self.ptr = new_ptr;
        self.cap = new_cap;
    }

    pub fn push(&mut self, val: T) {
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            ptr::write(self.ptr.add(self.len), val);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        unsafe { Some(ptr::read(self.ptr.add(self.len))) }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            unsafe { Some(&mut *self.ptr.add(index)) }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            while self.pop().is_some() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { dealloc(self.ptr as *mut u8, layout) };
        }
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for Vec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).expect("index out of bounds")
    }
}

pub struct IntoIter<T> {
    vec: Vec<T>,
    index: usize,
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            vec: self,
            index: 0,
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.index < self.vec.len {
            let val = unsafe { ptr::read(self.vec.ptr.add(self.index)) };
            self.index += 1;
            Some(val)
        } else {
            None
        }
    }
}

impl<T: Clone> Vec<T> {
    pub fn from_slice(slice: &[T]) -> Self {
        let mut vec = Vec::with_capacity(slice.len());
        for item in slice {
            vec.push(item.clone());
        }
        vec
    }
}

impl<T: fmt::Debug> fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

pub struct Iter<'a, T> {
    ptr: *const T,
    len: usize,
    index: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let val = unsafe { &*self.ptr.add(self.index) };
            self.index += 1;
            Some(val)
        } else {
            None
        }
    }
}

impl<T> Vec<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            ptr: self.ptr,
            len: self.len,
            index: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> FromIterator<T> for Vec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for item in iter {
            vec.push(item);
        }
        vec
    }
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    println!("Created empty vec: len={}, cap={}", v.len(), v.capacity());

    v.push(1);
    v.push(2);
    v.push(3);
    println!("After push 1,2,3: {:?}", v);

    v.push(4);
    println!("After push 4: len={}, cap={}", v.len(), v.capacity());

    if let Some(val) = v.get(1) {
        println!("v[1] = {}", val);
    }

    if let Some(val) = v.pop() {
        println!("Popped: {}", val);
    }
    println!("After pop: {:?}", v);

    println!("Iterating:");
    for val in &v {
        println!("  {}", val);
    }

    let sum: i32 = v.into_iter().sum();
    println!("Sum via into_iter: {}", sum);

    let v2: Vec<String> = Vec::from_slice(&["hello".to_string(), "world".to_string()]);
    println!("From slice: {:?}", v2);

    println!("\nAll tests passed!");
}

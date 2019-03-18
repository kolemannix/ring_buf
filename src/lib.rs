use std::marker::PhantomData;

// To put a slice on the heap, we just box it!
pub struct Ring<T, B = Box<[Option<T>]>> {
    buffer: B,
    read_idx: usize,
    write_idx: usize,
    capacity: usize,
    t: PhantomData<T>,
}

/// Public methods on Ring
impl<T> Ring<T> {
    fn looped(&self, index: usize) -> usize {
        index % self.capacity
    }   
    pub fn new(capacity: usize) -> Self {
        let mut buffer_vec: Vec<Option<T>> = Vec::with_capacity(capacity);
        unsafe {
            buffer_vec.set_len(capacity);
        }
        Ring {
            capacity: capacity,
            buffer: buffer_vec.into_boxed_slice(),
            read_idx: 0,
            write_idx: 0,
            t: PhantomData
        }
    }

    pub fn push(&mut self, elem: T) -> () {
        self.buffer[self.write_idx] = Some(elem);
        self.write_idx = self.looped(self.write_idx + 1);
        ()
    }
    pub fn pop(&mut self) -> Option<T> {
        let replacement = None;
        let result = match self.buffer.get(self.read_idx) {
            Some(_) => {
                unsafe {
                    let elem = std::mem::replace(self.buffer.get_unchecked_mut(self.read_idx), replacement);
                    elem
                }
            },
            None => {
                None   
            }
        };
        self.read_idx = self.looped(self.read_idx + 1);
        result
    }
}

impl<T> From<Vec<T>> for Ring<T> {
    fn from(vec: Vec<T>) -> Ring<T> {
        let len = vec.len();
        let vec_lifted: Vec<Option<T>> = vec.into_iter().map(|t| Some(t)).collect();
        assert!(len > 0);
        Ring {
            capacity: len,
            buffer: vec_lifted.into_boxed_slice(),
            read_idx: 0,
            write_idx: 0, 
            t: PhantomData,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_new() {
        let mut ring: Ring<i32> = Ring::new(3);
        assert_eq!(ring.pop(), None);
    }
    #[test]
    fn push_all_pop_all() {
        let mut ring: Ring<i32> = Ring::new(10);
        ring.push(10);
        ring.push(20);
        ring.push(30);
        let ten = ring.pop();
        let twenty = ring.pop();
        let thirty = ring.pop();
        assert_eq!(ten.unwrap(), 10);
        assert_eq!(twenty.unwrap(), 20);
        assert_eq!(thirty.unwrap(), 30);
    }
    #[test]
    fn push_pop_alternating() {
        let mut ring: Ring<i32> = Ring::new(3);
        ring.push(10);
        let ten = ring.pop();
        assert_eq!(ten.unwrap(), 10);

        ring.push(20);
        let twenty = ring.pop().unwrap();
        assert_eq!(twenty, 20);
    }
    #[test]
    fn push_past_capacity() {
        let mut ring: Ring<i32> = Ring::new(3);
        for i in 1 .. 10 {
            ring.push(i);
        }
        let first = ring.pop().unwrap();
        let second = ring.pop().unwrap();
        let third = ring.pop().unwrap();

        assert_eq!(first, 7);
        assert_eq!(second, 8);
        assert_eq!(third, 9);
    }

    #[test]
    fn from_vec() {
        let mut ring: Ring<i32> = vec![1, 2, 3].into();
        ring.push(4);
        // The push Overwrites the first element, assumed to be the 'oldest'
        assert_eq!(ring.pop().unwrap(), 4);
        assert_eq!(ring.pop().unwrap(), 2);
        assert_eq!(ring.pop().unwrap(), 3);
        assert_eq!(ring.pop(), None);
    }
}

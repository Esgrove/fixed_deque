#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::collections::VecDeque;
use std::collections::vec_deque::Drain;
use std::ops::{Index, IndexMut, RangeBounds};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A fixed size `VecDeque` to match Python Deque.
///
/// Once a deque is full,
/// when a new item is added,
/// an element from the opposite end is popped and returned.
/// <https://docs.python.org/3/library/collections.html#collections.deque>
#[derive(Debug, Default, Clone)]
pub struct Deque<T> {
    deque: VecDeque<T>,
    maxlen: usize,
}

impl<T> Deque<T> {
    /// Creates a new empty Deque with a given maximum length.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = Deque::new(3);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// deque.push_back(3);
    /// deque.push_back(4);
    /// assert_eq!(deque.len(), 3);
    /// assert_eq!(deque.get(0), Some(&2));
    /// ```
    #[must_use]
    pub fn new(maxlen: usize) -> Self {
        Self {
            deque: VecDeque::with_capacity(maxlen),
            maxlen,
        }
    }

    /// Creates a new Deque from a given single value and maximum length.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<i32> = Deque::from(1, 3);
    /// assert_eq!(deque.len(), 1);
    /// assert_eq!(deque.get(0), Some(&1));
    /// ```
    pub fn from(value: T, maxlen: usize) -> Self {
        Self {
            deque: VecDeque::from([value]),
            maxlen,
        }
    }

    /// Creates a new Deque from an existing `Vec` with a given maximum length.
    /// If the given vector is larger than the maximum length,
    /// only the first `maxlen` elements are used.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<i32> = Deque::from_vec(vec![1, 2, 3], 3);
    /// assert_eq!(deque.len(), 3);
    /// ```
    #[must_use]
    pub fn from_vec(mut vec: Vec<T>, maxlen: usize) -> Self {
        vec.truncate(maxlen);
        Self {
            deque: VecDeque::from(vec),
            maxlen,
        }
    }

    /// Creates a new Deque from an existing `VecDeque` with a given maximum length.
    /// If the given `VecDeque` is larger than the maximum length,
    /// only the first `maxlen` elements are used.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::VecDeque;
    /// use fixed_deque::Deque;
    ///
    /// let vec_deque: VecDeque<i32> = VecDeque::from(vec![1, 2, 3]);
    /// let deque: Deque<i32> = Deque::from_vec_deque(vec_deque, 3);
    /// assert_eq!(deque.len(), 3);
    /// ```
    #[must_use]
    pub fn from_vec_deque(mut deque: VecDeque<T>, maxlen: usize) -> Self {
        deque.truncate(maxlen);
        Self { deque, maxlen }
    }

    /// Returns the maximum length of the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<f64> = (vec![1.0, 2.0, 3.0], 5).into();
    /// assert_eq!(deque.len(), 3);
    /// assert_eq!(deque.maxlen(), 5);
    /// ```
    #[must_use]
    pub const fn maxlen(&self) -> usize {
        self.maxlen
    }

    /// Clears all elements from the deque, making it empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
    /// assert_eq!(deque.len(), 3);
    /// deque.clear();
    /// assert_eq!(deque.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.deque.clear();
    }

    /// Returns `true` if the deque contains an element equal to the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<&str> = (vec!["1", "2", "3"], 3).into();
    /// assert!(deque.contains(&"2"));
    /// assert!(!deque.contains(&"4"));
    /// ```
    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.deque.contains(value)
    }

    /// Add an element to the back of the Deque.
    /// If the Deque exceeds its maximum length,
    /// the front element is popped and returned.
    /// Otherwise, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = Deque::new(3);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// deque.push_back(3);
    /// assert_eq!(deque.len(), 3);
    /// let overflow = deque.push_back(4);
    /// assert_eq!(overflow, Some(1));
    /// assert_eq!(deque.len(), 3);
    /// ```
    pub fn push_back(&mut self, value: T) -> Option<T> {
        if self.deque.len() == self.maxlen {
            // If at max capacity, pop the front element before pushing
            let popped = self.deque.pop_front();
            self.deque.push_back(value);
            popped
        } else {
            self.deque.push_back(value);
            None
        }
    }

    /// Prepends an element to the deque.
    /// If the Deque exceeds its maximum length,
    /// the back element is popped and returned.
    /// Otherwise, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque = Deque::new(2);
    /// deque.push_front(1);
    /// deque.push_front(2);
    /// assert_eq!(deque.front(), Some(&2));
    /// deque.push_front(3);
    /// assert_eq!(deque.front(), Some(&3));
    /// assert_eq!(deque.len(), 2);
    /// ```
    pub fn push_front(&mut self, value: T) -> Option<T> {
        if self.deque.len() == self.maxlen {
            // If at max capacity, pop the back element before pushing
            let popped = self.deque.pop_back();
            self.deque.push_front(value);
            popped
        } else {
            self.deque.push_front(value);
            None
        }
    }

    /// Removes the first element and returns it,
    /// or `None` if the deque is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque = Deque::new(3);
    /// deque.push_back(1);
    /// deque.push_back(2);
    ///
    /// assert_eq!(deque.pop_front(), Some(1));
    /// assert_eq!(deque.pop_front(), Some(2));
    /// assert_eq!(deque.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.deque.pop_front()
    }

    /// Removes the last element from the deque and returns it,
    /// or `None` if it is empty.
    pub fn pop_back(&mut self) -> Option<T> {
        self.deque.pop_back()
    }

    /// Returns the number of elements in the Deque.
    #[must_use]
    pub fn len(&self) -> usize {
        self.deque.len()
    }

    /// Returns whether the Deque is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }

    /// Provides a reference to the back element, or `None` if the deque is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = Deque::new(3);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.back(), Some(&2));
    /// ```
    #[must_use]
    pub fn back(&self) -> Option<&T> {
        self.deque.back()
    }

    /// Provides a mutable reference to the back element, or `None` if the deque is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
    /// if let Some(back) = deque.back_mut() {
    ///     *back = 20;
    /// }
    /// assert_eq!(deque.back(), Some(&20));
    /// ```
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.deque.back_mut()
    }

    /// Provides a reference to the front element, or `None` if the deque is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
    /// assert_eq!(deque.front(), Some(&1));
    ///
    /// let empty_deque: Deque<i32> = Deque::new(3);
    /// assert_eq!(empty_deque.front(), None);
    /// ```
    #[must_use]
    pub fn front(&self) -> Option<&T> {
        self.deque.front()
    }

    /// Provides a mutable reference to the front element, or `None` if the deque is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
    /// if let Some(front) = deque.front_mut() {
    ///     *front = 10;
    /// }
    /// assert_eq!(deque.front(), Some(&10));
    /// ```
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.deque.front_mut()
    }

    /// Returns an immutable reference to the element at the given index.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    /// let mut deque: Deque<i32> = Deque::new(3);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.get(1), Some(&2));
    /// ```
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.deque.get(index)
    }

    /// Returns a mutable reference to the element at the given index.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = Deque::new(3);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// if let Some(value) = deque.get_mut(1) {
    ///     *value = 42;
    /// }
    /// assert_eq!(deque.get(1), Some(&42));
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.deque.get_mut(index)
    }

    /// Returns an iterator over all elements except the last one.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = Deque::new(3);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// deque.push_back(3);
    /// let mut iter = deque.iter_except_last();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_except_last(&self) -> impl Iterator<Item = &T> {
        self.deque.iter().take(self.deque.len().saturating_sub(1))
    }

    /// Returns a front-to-back iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque = Deque::new(3);
    /// deque.push_back(5);
    /// deque.push_back(3);
    /// deque.push_back(4);
    /// let b: &[_] = &[&5, &3, &4];
    /// let c: Vec<&i32> = deque.iter().collect();
    /// assert_eq!(&c[..], b);
    /// ```
    #[must_use]
    pub fn iter(&'_ self) -> std::collections::vec_deque::Iter<'_, T> {
        self.deque.iter()
    }

    /// Returns a front-to-back mutable iterator.
    ///
    /// This allows modifying each element in the deque in place.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque = Deque::new(3);
    /// deque.push_back(5);
    /// deque.push_back(3);
    /// deque.push_back(4);
    ///
    /// for value in deque.iter_mut() {
    ///     *value *= 2;
    /// }
    ///
    /// let b: &[_] = &[10, 6, 8];
    /// let c: Vec<i32> = deque.iter().copied().collect();
    /// assert_eq!(&c[..], b);
    /// ```
    #[must_use]
    pub fn iter_mut(&'_ mut self) -> std::collections::vec_deque::IterMut<'_, T> {
        self.deque.iter_mut()
    }

    /// Returns the number of elements the deque can hold without reallocating.
    /// If the number is larger than the max size,
    /// returns the max number of elements instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<i32> = Deque::new(10);
    /// assert!(deque.capacity() >= 10);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.deque.capacity().min(self.maxlen)
    }

    /// Returns `true` if the deque is full (i.e., `len() == maxlen()`).
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = Deque::new(3);
    /// assert!(!deque.is_full());
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// deque.push_back(3);
    /// assert!(deque.is_full());
    /// ```
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.deque.len() == self.maxlen
    }

    /// Returns the remaining capacity before the deque is full.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = Deque::new(5);
    /// assert_eq!(deque.remaining_capacity(), 5);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.remaining_capacity(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn remaining_capacity(&self) -> usize {
        self.maxlen.saturating_sub(self.deque.len())
    }

    /// Swaps elements at indices `i` and `j`.
    ///
    /// `i` and `j` may be equal.
    ///
    /// Element at index 0 is the front of the queue.
    ///
    /// # Panics
    ///
    /// Panics if either index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
    /// deque.swap(0, 2);
    /// assert_eq!(deque.get(0), Some(&3));
    /// assert_eq!(deque.get(2), Some(&1));
    /// ```
    pub fn swap(&mut self, i: usize, j: usize) {
        self.deque.swap(i, j);
    }

    /// Shortens the deque, keeping the first `len` elements and dropping the rest.
    ///
    /// If `len` is greater than or equal to the deque's current length, this has no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
    /// deque.truncate(2);
    /// assert_eq!(deque.len(), 2);
    /// assert_eq!(deque.get(0), Some(&1));
    /// assert_eq!(deque.get(1), Some(&2));
    /// ```
    pub fn truncate(&mut self, len: usize) {
        self.deque.truncate(len);
    }

    /// Removes and returns the element at `index` from the deque.
    /// Whichever end is closer to the removal point will be moved to make
    /// room, and all the affected elements will be moved to new positions.
    /// Returns `None` if `index` is out of bounds.
    ///
    /// Element at index 0 is the front of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
    /// assert_eq!(deque.remove(1), Some(2));
    /// assert_eq!(deque.len(), 2);
    /// assert_eq!(deque.get(0), Some(&1));
    /// assert_eq!(deque.get(1), Some(&3));
    /// ```
    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.deque.remove(index)
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&e)` returns `false`.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
    /// deque.retain(|&x| x % 2 == 0);
    /// assert_eq!(deque.len(), 2);
    /// assert_eq!(deque.get(0), Some(&2));
    /// assert_eq!(deque.get(1), Some(&4));
    /// ```
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.deque.retain(f);
    }

    /// Retains only the elements specified by the predicate, passing a mutable reference.
    ///
    /// In other words, remove all elements `e` for which `f(&mut e)` returns `false`.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
    /// deque.retain_mut(|x| {
    ///     if *x % 2 == 0 {
    ///         *x *= 10;
    ///         true
    ///     } else {
    ///         false
    ///     }
    /// });
    /// assert_eq!(deque.len(), 2);
    /// assert_eq!(deque.get(0), Some(&20));
    /// assert_eq!(deque.get(1), Some(&40));
    /// ```
    pub fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.deque.retain_mut(f);
    }

    /// Creates a draining iterator that removes the specified range in the deque
    /// and yields the removed items.
    ///
    /// When the iterator is dropped, all elements in the range are removed
    /// from the deque, even if the iterator was not fully consumed. If the
    /// iterator is not dropped (with `mem::forget` for example), the deque
    /// may have lost and leaked elements.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point or if
    /// the end point is greater than the length of the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
    /// let drained: Vec<i32> = deque.drain(1..4).collect();
    /// assert_eq!(drained, vec![2, 3, 4]);
    /// assert_eq!(deque.len(), 2);
    /// assert_eq!(deque.get(0), Some(&1));
    /// assert_eq!(deque.get(1), Some(&5));
    /// ```
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T>
    where
        R: RangeBounds<usize>,
    {
        self.deque.drain(range)
    }

    /// Rotates the deque `n` places to the left.
    ///
    /// Equivalently, this rotates the front of the deque towards the back.
    ///
    /// # Panics
    ///
    /// If `n` is greater than `len()`. To avoid panic, call with `n % len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
    /// deque.rotate_left(2);
    /// assert_eq!(deque.get(0), Some(&3));
    /// assert_eq!(deque.get(4), Some(&2));
    /// ```
    pub fn rotate_left(&mut self, n: usize) {
        self.deque.rotate_left(n);
    }

    /// Rotates the deque `n` places to the right.
    ///
    /// Equivalently, this rotates the back of the deque towards the front.
    ///
    /// # Panics
    ///
    /// If `n` is greater than `len()`. To avoid panic, call with `n % len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
    /// deque.rotate_right(2);
    /// assert_eq!(deque.get(0), Some(&4));
    /// assert_eq!(deque.get(4), Some(&3));
    /// ```
    pub fn rotate_right(&mut self, n: usize) {
        self.deque.rotate_right(n);
    }

    /// Returns a pair of slices which contain, in order, the contents of the deque.
    ///
    /// If `make_contiguous` was previously called,
    /// all elements will be in the first slice and the second slice will be empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = Deque::new(5);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// deque.push_back(3);
    /// let (front, back) = deque.as_slices();
    /// // The slices together contain all elements
    /// assert_eq!(front.len() + back.len(), 3);
    /// ```
    #[must_use]
    pub fn as_slices(&self) -> (&[T], &[T]) {
        self.deque.as_slices()
    }

    /// Returns a pair of mutable slices which contain, in order, the contents of the deque.
    ///
    /// If `make_contiguous` was previously called,
    /// all elements will be in the first slice and the second slice will be empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
    /// let (front, back) = deque.as_mut_slices();
    /// if let Some(first) = front.first_mut() {
    ///     *first = 10;
    /// }
    /// assert_eq!(deque.get(0), Some(&10));
    /// ```
    #[must_use]
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        self.deque.as_mut_slices()
    }

    /// Rearranges the internal storage of this deque so it is one contiguous slice,
    /// which is then returned.
    ///
    /// This method does not allocate and does not change the order of the elements.
    /// After calling this method, `as_slices` will return a single contiguous slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = Deque::new(5);
    /// deque.push_back(2);
    /// deque.push_back(1);
    /// deque.push_front(3);
    ///
    /// // The deque may be split across the ring buffer
    /// let slice = deque.make_contiguous();
    /// assert_eq!(slice, &[3, 2, 1]);
    ///
    /// // Now as_slices returns a single slice
    /// let (front, back) = deque.as_slices();
    /// assert_eq!(front, &[3, 2, 1]);
    /// assert!(back.is_empty());
    /// ```
    pub fn make_contiguous(&mut self) -> &mut [T] {
        self.deque.make_contiguous()
    }

    /// Binary searches the sorted deque for a given element.
    ///
    /// If the value is found then `Result::Ok` is returned,
    /// containing the index of the matching element.
    /// If there are multiple matches, then any one of the matches could be returned.
    ///
    /// If the value is not found then `Result::Err` is returned,
    /// containing the index where a matching element could be inserted while maintaining sorted order.
    ///
    /// # Errors
    ///
    /// Returns `Err(index)` if the value is not found, where `index` is the
    /// position where the value could be inserted to maintain sorted order.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
    /// assert_eq!(deque.binary_search(&3), Ok(2));
    /// assert_eq!(deque.binary_search(&6), Err(5));
    /// assert_eq!(deque.binary_search(&0), Err(0));
    /// ```
    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.deque.binary_search(x)
    }

    /// Binary searches the sorted deque with a comparator function.
    ///
    /// The comparator function should return an ordering for the partitioning
    /// of the deque. For a given element, the comparator function should return:
    /// - `Ordering::Less` if the element should come before the target
    /// - `Ordering::Equal` if the element matches the target
    /// - `Ordering::Greater` if the element should come after the target
    ///
    /// If the value is found then `Result::Ok` is returned,
    /// containing the index of the matching element.
    /// If there are multiple matches, then any one of the matches could be returned.
    ///
    /// If the value is not found then `Result::Err` is returned,
    /// containing the index where a matching element could be inserted while maintaining sorted order.
    ///
    /// # Errors
    ///
    /// Returns `Err(index)` if the value is not found, where `index` is the
    /// position where the value could be inserted to maintain sorted order.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
    /// assert_eq!(deque.binary_search_by(|x| x.cmp(&3)), Ok(2));
    /// assert_eq!(deque.binary_search_by(|x| x.cmp(&6)), Err(5));
    /// ```
    pub fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> std::cmp::Ordering,
    {
        self.deque.binary_search_by(f)
    }

    /// Binary searches the sorted deque with a key extraction function.
    ///
    /// Assumes that the deque is sorted by the key.
    ///
    /// If the value is found then `Result::Ok` is returned,
    /// containing the index of the matching element.
    /// If there are multiple matches, then any one of the matches could be returned.
    ///
    /// If the value is not found then `Result::Err` is returned,
    /// containing the index where a matching element could be inserted while maintaining sorted order.
    ///
    /// # Errors
    ///
    /// Returns `Err(index)` if the value is not found, where `index` is the
    /// position where the value could be inserted to maintain sorted order.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<(i32, &str)> = (vec![(1, "a"), (2, "b"), (3, "c")], 10).into();
    /// assert_eq!(deque.binary_search_by_key(&2, |&(k, _)| k), Ok(1));
    /// assert_eq!(deque.binary_search_by_key(&4, |&(k, _)| k), Err(3));
    /// ```
    pub fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        self.deque.binary_search_by_key(b, f)
    }

    /// Counts the number of elements equal to the given value.
    ///
    /// This is equivalent to Python's `deque.count(x)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<i32> = (vec![1, 2, 2, 3, 2, 4], 10).into();
    /// assert_eq!(deque.count(&2), 3);
    /// assert_eq!(deque.count(&5), 0);
    /// ```
    pub fn count(&self, value: &T) -> usize
    where
        T: PartialEq,
    {
        self.deque.iter().filter(|&x| x == value).count()
    }

    /// Returns the index of the first element equal to the given value,
    /// or `None` if not found.
    ///
    /// This is similar to Python's `deque.index(x)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<i32> = (vec![1, 2, 3, 2, 4], 10).into();
    /// assert_eq!(deque.index(&2), Some(1));
    /// assert_eq!(deque.index(&5), None);
    /// ```
    pub fn index(&self, value: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        self.deque.iter().position(|x| x == value)
    }

    /// Returns the index of the first element equal to the given value
    /// within the range `[start, end)`, or `None` if not found.
    ///
    /// This is similar to Python's `deque.index(x, start, stop)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<i32> = (vec![1, 2, 3, 2, 4], 10).into();
    /// assert_eq!(deque.index_range(&2, 0, 5), Some(1));
    /// assert_eq!(deque.index_range(&2, 2, 5), Some(3));
    /// assert_eq!(deque.index_range(&2, 4, 5), None);
    /// ```
    pub fn index_range(&self, value: &T, start: usize, end: usize) -> Option<usize>
    where
        T: PartialEq,
    {
        let end = end.min(self.deque.len());
        if start >= end {
            return None;
        }
        self.deque
            .iter()
            .skip(start)
            .take(end - start)
            .position(|x| x == value)
            .map(|pos| pos + start)
    }

    /// Extends the front of the deque with the contents of an iterator.
    ///
    /// Elements are prepended one at a time,
    /// so the final order is reversed relative to the iterator (matching Python's `deque.extendleft()` behavior).
    ///
    /// If the deque would exceed `maxlen`, elements are removed from the back.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![4, 5], 5).into();
    /// deque.extend_front([1, 2, 3]);
    /// // Elements are prepended in reverse order: 3, 2, 1, then 4, 5
    /// assert_eq!(deque.get(0), Some(&3));
    /// assert_eq!(deque.get(1), Some(&2));
    /// assert_eq!(deque.get(2), Some(&1));
    /// assert_eq!(deque.get(3), Some(&4));
    /// assert_eq!(deque.get(4), Some(&5));
    /// ```
    pub fn extend_front<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter {
            self.push_front(item);
        }
    }

    /// Reverses the order of elements in the deque in-place.
    ///
    /// This is equivalent to Python's `deque.reverse()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
    /// deque.reverse();
    /// assert_eq!(deque.get(0), Some(&5));
    /// assert_eq!(deque.get(4), Some(&1));
    /// ```
    pub fn reverse(&mut self) {
        let slice = self.deque.make_contiguous();
        slice.reverse();
    }

    /// Removes the first occurrence of a value from the deque.
    ///
    /// Returns the removed value if found, `None` otherwise.
    ///
    /// This is similar to Python's `deque.remove(value)`,
    /// but returns an Option instead of raising an exception when the value is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 2, 3, 2, 4], 10).into();
    /// assert_eq!(deque.remove_first(&2), Some(2));
    /// assert_eq!(deque.len(), 4);
    /// assert_eq!(deque.get(0), Some(&1));
    /// assert_eq!(deque.get(1), Some(&3)); // First 2 was removed
    /// assert_eq!(deque.get(2), Some(&2)); // Second 2 is still there
    ///
    /// assert_eq!(deque.remove_first(&5), None); // Returns None if not found
    /// ```
    pub fn remove_first(&mut self, value: &T) -> Option<T>
    where
        T: PartialEq,
    {
        if let Some(pos) = self.index(value) {
            self.deque.remove(pos)
        } else {
            None
        }
    }

    /// Inserts an element at the given index, shifting all elements after it to the right.
    ///
    /// If the deque is full, the last (back) element is dropped to make room for the
    /// new element, and the dropped element is returned. Otherwise, `None` is returned.
    ///
    /// # Panics
    ///
    /// Panics if `index > len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = (vec![1, 3, 4], 5).into();
    /// assert_eq!(deque.insert(1, 2), None);
    /// assert_eq!(deque.get(0), Some(&1));
    /// assert_eq!(deque.get(1), Some(&2));
    /// assert_eq!(deque.get(2), Some(&3));
    /// assert_eq!(deque.get(3), Some(&4));
    /// ```
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// // When full, the last element is dropped
    /// let mut deque: Deque<i32> = (vec![1, 2, 3], 3).into();
    /// let dropped = deque.insert(1, 10);
    /// assert_eq!(dropped, Some(3)); // 3 was at the back and got dropped
    /// assert_eq!(deque.iter().copied().collect::<Vec<_>>(), vec![1, 10, 2]);
    /// ```
    pub fn insert(&mut self, index: usize, value: T) -> Option<T> {
        if self.deque.len() == self.maxlen {
            // If at max capacity, pop the back element before inserting
            let popped = self.deque.pop_back();
            self.deque.insert(index, value);
            popped
        } else {
            self.deque.insert(index, value);
            None
        }
    }
}

// Implement From for single value.
impl<T> From<(T, usize)> for Deque<T> {
    /// Creates a new Deque from a single value and a maximum length.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<i32> = (1, 3).into();
    /// assert_eq!(deque.len(), 1);
    /// assert_eq!(deque.get(0), Some(&1));
    /// ```
    fn from((value, maxlen): (T, usize)) -> Self {
        Self {
            deque: VecDeque::from([value]),
            maxlen,
        }
    }
}

// Implement From for arrays.
impl<T, const N: usize> From<([T; N], usize)> for Deque<T> {
    /// Creates a new Deque from an array and a maximum length.
    /// If the array is larger than the maximum length,
    /// only the first `maxlen` elements are used.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<i32> = ([1, 2, 3], 16).into();
    /// assert_eq!(deque.len(), 3);
    /// assert_eq!(deque.maxlen(), 16);
    /// assert_eq!(deque.get(0), Some(&1));
    /// assert_eq!(deque.get(2), Some(&3));
    ///
    /// let deque: Deque<f64> = ([1.0, 2.0, 3.0], 2).into();
    /// assert_eq!(deque.len(), 2);
    /// assert_eq!(deque.maxlen(), 2);
    /// assert_eq!(deque.get(0), Some(&1.0));
    /// assert_eq!(deque.get(1), Some(&2.0));
    /// ```
    fn from((array, maxlen): ([T; N], usize)) -> Self {
        let deque = if N > maxlen {
            // If the array size exceeds maxlen, take only the first `maxlen` elements.
            VecDeque::from(array.into_iter().take(maxlen).collect::<Vec<_>>())
        } else {
            VecDeque::from(array)
        };
        Self { deque, maxlen }
    }
}

// Implement From for Vec.
impl<T> From<(Vec<T>, usize)> for Deque<T> {
    /// Creates a new Deque from a Vec and a maximum length.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<i32> = (vec![1, 2, 3], 3).into();
    /// assert_eq!(deque.len(), 3);
    /// assert_eq!(deque.get(0), Some(&1));
    /// assert_eq!(deque.get(2), Some(&3));
    /// ```
    fn from((mut vec, maxlen): (Vec<T>, usize)) -> Self {
        vec.truncate(maxlen);
        Self {
            deque: VecDeque::from(vec),
            maxlen,
        }
    }
}

// Implement From for VecDeque.
impl<T> From<(VecDeque<T>, usize)> for Deque<T> {
    /// Creates a new Deque from a `VecDeque` and a maximum length.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::VecDeque;
    /// use fixed_deque::Deque;
    ///
    /// let vec_deque: VecDeque<i32> = VecDeque::from(vec![1, 2, 3]);
    /// let deque: Deque<i32> = (vec_deque, 3).into();
    /// assert_eq!(deque.len(), 3);
    /// assert_eq!(deque.get(0), Some(&1));
    /// ```
    fn from((mut deque, maxlen): (VecDeque<T>, usize)) -> Self {
        deque.truncate(maxlen);
        Self { deque, maxlen }
    }
}

impl<T: PartialEq> PartialEq for Deque<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deque == other.deque
    }
}

impl<T: PartialEq> PartialEq<VecDeque<T>> for Deque<T> {
    fn eq(&self, other: &VecDeque<T>) -> bool {
        &self.deque == other
    }
}

impl<T: PartialEq> PartialEq<Deque<T>> for VecDeque<T> {
    fn eq(&self, other: &Deque<T>) -> bool {
        self == &other.deque
    }
}

impl<T: Eq> Eq for Deque<T> {}

impl<T> Index<usize> for Deque<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.deque[index]
    }
}

impl<T> IndexMut<usize> for Deque<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.deque[index]
    }
}

// Implement IntoIterator for owned Deque
impl<T> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.deque.into_iter()
    }
}

// Implement IntoIterator for Deque references
impl<'a, T> IntoIterator for &'a Deque<T> {
    type Item = &'a T;
    type IntoIter = std::collections::vec_deque::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.deque.iter()
    }
}

// Implement IntoIterator for mutable Deque references
impl<'a, T> IntoIterator for &'a mut Deque<T> {
    type Item = &'a mut T;
    type IntoIter = std::collections::vec_deque::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.deque.iter_mut()
    }
}

// Implement FromIterator to create Deque from an iterator
impl<T> FromIterator<T> for Deque<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let deque: VecDeque<T> = iter.into_iter().collect();
        let maxlen = deque.len();
        Self::from_vec_deque(deque, maxlen)
    }
}

impl<T> Extend<T> for Deque<T> {
    /// Extends the deque with the contents of an iterator.
    ///
    /// Elements are added to the back of the deque.
    /// If the number of elements would exceed `maxlen`,
    /// elements are removed from the front to maintain the maximum length constraint.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = Deque::new(3);
    /// deque.push_back(1);
    /// deque.extend([2, 3, 4]);
    /// assert_eq!(deque.len(), 3);
    /// assert_eq!(deque.front(), Some(&2));
    /// assert_eq!(deque.back(), Some(&4));
    /// ```
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();

        // If we know the iterator fits, use direct extend
        if iter
            .size_hint()
            .1
            .is_some_and(|upper| upper <= self.remaining_capacity())
        {
            self.deque.extend(iter);
        } else {
            // Otherwise, push one by one to respect maxlen
            for item in iter {
                self.push_back(item);
            }
        }
    }
}

impl<'a, T> Extend<&'a T> for Deque<T>
where
    T: Copy,
{
    /// Extends the deque with the contents of an iterator of references.
    ///
    /// Elements are copied and added to the back of the deque.
    /// If the number of elements would exceed `maxlen`,
    /// elements are removed from the front to maintain the maximum length constraint.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let mut deque: Deque<i32> = Deque::new(3);
    /// deque.push_back(1);
    /// let values = [2, 3, 4];
    /// deque.extend(&values);
    /// assert_eq!(deque.len(), 3);
    /// assert_eq!(deque.front(), Some(&2));
    /// assert_eq!(deque.back(), Some(&4));
    /// ```
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        let iter = iter.into_iter();

        // If we know the iterator fits, use direct extend
        if iter
            .size_hint()
            .1
            .is_some_and(|upper| upper <= self.remaining_capacity())
        {
            self.deque.extend(iter);
        } else {
            // Otherwise, push one by one to respect maxlen
            for &item in iter {
                self.push_back(item);
            }
        }
    }
}

#[cfg(test)]
mod initialization_tests {
    use super::Deque;
    use std::collections::VecDeque;

    #[test]
    fn test_initialization() {
        let mut deque: Deque<i32> = Deque::new(1024);
        assert_eq!(deque.len(), 0);
        assert_eq!(deque.maxlen(), 1024);

        deque = Deque::from(1, 128);
        assert_eq!(deque.len(), 1);
        assert_eq!(deque.maxlen(), 128);

        let deque = Deque::from_vec(vec![1, 2, 3, 4, 5], 5);
        assert_eq!(deque.len(), 5);
        assert_eq!(deque.maxlen(), 5);

        let vec_deque = VecDeque::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let deque = Deque::from_vec_deque(vec_deque, 100);
        assert_eq!(deque.len(), 10);
        assert_eq!(deque.maxlen(), 100);
    }

    #[test]
    fn test_initialization_with_into() {
        let deque: Deque<&str> = ("a", 1024).into();
        assert_eq!(deque.len(), 1);
        assert_eq!(deque.maxlen(), 1024);

        let mut deque: Deque<usize> = ([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5).into();
        assert_eq!(deque.len(), 5);
        assert_eq!(deque.maxlen(), 5);

        let vec_deque = VecDeque::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        deque = (vec_deque, 100).into();
        assert_eq!(deque.len(), 10);
        assert_eq!(deque.maxlen(), 100);
    }

    #[test]
    fn test_initialization_from_longer_than_maxlen() {
        let deque = Deque::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5);
        assert_eq!(deque.len(), 5);
        assert_eq!(deque.maxlen(), 5);

        let vec_deque = VecDeque::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let deque = Deque::from_vec_deque(vec_deque, 8);
        assert_eq!(deque.len(), 8);
        assert_eq!(deque.maxlen(), 8);

        let deque: Deque<i32> = ([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5).into();
        assert_eq!(deque.len(), 5);
        assert_eq!(deque.maxlen(), 5);

        let deque: Deque<i32> = (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 6).into();
        assert_eq!(deque.len(), 6);
        assert_eq!(deque.maxlen(), 6);

        let vec_deque = VecDeque::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let deque: Deque<i32> = (vec_deque, 8).into();
        assert_eq!(deque.len(), 8);
        assert_eq!(deque.maxlen(), 8);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&8));
    }

    #[test]
    fn test_from_vec_truncation_keeps_first_elements() {
        let deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 3).into();
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&3));
    }

    #[test]
    fn test_from_vec_deque_truncation_keeps_first_elements() {
        let vec_deque = VecDeque::from(vec![10, 20, 30, 40, 50]);
        let deque: Deque<i32> = (vec_deque, 3).into();
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.get(0), Some(&10));
        assert_eq!(deque.get(1), Some(&20));
        assert_eq!(deque.get(2), Some(&30));
    }

    #[test]
    fn test_from_vec_smaller_than_maxlen() {
        let deque: Deque<i32> = (vec![1, 2], 10).into();
        assert_eq!(deque.len(), 2);
        assert_eq!(deque.maxlen(), 10);
    }

    #[test]
    fn test_from_empty_vec() {
        let deque: Deque<i32> = (vec![], 5).into();
        assert_eq!(deque.len(), 0);
        assert_eq!(deque.maxlen(), 5);
    }
}

#[cfg(test)]
mod comparison_tests {
    use super::Deque;
    use std::collections::VecDeque;

    #[test]
    fn test_equal_deques() {
        let mut deque1: Deque<i32> = Deque::new(3);
        deque1.push_back(1);
        deque1.push_back(2);
        deque1.push_back(3);

        let mut deque2: Deque<i32> = Deque::new(3);
        deque2.push_back(1);
        deque2.push_back(2);
        deque2.push_back(3);

        assert_eq!(
            deque1, deque2,
            "Deques with the same elements should be equal"
        );
    }

    #[test]
    fn test_unequal_deques_different_elements() {
        let mut deque1: Deque<i32> = Deque::new(3);
        deque1.push_back(1);
        deque1.push_back(2);
        deque1.push_back(3);

        let mut deque2: Deque<i32> = Deque::new(3);
        deque2.push_back(4);
        deque2.push_back(5);
        deque2.push_back(6);

        assert_ne!(
            deque1, deque2,
            "Deques with different elements should not be equal"
        );
    }

    #[test]
    fn test_unequal_deques_different_lengths() {
        let mut deque1: Deque<i32> = Deque::new(3);
        deque1.push_back(1);
        deque1.push_back(2);

        let mut deque2: Deque<i32> = Deque::new(3);
        deque2.push_back(1);
        deque2.push_back(2);
        deque2.push_back(3);

        assert_ne!(
            deque1, deque2,
            "Deques with different lengths should not be equal"
        );
    }

    #[test]
    fn test_empty_deques_are_equal() {
        let deque1: Deque<i32> = Deque::new(3);
        let deque2: Deque<i32> = Deque::new(3);

        assert_eq!(deque1, deque2, "Empty deques should be equal");
    }

    #[test]
    fn test_partial_eq_with_subsequent_push_back() {
        let mut deque1: Deque<i32> = Deque::new(3);
        deque1.push_back(1);
        deque1.push_back(2);
        deque1.push_back(3);

        let mut deque2: Deque<i32> = Deque::new(3);
        deque2.push_back(1);
        deque2.push_back(2);

        assert_ne!(
            deque1, deque2,
            "Deque with missing elements should not be equal"
        );

        deque2.push_back(3);
        assert_eq!(
            deque1, deque2,
            "After pushing the same element, deques should be equal"
        );
    }

    #[test]
    fn test_deque_eq_vecdeque() {
        let deque: Deque<i32> = Deque::from_vec(vec![5, 6, 7], 3);
        let vec_deque: VecDeque<i32> = vec![5, 6, 7].into_iter().collect();

        assert_eq!(
            deque, vec_deque,
            "Deque should be equal to VecDeque with the same elements"
        );
    }

    #[test]
    fn test_vecdeque_eq_deque() {
        let mut deque1: Deque<i32> = Deque::new(10);
        deque1.push_back(1);
        deque1.push_back(2);
        deque1.push_back(3);

        let mut vecdeque: VecDeque<i32> = VecDeque::new();
        vecdeque.push_back(1);
        vecdeque.push_back(2);
        vecdeque.push_back(3);

        assert_eq!(
            vecdeque, deque1,
            "VecDeque should be equal to Deque with the same elements"
        );
    }

    #[test]
    fn test_unequal_deque_and_vecdeque() {
        let mut deque1: Deque<i32> = Deque::new(3);
        deque1.push_back(1);
        deque1.push_back(2);

        let mut vecdeque: VecDeque<i32> = VecDeque::new();
        vecdeque.push_back(3);
        vecdeque.push_back(4);

        assert_ne!(
            deque1, vecdeque,
            "Deque and VecDeque with different elements should not be equal"
        );
    }
}

#[cfg(test)]
mod extend_tests {
    use super::Deque;

    #[test]
    fn test_extend_owned_values() {
        let mut deque: Deque<i32> = Deque::new(5);
        deque.push_back(1);
        deque.extend([2, 3, 4]);

        assert_eq!(deque.len(), 4);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&4));
    }

    #[test]
    fn test_extend_respects_maxlen() {
        let mut deque: Deque<i32> = Deque::new(3);
        deque.push_back(1);
        deque.extend([2, 3, 4, 5]);

        assert_eq!(deque.len(), 3);
        assert_eq!(deque.maxlen(), 3);
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&5));
    }

    #[test]
    fn test_extend_empty_iterator() {
        let mut deque: Deque<i32> = Deque::new(3);
        deque.push_back(1);
        deque.extend(std::iter::empty::<i32>());

        assert_eq!(deque.len(), 1);
        assert_eq!(deque.front(), Some(&1));
    }

    #[test]
    fn test_extend_on_empty_deque() {
        let mut deque: Deque<i32> = Deque::new(3);
        deque.extend([1, 2, 3]);

        assert_eq!(deque.len(), 3);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&3));
    }

    #[test]
    fn test_extend_on_full_deque() {
        let mut deque: Deque<i32> = Deque::new(3);
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        deque.extend([4, 5]);

        assert_eq!(deque.len(), 3);
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&5));
    }

    #[test]
    fn test_extend_references() {
        let mut deque: Deque<i32> = Deque::new(5);
        deque.push_back(1);
        let values = [2, 3, 4];
        deque.extend(&values);

        assert_eq!(deque.len(), 4);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&4));
    }

    #[test]
    fn test_extend_references_respects_maxlen() {
        let mut deque: Deque<i32> = Deque::new(3);
        deque.push_back(1);
        let values = [2, 3, 4, 5];
        deque.extend(&values);

        assert_eq!(deque.len(), 3);
        assert_eq!(deque.maxlen(), 3);
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&5));
    }

    #[test]
    fn test_extend_from_vec() {
        let mut deque: Deque<i32> = Deque::new(4);
        deque.extend(vec![1, 2, 3, 4, 5, 6]);

        assert_eq!(deque.len(), 4);
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&6));
    }

    #[test]
    fn test_extend_with_iterator() {
        let mut deque: Deque<i32> = Deque::new(3);
        deque.extend((1..=5).map(|x| x * 2));

        assert_eq!(deque.len(), 3);
        assert_eq!(deque.front(), Some(&6));
        assert_eq!(deque.back(), Some(&10));
    }

    #[test]
    fn test_extend_string_references() {
        let mut deque: Deque<char> = Deque::new(4);
        let chars: Vec<char> = "hello".chars().collect();
        deque.extend(&chars);

        assert_eq!(deque.len(), 4);
        assert_eq!(deque.front(), Some(&'e'));
        assert_eq!(deque.back(), Some(&'o'));
    }

    #[test]
    fn test_extend_fits_within_remaining_space() {
        // maxlen 10, contains 1 element, extend adds 5 elements (fits easily)
        let mut deque: Deque<i32> = Deque::new(10);
        deque.push_back(1);
        deque.extend([2, 3, 4, 5, 6]);

        assert_eq!(deque.len(), 6);
        assert_eq!(deque.maxlen(), 10);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&6));
    }

    #[test]
    fn test_extend_fills_exactly() {
        // maxlen 5, contains 2 elements, extend adds exactly 3 to fill it
        let mut deque: Deque<i32> = Deque::new(5);
        deque.push_back(1);
        deque.push_back(2);
        deque.extend([3, 4, 5]);

        assert_eq!(deque.len(), 5);
        assert_eq!(deque.maxlen(), 5);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&5));
    }

    #[test]
    fn test_extend_references_fits_within_remaining_space() {
        // maxlen 10, contains 1 element, extend adds 5 elements (fits easily)
        let mut deque: Deque<i32> = Deque::new(10);
        deque.push_back(1);
        let values = [2, 3, 4, 5, 6];
        deque.extend(&values);

        assert_eq!(deque.len(), 6);
        assert_eq!(deque.maxlen(), 10);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&6));
    }

    #[test]
    fn test_extend_references_fills_exactly() {
        // maxlen 5, contains 2 elements, extend adds exactly 3 to fill it
        let mut deque: Deque<i32> = Deque::new(5);
        deque.push_back(1);
        deque.push_back(2);
        let values = [3, 4, 5];
        deque.extend(&values);

        assert_eq!(deque.len(), 5);
        assert_eq!(deque.maxlen(), 5);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&5));
    }

    #[test]
    fn test_extend_with_filter_iterator() {
        // filter() has unknown upper bound (size_hint returns (0, None) for upper)
        let mut deque: Deque<i32> = Deque::new(3);
        deque.extend((1..=10).filter(|x| x % 2 == 0)); // 2, 4, 6, 8, 10

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&6));
        assert_eq!(deque.back(), Some(&10));
    }

    #[test]
    fn test_extend_with_take_iterator() {
        // take() has known upper bound
        let mut deque: Deque<i32> = Deque::new(3);
        deque.extend((1..=100).take(5)); // 1, 2, 3, 4, 5

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&5));
    }

    #[test]
    fn test_extend_with_skip_iterator() {
        // skip() preserves size_hint from inner iterator
        let mut deque: Deque<i32> = Deque::new(3);
        deque.extend((1..=10).skip(5)); // 6, 7, 8, 9, 10

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&8));
        assert_eq!(deque.back(), Some(&10));
    }

    #[test]
    fn test_extend_with_chain_iterator() {
        // chain() combines two iterators
        let mut deque: Deque<i32> = Deque::new(4);
        deque.extend([1, 2].into_iter().chain([3, 4, 5, 6]));

        assert_eq!(deque.len(), 4);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&6));
    }

    #[test]
    fn test_extend_with_flat_map_iterator() {
        // flat_map() has no upper bound
        let mut deque: Deque<i32> = Deque::new(4);
        deque.extend([1, 2, 3].into_iter().flat_map(|x| [x, x * 10]));
        // produces: 1, 10, 2, 20, 3, 30

        assert_eq!(deque.len(), 4);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&2));
        assert_eq!(deque.back(), Some(&30));
    }

    #[test]
    fn test_extend_with_cycle_take_iterator() {
        // cycle().take() - cycle has no upper bound but take limits it
        let mut deque: Deque<i32> = Deque::new(5);
        deque.extend([1, 2, 3].into_iter().cycle().take(7));
        // produces: 1, 2, 3, 1, 2, 3, 1

        assert_eq!(deque.len(), 5);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&1));
    }

    #[test]
    fn test_extend_with_enumerate_iterator() {
        // enumerate() preserves size_hint
        let mut deque: Deque<(usize, i32)> = Deque::new(3);
        deque.extend([10, 20, 30, 40, 50].into_iter().enumerate());

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&(2, 30)));
        assert_eq!(deque.back(), Some(&(4, 50)));
    }

    #[test]
    fn test_extend_with_zip_iterator() {
        // zip() has upper bound based on shorter iterator
        let mut deque: Deque<(i32, char)> = Deque::new(3);
        deque.extend([1, 2, 3, 4, 5].into_iter().zip(['a', 'b', 'c', 'd', 'e']));

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&(3, 'c')));
        assert_eq!(deque.back(), Some(&(5, 'e')));
    }

    #[test]
    fn test_extend_with_rev_iterator() {
        // rev() preserves size_hint
        let mut deque: Deque<i32> = Deque::new(3);
        deque.extend([1, 2, 3, 4, 5].into_iter().rev());
        // produces: 5, 4, 3, 2, 1

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&1));
    }

    #[test]
    fn test_extend_with_peekable_iterator() {
        // peekable() preserves size_hint
        let mut deque: Deque<i32> = Deque::new(3);
        deque.extend([1, 2, 3, 4, 5].into_iter().peekable());

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&5));
    }

    #[test]
    fn test_extend_with_take_while_iterator() {
        // take_while() has unknown upper bound (returns None)
        let mut deque: Deque<i32> = Deque::new(3);
        deque.extend((1..=10).take_while(|&x| x <= 6)); // 1, 2, 3, 4, 5, 6

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&4));
        assert_eq!(deque.back(), Some(&6));
    }

    #[test]
    fn test_extend_with_skip_while_iterator() {
        // skip_while() has unknown upper bound
        let mut deque: Deque<i32> = Deque::new(3);
        deque.extend((1..=10).skip_while(|&x| x < 5)); // 5, 6, 7, 8, 9, 10

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&8));
        assert_eq!(deque.back(), Some(&10));
    }

    #[test]
    fn test_extend_with_inspect_iterator() {
        // inspect() preserves size_hint
        let mut deque: Deque<i32> = Deque::new(3);
        let mut seen = Vec::new();
        deque.extend((1..=5).inspect(|&x| seen.push(x)));

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(seen, vec![1, 2, 3, 4, 5]); // all items were inspected
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&5));
    }

    #[test]
    fn test_extend_with_fuse_iterator() {
        // fuse() preserves size_hint
        let mut deque: Deque<i32> = Deque::new(3);
        deque.extend([1, 2, 3, 4, 5].into_iter().fuse());

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&5));
    }

    #[test]
    fn test_extend_with_step_by_iterator() {
        // step_by() has known upper bound
        let mut deque: Deque<i32> = Deque::new(3);
        deque.extend((0..=20).step_by(3)); // 0, 3, 6, 9, 12, 15, 18

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&12));
        assert_eq!(deque.back(), Some(&18));
    }

    #[test]
    #[allow(clippy::cloned_instead_of_copied)]
    fn test_extend_with_cloned_iterator() {
        // cloned() preserves size_hint
        let mut deque: Deque<i32> = Deque::new(3);
        let values = [1, 2, 3, 4, 5];
        deque.extend(values.iter().cloned());

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&5));
    }

    #[test]
    fn test_extend_with_copied_iterator() {
        // copied() preserves size_hint
        let mut deque: Deque<i32> = Deque::new(3);
        let values = [1, 2, 3, 4, 5];
        deque.extend(values.iter().copied());

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&5));
    }

    #[test]
    fn test_extend_multiple_times_respects_maxlen() {
        let mut deque: Deque<i32> = Deque::new(5);
        deque.extend([1, 2, 3]);
        deque.extend([4, 5, 6]);
        deque.extend([7, 8]);

        assert_eq!(deque.len(), 5);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&4));
        assert_eq!(deque.back(), Some(&8));
    }

    #[test]
    #[allow(clippy::manual_repeat_n)]
    fn test_extend_with_repeat_take_iterator() {
        // repeat().take() - repeat has no bound, take limits it
        let mut deque: Deque<i32> = Deque::new(3);
        deque.extend(std::iter::repeat(42).take(10));

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&42));
        assert_eq!(deque.back(), Some(&42));
    }

    #[test]
    fn test_extend_with_once_iterator() {
        // once() has exact size hint (1, Some(1))
        let mut deque: Deque<i32> = Deque::new(3);
        deque.push_back(1);
        deque.push_back(2);
        deque.extend(std::iter::once(3));

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&3));
    }

    #[test]
    fn test_extend_with_empty_iterator_type() {
        // empty() has exact size hint (0, Some(0))
        let mut deque: Deque<i32> = Deque::new(3);
        deque.push_back(1);
        deque.extend(std::iter::empty::<i32>());

        assert_eq!(deque.len(), 1);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&1));
    }

    #[test]
    fn test_extend_references_with_filter() {
        let mut deque: Deque<i32> = Deque::new(3);
        let values: Vec<i32> = (1..=10).collect();
        deque.extend(values.iter().filter(|&&x| x % 2 == 0)); // 2, 4, 6, 8, 10

        assert_eq!(deque.len(), 3);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&6));
        assert_eq!(deque.back(), Some(&10));
    }

    #[test]
    fn test_extend_maxlen_one() {
        // Edge case: maxlen of 1
        let mut deque: Deque<i32> = Deque::new(1);
        deque.extend([1, 2, 3, 4, 5]);

        assert_eq!(deque.len(), 1);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&5));
        assert_eq!(deque.back(), Some(&5));
    }

    #[test]
    fn test_extend_large_iterator() {
        // Large iterator to stress test
        let mut deque: Deque<i32> = Deque::new(10);
        deque.extend(0..10000);

        assert_eq!(deque.len(), 10);
        assert!(deque.len() <= deque.maxlen());
        assert_eq!(deque.front(), Some(&9990));
        assert_eq!(deque.back(), Some(&9999));
    }
}

#[cfg(test)]
mod is_full_tests {
    use super::Deque;

    #[test]
    fn test_is_full_empty_deque() {
        let deque: Deque<i32> = Deque::new(3);
        assert!(!deque.is_full());
    }

    #[test]
    fn test_is_full_partially_filled() {
        let mut deque: Deque<i32> = Deque::new(5);
        deque.push_back(1);
        deque.push_back(2);
        assert!(!deque.is_full());
    }

    #[test]
    fn test_is_full_exactly_full() {
        let mut deque: Deque<i32> = Deque::new(3);
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        assert!(deque.is_full());
    }

    #[test]
    fn test_is_full_after_overflow() {
        let mut deque: Deque<i32> = Deque::new(2);
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3); // Overflow
        assert!(deque.is_full());
    }

    #[test]
    fn test_is_full_after_pop() {
        let mut deque: Deque<i32> = Deque::new(2);
        deque.push_back(1);
        deque.push_back(2);
        assert!(deque.is_full());
        deque.pop_front();
        assert!(!deque.is_full());
    }
}

#[cfg(test)]
mod remaining_capacity_tests {
    use super::Deque;

    #[test]
    fn test_remaining_capacity_empty() {
        let deque: Deque<i32> = Deque::new(5);
        assert_eq!(deque.remaining_capacity(), 5);
    }

    #[test]
    fn test_remaining_capacity_partially_filled() {
        let mut deque: Deque<i32> = Deque::new(10);
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        assert_eq!(deque.remaining_capacity(), 7);
    }

    #[test]
    fn test_remaining_capacity_full() {
        let mut deque: Deque<i32> = Deque::new(3);
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        assert_eq!(deque.remaining_capacity(), 0);
    }
}

#[cfg(test)]
mod swap_tests {
    use super::Deque;

    #[test]
    fn test_swap_basic() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        deque.swap(0, 4);
        assert_eq!(deque.get(0), Some(&5));
        assert_eq!(deque.get(4), Some(&1));
        // Middle elements unchanged
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&3));
        assert_eq!(deque.get(3), Some(&4));
    }

    #[test]
    fn test_swap_adjacent() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.swap(0, 1);
        assert_eq!(deque.get(0), Some(&2));
        assert_eq!(deque.get(1), Some(&1));
        assert_eq!(deque.get(2), Some(&3));
    }

    #[test]
    fn test_swap_same_index() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.swap(1, 1);
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&3));
    }

    #[test]
    fn test_swap_two_elements() {
        let mut deque: Deque<char> = (vec!['a', 'b'], 5).into();
        deque.swap(0, 1);
        assert_eq!(deque.get(0), Some(&'b'));
        assert_eq!(deque.get(1), Some(&'a'));
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_swap_out_of_bounds() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.swap(0, 10);
    }
}

#[cfg(test)]
mod truncate_tests {
    use super::Deque;

    #[test]
    fn test_truncate_basic() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        deque.truncate(3);
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&3));
    }

    #[test]
    fn test_truncate_to_zero() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.truncate(0);
        assert!(deque.is_empty());
        assert_eq!(deque.len(), 0);
    }

    #[test]
    fn test_truncate_larger_than_len() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 10).into();
        deque.truncate(100);
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.get(0), Some(&1));
    }

    #[test]
    fn test_truncate_equal_to_len() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.truncate(3);
        assert_eq!(deque.len(), 3);
    }

    #[test]
    fn test_truncate_empty_deque() {
        let mut deque: Deque<i32> = Deque::new(5);
        deque.truncate(2);
        assert!(deque.is_empty());
    }

    #[test]
    fn test_truncate_to_one() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        deque.truncate(1);
        assert_eq!(deque.len(), 1);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&1));
    }
}

#[cfg(test)]
mod remove_tests {
    use super::Deque;

    #[test]
    fn test_remove_middle() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        let removed = deque.remove(2);
        assert_eq!(removed, Some(3));
        assert_eq!(deque.len(), 4);
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&4));
        assert_eq!(deque.get(3), Some(&5));
    }

    #[test]
    fn test_remove_front() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        let removed = deque.remove(0);
        assert_eq!(removed, Some(1));
        assert_eq!(deque.len(), 2);
        assert_eq!(deque.front(), Some(&2));
    }

    #[test]
    fn test_remove_back() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        let removed = deque.remove(2);
        assert_eq!(removed, Some(3));
        assert_eq!(deque.len(), 2);
        assert_eq!(deque.back(), Some(&2));
    }

    #[test]
    fn test_remove_out_of_bounds() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        let removed = deque.remove(10);
        assert_eq!(removed, None);
        assert_eq!(deque.len(), 3);
    }

    #[test]
    fn test_remove_only_element() {
        let mut deque: Deque<i32> = (vec![42], 5).into();
        let removed = deque.remove(0);
        assert_eq!(removed, Some(42));
        assert!(deque.is_empty());
    }

    #[test]
    fn test_remove_from_empty() {
        let mut deque: Deque<i32> = Deque::new(5);
        let removed = deque.remove(0);
        assert_eq!(removed, None);
    }
}

#[cfg(test)]
mod retain_tests {
    use super::Deque;

    #[test]
    fn test_retain_even_numbers() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5, 6], 10).into();
        deque.retain(|&x| x % 2 == 0);
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.get(0), Some(&2));
        assert_eq!(deque.get(1), Some(&4));
        assert_eq!(deque.get(2), Some(&6));
    }

    #[test]
    fn test_retain_all() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.retain(|_| true);
        assert_eq!(deque.len(), 3);
    }

    #[test]
    fn test_retain_none() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.retain(|_| false);
        assert!(deque.is_empty());
    }

    #[test]
    fn test_retain_empty_deque() {
        let mut deque: Deque<i32> = Deque::new(5);
        deque.retain(|&x| x > 0);
        assert!(deque.is_empty());
    }

    #[test]
    fn test_retain_strings() {
        let mut deque: Deque<String> = (
            vec!["hello".to_string(), "world".to_string(), "hi".to_string()],
            5,
        )
            .into();
        deque.retain(|s| s.len() > 2);
        assert_eq!(deque.len(), 2);
        assert_eq!(deque.get(0), Some(&"hello".to_string()));
        assert_eq!(deque.get(1), Some(&"world".to_string()));
    }
}

#[cfg(test)]
mod retain_mut_tests {
    use super::Deque;

    #[test]
    fn test_retain_mut_filter_and_modify() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        deque.retain_mut(|x| {
            if *x % 2 == 0 {
                *x *= 10;
                true
            } else {
                false
            }
        });
        assert_eq!(deque.len(), 2);
        assert_eq!(deque.get(0), Some(&20));
        assert_eq!(deque.get(1), Some(&40));
    }

    #[test]
    fn test_retain_mut_modify_all() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.retain_mut(|x| {
            *x += 100;
            true
        });
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.get(0), Some(&101));
        assert_eq!(deque.get(1), Some(&102));
        assert_eq!(deque.get(2), Some(&103));
    }

    #[test]
    fn test_retain_mut_remove_all() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.retain_mut(|_| false);
        assert!(deque.is_empty());
    }
}

#[cfg(test)]
mod drain_tests {
    use super::Deque;

    #[test]
    fn test_drain_middle() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        let drained: Vec<i32> = deque.drain(1..4).collect();
        assert_eq!(drained, vec![2, 3, 4]);
        assert_eq!(deque.len(), 2);
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(1), Some(&5));
    }

    #[test]
    fn test_drain_all() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        let drained: Vec<i32> = deque.drain(..).collect();
        assert_eq!(drained, vec![1, 2, 3]);
        assert!(deque.is_empty());
    }

    #[test]
    fn test_drain_front() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        let drained: Vec<i32> = deque.drain(..2).collect();
        assert_eq!(drained, vec![1, 2]);
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.front(), Some(&3));
    }

    #[test]
    fn test_drain_back() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        let drained: Vec<i32> = deque.drain(3..).collect();
        assert_eq!(drained, vec![4, 5]);
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.back(), Some(&3));
    }

    #[test]
    fn test_drain_empty_range() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        assert!(deque.drain(1..1).next().is_none());
        assert_eq!(deque.len(), 3);
    }

    #[test]
    fn test_drain_inclusive_range() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        let drained: Vec<i32> = deque.drain(1..=3).collect();
        assert_eq!(drained, vec![2, 3, 4]);
        assert_eq!(deque.len(), 2);
    }
}

#[cfg(test)]
mod rotate_left_tests {
    use super::Deque;

    #[test]
    fn test_rotate_left_basic() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        deque.rotate_left(2);
        assert_eq!(deque.get(0), Some(&3));
        assert_eq!(deque.get(1), Some(&4));
        assert_eq!(deque.get(2), Some(&5));
        assert_eq!(deque.get(3), Some(&1));
        assert_eq!(deque.get(4), Some(&2));
    }

    #[test]
    fn test_rotate_left_zero() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.rotate_left(0);
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&3));
    }

    #[test]
    fn test_rotate_left_full_rotation() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.rotate_left(3);
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&3));
    }

    #[test]
    fn test_rotate_left_one() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4], 10).into();
        deque.rotate_left(1);
        assert_eq!(deque.get(0), Some(&2));
        assert_eq!(deque.get(3), Some(&1));
    }
}

#[cfg(test)]
mod rotate_right_tests {
    use super::Deque;

    #[test]
    fn test_rotate_right_basic() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        deque.rotate_right(2);
        assert_eq!(deque.get(0), Some(&4));
        assert_eq!(deque.get(1), Some(&5));
        assert_eq!(deque.get(2), Some(&1));
        assert_eq!(deque.get(3), Some(&2));
        assert_eq!(deque.get(4), Some(&3));
    }

    #[test]
    fn test_rotate_right_zero() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.rotate_right(0);
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&3));
    }

    #[test]
    fn test_rotate_right_full_rotation() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.rotate_right(3);
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&3));
    }

    #[test]
    fn test_rotate_left_and_right_inverse() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        deque.rotate_left(2);
        deque.rotate_right(2);
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(4), Some(&5));
    }
}

#[cfg(test)]
mod as_slices_tests {
    use super::Deque;

    #[test]
    fn test_as_slices_basic() {
        let mut deque: Deque<i32> = Deque::new(5);
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        let (front, back) = deque.as_slices();
        assert_eq!(front.len() + back.len(), 3);
    }

    #[test]
    fn test_as_slices_empty() {
        let deque: Deque<i32> = Deque::new(5);
        let (front, back) = deque.as_slices();
        assert!(front.is_empty());
        assert!(back.is_empty());
    }

    #[test]
    fn test_as_slices_after_push_front() {
        let mut deque: Deque<i32> = Deque::new(5);
        deque.push_back(2);
        deque.push_back(3);
        deque.push_front(1);
        let (front, back) = deque.as_slices();
        // Together they should contain all elements
        let mut all: Vec<i32> = front.to_vec();
        all.extend_from_slice(back);
        assert_eq!(all, vec![1, 2, 3]);
    }
}

#[cfg(test)]
mod as_mut_slices_tests {
    use super::Deque;

    #[test]
    fn test_as_mut_slices_modify() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        let (front, _back) = deque.as_mut_slices();
        if let Some(first) = front.first_mut() {
            *first = 100;
        }
        assert_eq!(deque.get(0), Some(&100));
    }

    #[test]
    fn test_as_mut_slices_modify_all() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        {
            let (front, back) = deque.as_mut_slices();
            for x in front.iter_mut() {
                *x *= 2;
            }
            for x in back.iter_mut() {
                *x *= 2;
            }
        }
        // All elements should be doubled
        let sum: i32 = deque.iter().sum();
        assert_eq!(sum, 12); // (1+2+3)*2 = 12
    }
}

#[cfg(test)]
mod make_contiguous_tests {
    use super::Deque;

    #[test]
    fn test_make_contiguous_basic() {
        let mut deque: Deque<i32> = Deque::new(5);
        deque.push_back(2);
        deque.push_back(3);
        deque.push_front(1);

        let slice = deque.make_contiguous();
        assert_eq!(slice, &[1, 2, 3]);
    }

    #[test]
    fn test_make_contiguous_already_contiguous() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        let slice = deque.make_contiguous();
        assert_eq!(slice, &[1, 2, 3]);
    }

    #[test]
    fn test_make_contiguous_empty() {
        let mut deque: Deque<i32> = Deque::new(5);
        let slice = deque.make_contiguous();
        assert!(slice.is_empty());
    }

    #[test]
    fn test_make_contiguous_then_as_slices() {
        let mut deque: Deque<i32> = Deque::new(5);
        deque.push_back(2);
        deque.push_front(1);
        deque.push_back(3);

        deque.make_contiguous();
        let (front, back) = deque.as_slices();
        assert_eq!(front, &[1, 2, 3]);
        assert!(back.is_empty());
    }
}

#[cfg(test)]
mod binary_search_tests {
    use super::Deque;

    #[test]
    fn test_binary_search_found() {
        let deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        assert_eq!(deque.binary_search(&3), Ok(2));
        assert_eq!(deque.binary_search(&1), Ok(0));
        assert_eq!(deque.binary_search(&5), Ok(4));
    }

    #[test]
    fn test_binary_search_not_found() {
        let deque: Deque<i32> = (vec![1, 2, 4, 5], 10).into();
        assert_eq!(deque.binary_search(&3), Err(2));
        assert_eq!(deque.binary_search(&0), Err(0));
        assert_eq!(deque.binary_search(&6), Err(4));
    }

    #[test]
    fn test_binary_search_empty() {
        let deque: Deque<i32> = Deque::new(5);
        assert_eq!(deque.binary_search(&1), Err(0));
    }

    #[test]
    fn test_binary_search_single_element() {
        let deque: Deque<i32> = (vec![5], 10).into();
        assert_eq!(deque.binary_search(&5), Ok(0));
        assert_eq!(deque.binary_search(&3), Err(0));
        assert_eq!(deque.binary_search(&7), Err(1));
    }
}

#[cfg(test)]
mod binary_search_by_tests {
    use super::Deque;

    #[test]
    fn test_binary_search_by_found() {
        let deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        assert_eq!(deque.binary_search_by(|x| x.cmp(&3)), Ok(2));
    }

    #[test]
    fn test_binary_search_by_not_found() {
        let deque: Deque<i32> = (vec![1, 2, 4, 5], 10).into();
        assert_eq!(deque.binary_search_by(|x| x.cmp(&3)), Err(2));
    }

    #[test]
    fn test_binary_search_by_custom_comparator() {
        // Search in reverse-sorted deque
        let deque: Deque<i32> = (vec![5, 4, 3, 2, 1], 10).into();
        // Reverse comparison
        assert_eq!(deque.binary_search_by(|x| 3.cmp(x)), Ok(2));
    }
}

#[cfg(test)]
mod binary_search_by_key_tests {
    use super::Deque;

    #[test]
    fn test_binary_search_by_key_found() {
        let deque: Deque<(i32, &str)> = (vec![(1, "a"), (2, "b"), (3, "c"), (4, "d")], 10).into();
        assert_eq!(deque.binary_search_by_key(&2, |&(k, _)| k), Ok(1));
        assert_eq!(deque.binary_search_by_key(&4, |&(k, _)| k), Ok(3));
    }

    #[test]
    fn test_binary_search_by_key_not_found() {
        let deque: Deque<(i32, &str)> = (vec![(1, "a"), (3, "c"), (5, "e")], 10).into();
        assert_eq!(deque.binary_search_by_key(&2, |&(k, _)| k), Err(1));
        assert_eq!(deque.binary_search_by_key(&4, |&(k, _)| k), Err(2));
        assert_eq!(deque.binary_search_by_key(&6, |&(k, _)| k), Err(3));
    }

    #[test]
    fn test_binary_search_by_key_string_key() {
        let deque: Deque<(String, i32)> = (
            vec![
                ("apple".to_string(), 1),
                ("banana".to_string(), 2),
                ("cherry".to_string(), 3),
            ],
            10,
        )
            .into();
        assert_eq!(
            deque.binary_search_by_key(&"banana".to_string(), |(k, _)| k.clone()),
            Ok(1)
        );
    }
}

#[cfg(test)]
mod integration_tests {
    use super::Deque;

    #[test]
    fn test_retain_then_extend() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 5).into();
        deque.retain(|&x| x % 2 == 0);
        assert_eq!(deque.len(), 2);
        deque.extend([6, 7, 8]);
        assert_eq!(deque.len(), 5);
        assert_eq!(deque.front(), Some(&2));
    }

    #[test]
    fn test_drain_then_push() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 5).into();
        let _ = deque.drain(..3);
        assert_eq!(deque.len(), 2);
        deque.push_back(6);
        deque.push_back(7);
        assert_eq!(deque.len(), 4);
        assert_eq!(deque.front(), Some(&4));
    }

    #[test]
    fn test_rotate_then_binary_search() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        // After rotate_left(2): [3, 4, 5, 1, 2] - not sorted!
        // This shows that binary_search requires sorted data
        deque.rotate_left(2);
        // The deque is now [3, 4, 5, 1, 2] which is not sorted
        // binary_search on unsorted data gives undefined but safe results
        let _ = deque.binary_search(&3); // May not find it correctly
    }

    #[test]
    fn test_swap_and_remove() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        deque.swap(0, 4);
        assert_eq!(deque.remove(0), Some(5));
        assert_eq!(deque.len(), 4);
        assert_eq!(deque.front(), Some(&2));
    }

    #[test]
    fn test_truncate_and_remaining_capacity() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        assert_eq!(deque.remaining_capacity(), 5);
        deque.truncate(2);
        assert_eq!(deque.remaining_capacity(), 8);
    }

    #[test]
    fn test_is_full_after_retain() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 5).into();
        assert!(deque.is_full());
        deque.retain(|&x| x > 3);
        assert!(!deque.is_full());
        assert_eq!(deque.remaining_capacity(), 3);
    }
}

#[cfg(test)]
mod count_tests {
    use super::Deque;

    #[test]
    fn test_count_multiple_occurrences() {
        let deque: Deque<i32> = (vec![1, 2, 2, 3, 2, 4], 10).into();
        assert_eq!(deque.count(&2), 3);
    }

    #[test]
    fn test_count_single_occurrence() {
        let deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        assert_eq!(deque.count(&3), 1);
    }

    #[test]
    fn test_count_no_occurrences() {
        let deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        assert_eq!(deque.count(&10), 0);
    }

    #[test]
    fn test_count_empty_deque() {
        let deque: Deque<i32> = Deque::new(5);
        assert_eq!(deque.count(&1), 0);
    }

    #[test]
    fn test_count_all_same() {
        let deque: Deque<i32> = (vec![7, 7, 7, 7, 7], 10).into();
        assert_eq!(deque.count(&7), 5);
    }

    #[test]
    fn test_count_strings() {
        let deque: Deque<&str> = (vec!["a", "b", "a", "c", "a"], 10).into();
        assert_eq!(deque.count(&"a"), 3);
        assert_eq!(deque.count(&"b"), 1);
        assert_eq!(deque.count(&"d"), 0);
    }
}

#[cfg(test)]
mod index_tests {
    use super::Deque;

    #[test]
    fn test_index_found_first() {
        let deque: Deque<i32> = (vec![1, 2, 3, 2, 4], 10).into();
        assert_eq!(deque.index(&2), Some(1));
    }

    #[test]
    fn test_index_found_at_start() {
        let deque: Deque<i32> = (vec![5, 2, 3, 4], 10).into();
        assert_eq!(deque.index(&5), Some(0));
    }

    #[test]
    fn test_index_found_at_end() {
        let deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        assert_eq!(deque.index(&5), Some(4));
    }

    #[test]
    fn test_index_not_found() {
        let deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        assert_eq!(deque.index(&10), None);
    }

    #[test]
    fn test_index_empty_deque() {
        let deque: Deque<i32> = Deque::new(5);
        assert_eq!(deque.index(&1), None);
    }

    #[test]
    fn test_index_range_basic() {
        let deque: Deque<i32> = (vec![1, 2, 3, 2, 4], 10).into();
        assert_eq!(deque.index_range(&2, 0, 5), Some(1));
        assert_eq!(deque.index_range(&2, 2, 5), Some(3));
    }

    #[test]
    fn test_index_range_not_in_range() {
        let deque: Deque<i32> = (vec![1, 2, 3, 2, 4], 10).into();
        assert_eq!(deque.index_range(&2, 4, 5), None);
    }

    #[test]
    fn test_index_range_invalid_range() {
        let deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        assert_eq!(deque.index_range(&2, 5, 3), None); // start > end
    }

    #[test]
    fn test_index_range_end_beyond_len() {
        let deque: Deque<i32> = (vec![1, 2, 3], 10).into();
        assert_eq!(deque.index_range(&3, 0, 100), Some(2));
    }
}

#[cfg(test)]
mod extend_front_tests {
    use super::Deque;

    #[test]
    fn test_extend_front_basic() {
        let mut deque: Deque<i32> = (vec![4, 5], 10).into();
        deque.extend_front([1, 2, 3]);
        // Elements prepended in reverse: 3, 2, 1, 4, 5
        assert_eq!(deque.len(), 5);
        assert_eq!(deque.get(0), Some(&3));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&1));
        assert_eq!(deque.get(3), Some(&4));
        assert_eq!(deque.get(4), Some(&5));
    }

    #[test]
    fn test_extend_front_empty_deque() {
        let mut deque: Deque<i32> = Deque::new(10);
        deque.extend_front([1, 2, 3]);
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.get(0), Some(&3));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&1));
    }

    #[test]
    fn test_extend_front_respects_maxlen() {
        let mut deque: Deque<i32> = (vec![4, 5], 4).into();
        deque.extend_front([1, 2, 3]);
        // maxlen is 4, so oldest elements (from back) are dropped
        assert_eq!(deque.len(), 4);
        assert_eq!(deque.get(0), Some(&3));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&1));
        assert_eq!(deque.get(3), Some(&4));
    }

    #[test]
    fn test_extend_front_empty_iterator() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 10).into();
        deque.extend_front(std::iter::empty());
        assert_eq!(deque.len(), 3);
    }

    #[test]
    fn test_extend_front_maxlen_one() {
        let mut deque: Deque<i32> = Deque::new(1);
        deque.extend_front([1, 2, 3]);
        assert_eq!(deque.len(), 1);
        assert_eq!(deque.front(), Some(&3));
    }
}

#[cfg(test)]
mod reverse_tests {
    use super::Deque;

    #[test]
    fn test_reverse_basic() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        deque.reverse();
        assert_eq!(deque.get(0), Some(&5));
        assert_eq!(deque.get(1), Some(&4));
        assert_eq!(deque.get(2), Some(&3));
        assert_eq!(deque.get(3), Some(&2));
        assert_eq!(deque.get(4), Some(&1));
    }

    #[test]
    fn test_reverse_two_elements() {
        let mut deque: Deque<i32> = (vec![1, 2], 10).into();
        deque.reverse();
        assert_eq!(deque.get(0), Some(&2));
        assert_eq!(deque.get(1), Some(&1));
    }

    #[test]
    fn test_reverse_single_element() {
        let mut deque: Deque<i32> = (vec![42], 10).into();
        deque.reverse();
        assert_eq!(deque.get(0), Some(&42));
    }

    #[test]
    fn test_reverse_empty() {
        let mut deque: Deque<i32> = Deque::new(10);
        deque.reverse(); // Should not panic
        assert!(deque.is_empty());
    }

    #[test]
    fn test_reverse_twice() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        deque.reverse();
        deque.reverse();
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(4), Some(&5));
    }

    #[test]
    fn test_reverse_strings() {
        let mut deque: Deque<&str> = (vec!["a", "b", "c"], 10).into();
        deque.reverse();
        assert_eq!(deque.get(0), Some(&"c"));
        assert_eq!(deque.get(1), Some(&"b"));
        assert_eq!(deque.get(2), Some(&"a"));
    }
}

#[cfg(test)]
mod remove_first_tests {
    use super::Deque;

    #[test]
    fn test_remove_first_found() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 2, 4], 10).into();
        assert_eq!(deque.remove_first(&2), Some(2));
        assert_eq!(deque.len(), 4);
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(1), Some(&3));
        assert_eq!(deque.get(2), Some(&2)); // Second 2 still there
        assert_eq!(deque.get(3), Some(&4));
    }

    #[test]
    fn test_remove_first_not_found() {
        let mut deque: Deque<i32> = (vec![1, 2, 3, 4, 5], 10).into();
        assert_eq!(deque.remove_first(&10), None);
        assert_eq!(deque.len(), 5);
    }

    #[test]
    fn test_remove_first_from_front() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 10).into();
        assert_eq!(deque.remove_first(&1), Some(1));
        assert_eq!(deque.len(), 2);
        assert_eq!(deque.front(), Some(&2));
    }

    #[test]
    fn test_remove_first_from_back() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 10).into();
        assert_eq!(deque.remove_first(&3), Some(3));
        assert_eq!(deque.len(), 2);
        assert_eq!(deque.back(), Some(&2));
    }

    #[test]
    fn test_remove_first_only_element() {
        let mut deque: Deque<i32> = (vec![42], 10).into();
        assert_eq!(deque.remove_first(&42), Some(42));
        assert!(deque.is_empty());
    }

    #[test]
    fn test_remove_first_empty_deque() {
        let mut deque: Deque<i32> = Deque::new(10);
        assert_eq!(deque.remove_first(&1), None);
    }

    #[test]
    fn test_remove_first_all_same() {
        let mut deque: Deque<i32> = (vec![5, 5, 5, 5], 10).into();
        assert_eq!(deque.remove_first(&5), Some(5));
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.count(&5), 3);
    }
}

#[cfg(test)]
mod insert_tests {
    use super::Deque;

    #[test]
    fn test_insert_middle() {
        let mut deque: Deque<i32> = (vec![1, 3, 4], 5).into();
        assert_eq!(deque.insert(1, 2), None);
        assert_eq!(deque.len(), 4);
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&3));
        assert_eq!(deque.get(3), Some(&4));
    }

    #[test]
    fn test_insert_at_front() {
        let mut deque: Deque<i32> = (vec![2, 3, 4], 5).into();
        assert_eq!(deque.insert(0, 1), None);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.len(), 4);
    }

    #[test]
    fn test_insert_at_back() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        assert_eq!(deque.insert(3, 4), None);
        assert_eq!(deque.back(), Some(&4));
        assert_eq!(deque.len(), 4);
    }

    #[test]
    fn test_insert_into_empty() {
        let mut deque: Deque<i32> = Deque::new(5);
        assert_eq!(deque.insert(0, 1), None);
        assert_eq!(deque.len(), 1);
        assert_eq!(deque.front(), Some(&1));
    }

    #[test]
    fn test_insert_full_deque_drops_back() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 3).into();
        let dropped = deque.insert(1, 10);
        assert_eq!(dropped, Some(3)); // 3 was at the back and got dropped
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.iter().copied().collect::<Vec<_>>(), vec![1, 10, 2]);
    }

    #[test]
    fn test_insert_full_deque_at_front() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 3).into();
        let dropped = deque.insert(0, 0);
        assert_eq!(dropped, Some(3));
        assert_eq!(deque.iter().copied().collect::<Vec<_>>(), vec![0, 1, 2]);
    }

    #[test]
    fn test_insert_full_deque_at_back() {
        // Inserting at the back of a full deque: we pop_back first, then insert.
        // So [1,2,3] -> pop_back -> [1,2] (len=2) -> insert(2, 10) -> [1,2,10]
        // Note: after popping, len is 2, so we can only insert at index 0, 1, or 2
        let mut deque: Deque<i32> = (vec![1, 2, 3], 3).into();
        let dropped = deque.insert(2, 10); // Insert at the new back position
        assert_eq!(dropped, Some(3));
        assert_eq!(deque.iter().copied().collect::<Vec<_>>(), vec![1, 2, 10]);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_insert_index_out_of_bounds() {
        let mut deque: Deque<i32> = (vec![1, 2, 3], 5).into();
        deque.insert(10, 4);
    }

    #[test]
    fn test_insert_fills_to_maxlen() {
        let mut deque: Deque<i32> = (vec![1, 2, 4], 4).into();
        assert_eq!(deque.insert(2, 3), None);
        assert_eq!(deque.len(), 4);
        assert!(deque.is_full());
        assert_eq!(deque.get(2), Some(&3));
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use super::Deque;

    #[test]
    fn test_serialize_empty_deque() {
        let deque: Deque<i32> = Deque::new(3);
        let serialized = serde_json::to_string(&deque).expect("Failed to serialize Deque");
        assert_eq!(serialized, "[]");
    }

    #[test]
    fn test_serialize_deque_with_elements() {
        let mut deque: Deque<i32> = Deque::new(2);
        deque.push_back(1);
        deque.push_back(2);
        let serialized = serde_json::to_string(&deque).expect("Failed to serialize Deque");
        assert_eq!(serialized, "[1,2]");
    }

    #[test]
    fn test_deserialize_empty_deque() {
        let data = "[]";
        let deque: Deque<i32> = serde_json::from_str(data).expect("Failed to deserialize Deque");
        assert_eq!(deque.len(), 0);
    }

    #[test]
    fn test_deserialize_deque_with_elements() {
        let data = "[1,2,3]";
        let deque: Deque<i32> = serde_json::from_str(data).expect("Failed to deserialize Deque");
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.get(0), Some(&1));
        assert_eq!(deque.get(1), Some(&2));
        assert_eq!(deque.get(2), Some(&3));
    }

    #[test]
    fn test_serialize_and_deserialize() {
        let mut deque: Deque<i32> = Deque::new(3);
        deque.push_back(10);
        deque.push_back(20);
        deque.push_back(30);

        let serialized = serde_json::to_string(&deque).expect("Failed to serialize Deque");
        assert_eq!(serialized, "[10,20,30]");

        let deserialized: Deque<i32> =
            serde_json::from_str(&serialized).expect("Failed to deserialize Deque");

        assert_eq!(deque, deserialized);
        assert_eq!(deserialized.len(), 3);
        assert_eq!(deserialized.get(0), Some(&10));
        assert_eq!(deserialized.get(1), Some(&20));
        assert_eq!(deserialized.get(2), Some(&30));
    }
}

#[cfg(feature = "serde")]
impl<T: Serialize> Serialize for Deque<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.deque.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Deque<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deque = VecDeque::deserialize(deserializer)?;
        let maxlen = deque.len();
        Ok(Self { deque, maxlen })
    }
}

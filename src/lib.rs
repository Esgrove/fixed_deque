#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A fixed size `VecDeque` to match Python Deque.
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
    pub fn maxlen(&self) -> usize {
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
    pub fn iter(&self) -> std::collections::vec_deque::Iter<T> {
        self.deque.iter()
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
        Deque {
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
        Deque { deque, maxlen }
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
        Deque {
            deque: VecDeque::from(vec),
            maxlen,
        }
    }
}

// Implement From for VecDeque.
impl<T> From<(VecDeque<T>, usize)> for Deque<T> {
    /// Creates a new Deque from a VecDeque and a maximum length.
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
        Deque { deque, maxlen }
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

        assert_eq!(deserialized.len(), 3);
        assert_eq!(deserialized.get(0), Some(&10));
        assert_eq!(deserialized.get(1), Some(&20));
        assert_eq!(deserialized.get(2), Some(&30));
    }
}

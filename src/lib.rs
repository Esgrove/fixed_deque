#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A fixed size `VecDeque` to match Python Deque
/// <https://docs.python.org/3/library/collections.html#collections.deque>
#[derive(Debug, Default, Clone)]
pub struct Deque<T> {
    deque: VecDeque<T>,
    maxlen: usize,
}

impl<T> Deque<T> {
    /// Creates a new Deque with a given maximum length.
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
    /// let deque: Deque<i32> = Deque::new_from(1, 3);
    /// assert_eq!(deque.len(), 1);
    /// assert_eq!(deque.get(0), Some(&1));
    /// ```
    pub fn new_from(value: T, maxlen: usize) -> Self {
        Self {
            deque: VecDeque::from([value]),
            maxlen,
        }
    }

    /// Creates a new Deque from an existing `Vec` with a given maximum length.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_deque::Deque;
    ///
    /// let deque: Deque<i32> = Deque::new_from_vec(vec![1, 2, 3], 3);
    /// assert_eq!(deque.len(), 3);
    /// ```
    #[must_use]
    pub fn new_from_vec(vec: Vec<T>, maxlen: usize) -> Self {
        Self {
            deque: VecDeque::from(vec),
            maxlen,
        }
    }

    /// Creates a new Deque from an existing `VecDeque` with a given maximum length.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::VecDeque;
    /// use fixed_deque::Deque;
    ///
    /// let vec_deque: VecDeque<i32> = VecDeque::from(vec![1, 2, 3]);
    /// let deque: Deque<i32> = Deque::new_from_vec_deque(vec_deque, 3);
    /// assert_eq!(deque.len(), 3);
    /// ```
    #[must_use]
    pub const fn new_from_vec_deque(deque: VecDeque<T>, maxlen: usize) -> Self {
        Self { deque, maxlen }
    }

    /// Add an element to the back of the Deque.
    /// If the Deque exceeds its maximum length, the front element is removed.
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
        self.deque.push_back(value);
        if self.deque.len() > self.maxlen {
            self.deque.pop_front()
        } else {
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

    /// Returns the last element of the Deque.
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
        self.deque.capacity()
    }
}

impl<T: PartialEq> PartialEq for Deque<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deque == other.deque
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
        Self::new_from_vec_deque(deque, maxlen)
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

use std::{iter::{zip, Zip}, slice::Iter, vec::IntoIter};
use crate::Constructor;


// StackMap \\

    /// A key-value map that preserves insertion order.
    #[derive(Constructor)]
    pub struct StackMap<K: PartialEq, V> {
        #[new(default)]
        keys: Vec<K>,
        #[new(default)]
        values: Vec<V>,
    }

    impl<K: PartialEq, V: Default> StackMap<K, V> {
        /// Return a mutable reference to the value for `key`.
        ///
        /// If the key is not present, inserts it with `V::default()` first.
        ///
        /// # Example
        /// ```
        /// use deki_core::collections::StackMap;
        /// let mut map = StackMap::<&str,i32>::new();
        /// *map.entry("count") = 1;
        /// assert_eq!(*map.entry("count"), 1);
        /// ```
        pub fn entry(&mut self, key: K) -> &mut V {
            match self.key_idx(&key) {
                None => {
                    self.keys.push(key);
                    self.values.push(V::default());
                    self.values.last_mut().unwrap()
                }
                Some(id) => &mut self.values[id],
            }
        }
    }

    impl<K: PartialEq, V> StackMap<K, V> {
        /// Return the index of `key`, or `None` if not found.
        ///
        /// # Example
        /// ```
        /// use deki_core::collections::StackMap;
        /// let map = StackMap::<&str,i32>::new();
        /// assert_eq!(map.key_idx(&"missing"), None);
        /// ```
        pub fn key_idx(&self, key: &K) -> Option<usize> {
            self.keys.iter().enumerate().find_map(|(id, k)| if key == k { Some(id) } else { None })
        }
        /// Return a read-only reference to the keys.
        pub fn keys(&self) -> &[K] {
            &self.keys
        }
        /// Return a read-only reference to the values.
        pub fn values(&self) -> &[V] {
            &self.values
        }
        /// Yield `(key, value)` pairs in insertion order.
        ///
        /// # Example
        /// ```
        /// use deki_core::collections::StackMap;
        /// let mut map = StackMap::<&str,i32>::new();
        /// map.entry("a");
        /// map.entry("b");
        /// let keys: Vec<_> = map.iter().map(|(k, _)| *k).collect();
        /// assert_eq!(keys, vec!["a", "b"]);
        /// ```
        pub fn iter(&self) -> Zip<Iter<'_, K>, Iter<'_, V>> {
            zip(self.keys.iter(), self.values.iter())
        }
        /// Consume the map, yielding `(key, value)` pairs in insertion order
        ///
        /// # Example
        /// ```
        /// use deki_core::collections::StackMap;
        /// let mut map = StackMap::<&str,i32>::new();
        /// *map.entry("x") = 1;
        /// let pairs: Vec<_> = map.into_iter().collect();
        /// assert_eq!(pairs, vec![("x", 1)]);
        /// ```
        #[allow(clippy::should_implement_trait)]
        pub fn into_iter(self) -> Zip<IntoIter<K>, IntoIter<V>> {
            zip(self.keys, self.values)
        }
        /// Return `true` if the map contains no keys.
        ///
        /// # Example
        /// ```
        /// use deki_core::collections::StackMap;
        /// let map = StackMap::<&str,i32>::new();
        /// assert!(map.is_empty());
        /// ```
        pub fn is_empty(&self) -> bool {
            self.keys.is_empty()
        }
        /// Return the number of entries in the map.
        pub fn len(&self) -> usize {
            self.keys.len()
        }
    }

    impl<K: PartialEq, V: Default> Default for StackMap<K, V> {
        fn default() -> Self { Self::new() }
    }


// Tests \\

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stackmap_entry_does_not_dedup() {
        let mut map: StackMap<String, i32> = StackMap::new();
        map.entry("a".into());
        map.entry("a".into());
        assert_eq!(map.key_idx(&"a".into()), Some(0));
    }

    #[test]
    fn stackmap_keys_keep_insertion_order() {
        let mut map: StackMap<String, i32> = StackMap::new();
        map.entry("z".into());
        map.entry("a".into());
        map.entry("m".into());
        let keys: Vec<String> = map.keys().to_vec();
        assert_eq!(keys, vec![String::from("z"), String::from("a"), String::from("m")]);
    }

    #[test]
    fn stackmap_len_nonempty() {
        let mut map: StackMap<i32, String> = StackMap::new();
        for i in 0..5 {
            map.entry(i);
        }
        assert_eq!(map.len(), 5);
    }

    #[test]
    fn stackmap_keys_returns_correct_slice() {
        let mut map: StackMap<String, i32> = StackMap::new();
        map.entry("b".into());
        map.entry("a".into());
        assert_eq!(map.keys(), &["b", "a"]);
    }

    #[test]
    fn stackmap_values_returns_correct_slice() {
        let mut map: StackMap<String, i32> = StackMap::new();
        map.entry("x".into());
        *map.entry("x".into()) = 42;
        assert_eq!(map.values(), &[42]);
    }
}

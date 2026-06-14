use std::{iter::{zip, Zip}, slice::Iter, vec::IntoIter};
use crate::Constructor;


// StackMap \\

    /// A key-value map that preserves insertion order.
    ///
    /// # Example
    /// ```
    /// use deki_core::collections::StackMap;
    /// let mut map = StackMap::<&str,i32>::new();
    /// *map.entry("x") = 42;
    /// assert_eq!(map.key_idx(&"x"), Some(0));
    /// ```
    #[derive(Constructor)]
    pub struct StackMap<K: PartialEq, V> {
        #[new(default)]
        pub keys: Vec<K>,
        #[new(default)]
        pub value: Vec<V>,
    }

    impl<K: PartialEq, V: Default> StackMap<K, V> {
        /// Returns a mutable reference to the value for `key`.
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
                    self.value.push(V::default());
                    self.value.last_mut().unwrap()
                }
                Some(id) => &mut self.value[id],
            }
        }
    }

    impl<K: PartialEq, V> StackMap<K, V> {
        /// Returns the index of `key`, or `None` if not found.
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
        /// Yields `(key, value)` pairs in insertion order.
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
            zip(self.keys.iter(), self.value.iter())
        }
        /// Consumes the map, yielding `(key, value)` pairs in insertion order.
        ///
        /// # Example
        /// ```
        /// use deki_core::collections::StackMap;
        /// let mut map = StackMap::<&str,i32>::new();
        /// *map.entry("x") = 1;
        /// let pairs: Vec<_> = map.into_iter().collect();
        /// assert_eq!(pairs, vec![("x", 1)]);
        /// ```
        pub fn into_iter(self) -> Zip<IntoIter<K>, IntoIter<V>> {
            zip(self.keys.into_iter(), self.value.into_iter())
        }
        /// Returns `true` if the map contains no keys.
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
    }

    impl<K: PartialEq, V: Default> Default for StackMap<K, V> {
        fn default() -> Self { Self::new() }
    }


// Tests \\

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stackmap_default_is_empty() {
        let map: StackMap<String, i32> = StackMap::default();
        assert!(map.is_empty());
    }

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
        let keys: Vec<String> = map.keys.iter().cloned().collect();
        assert_eq!(keys, vec![String::from("z"), String::from("a"), String::from("m")]);
    }

    #[test]
    fn stackmap_keys_and_values_len_match() {
        let mut map: StackMap<i32, String> = StackMap::new();
        for i in 0..5 {
            map.entry(i);
        }
        assert_eq!(map.keys.len(), map.value.len());
    }
}

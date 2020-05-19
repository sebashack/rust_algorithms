use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::algorithms::linked_list_queue::LinkedQueue;

struct HashTable<K, V> {
    num_chains: usize,
    chains: Vec<LinkedQueue<(Box<K>, Box<V>)>>,
}

impl<K, V> HashTable<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        let num_chains = 10000;
        let mut chains = Vec::with_capacity(num_chains);

        for i in 0..num_chains {
            chains.insert(i, LinkedQueue::new());
        }

        HashTable { num_chains, chains }
    }

    pub fn get<'a>(&'a self, key: K) -> Option<&'a V> {
        let i = self.hash(&key);

        if let Some(q) = self.chains.get(i) {
            for (k, v) in q.iter() {
                if key == **k {
                    return Some(v);
                }
            }
        }

        return None;
    }

    pub fn put(&mut self, key: K, new_val: V) {
        let i = self.hash(&key);

        if let Some(q) = self.chains.get_mut(i) {
            for (k, v) in q.iter_mut() {
                if key == **k {
                    *v = Box::new(new_val);
                    return;
                }
            }
        }

        self.chains[i].enqueue((Box::new(key), Box::new(new_val)));
    }

    fn hash(&self, key: &K) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        s.finish() as usize % self.num_chains
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::separate_chainining_hash_table::HashTable;

    #[test]
    fn interface_operations_should_work_as_expected() {
        let mut t = HashTable::<String, String>::new();

        t.put(String::from("key"), String::from("some value 1"));
        t.put(String::from("key-ab"), String::from("some value 2"));
        t.put(String::from("key-abcd"), String::from("some value 3"));
        t.put(String::from("key-abcdef"), String::from("some value 4"));
        t.put(
            String::from("key-abcdefghijk"),
            String::from("some value 5"),
        );

        assert!(t.get(String::from("key")) == Some(&String::from("some value 1")));
        assert!(t.get(String::from("key-ab")) == Some(&String::from("some value 2")));
        assert!(t.get(String::from("key-abcd")) == Some(&String::from("some value 3")));
        assert!(t.get(String::from("key-abcdef")) == Some(&String::from("some value 4")));
        assert!(t.get(String::from("key-abcdefghijk")) == Some(&String::from("some value 5")));
    }
}

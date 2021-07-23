use std::collections::HashMap;

#[derive(Debug)]
pub struct Trie<T> {
    children: HashMap<char, Trie<T>>,
    is_leaf: bool,
    value: Option<T>
}

impl<T> Default for Trie<T> {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
            is_leaf: false,
            value: None
        }
    }
}

impl<T: Clone> Trie<T> {
    /// Creates a root node.
    pub fn new() -> Self { Trie::default() }

    /// Returns `true` if the given word exists.
    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = self;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(next_node) => current_node = next_node,
                None => return false
            }
        }

        if current_node.value.is_some() { true } else { false }
    }

    /// Returns all the words from the Trie.
    pub fn get_all(&self) -> Vec<(String, T)> { self.starts_with("") }

    /// Creates the nodes that represent a new word.
    pub fn insert(&mut self, word: &str, value: Option<T>) {
        let mut last_node = word.chars().fold(self, |current_node, c| {
            current_node.is_leaf = false;
            current_node.children.entry(c).or_insert(Trie::default())
        });

        last_node.value = value;

        if last_node.children.is_empty() {
            last_node.is_leaf = true;
        }
    }

    /// Removes a word by reinitializing its `value`
    /// and by updating leaf position as necessary.
    pub fn remove(&mut self, word: &str) -> bool {
        let previous_word_index = self.get_previous_word_index(word);
        let mut current_node = self;

        for (i, c) in word.chars().enumerate() {
            if previous_word_index.is_some() &&
               previous_word_index.unwrap() == i
            {
                current_node.is_leaf = true;
            }

            match current_node.children.get_mut(&c) {
                Some(next_node) => current_node = next_node,
                None => return false
            }
        }

        current_node.value = None;

        true
    }

    /// Returns the words that start with the given prefix.
    /// Words are returned in a tuple with their associated value.
    pub fn starts_with(&self, prefix: &str) -> Vec<(String, T)> {
        let mut current_node = self;

        for c in prefix.chars() {
            match current_node.children.get(&c) {
                Some(next_node) => current_node = next_node,
                None => return Vec::new()
            }
        }

        let mut words = current_node.dfs(prefix, "");

        // If it's a word, add the prefix itself.
        if let Some(value) = &current_node.value {
            words.push((prefix.to_string(), value.clone()));
        }

        words
    }

    /// Returns the index of the most direct parent for a given word.
    fn get_previous_word_index(&self, word: &str) -> Option<usize> {
        let mut current_node = self;
        let mut previous_word_index = None;

        for (i, c) in word.chars().enumerate() {
            if current_node.value.is_some() {
                previous_word_index = Some(i);
            }

            match current_node.children.get(&c) {
                Some(next_node) => current_node = next_node,
                None => return None
            }
        }

        previous_word_index
    }

    /// Depth-first search.
    fn dfs(&self, prefix: &str, buffer: &str) -> Vec<(String, T)> {
        let depth = prefix.chars().count() + buffer.chars().count();
        let mut words = Vec::new();

        for (k, v) in self.children.iter() {
            let mut buffer = buffer.chars()
                .into_iter()
                .take(depth)
                .collect::<String>();

            buffer.push(*k);

            if let Some(value) = &v.value {
                let mut new_word = String::from(prefix);
                new_word.push_str(&buffer);

                words.push((new_word, value.clone()));
            }

            if let Some(next_node) = self.children.get(&k) {
                words.extend(next_node.dfs(prefix, &buffer));
            }
        }

        words
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, hash::Hash};
    use super::*;

    fn eq_unordered<T>(a: &[T], b: &[T]) -> bool
    where
        T: Eq + Hash,
    {
        let a: HashSet<_> = a.iter().collect();
        let b: HashSet<_> = b.iter().collect();

        a == b
    }

    const DICT: [&str; 6] = [
        "bleu",
        "blanc",
        "blanche",
        "bol",
        "blâme",
        "blatte"
    ];

    fn bootstrap() -> Trie<usize> {
        let mut t = Trie::new();

        for (i, w) in DICT.iter().enumerate() {
            t.insert(w, Some(i + 1));
        }

        t
    }

    #[test]
    fn contains() {
        let t = bootstrap();

        assert_eq!(t.contains("bol"), true);
        assert_eq!(t.contains("b"), false);
        assert_eq!(t.contains("figue"), false);
    }

    #[test]
    fn remove() {
        let mut t = bootstrap();

        t.remove("bol");
        t.remove("blanche");
        t.remove("blâme");

        assert_eq!(t.contains("blanc"), true);

        t.remove("blanc");

        assert!(eq_unordered(&t.get_all(), &[
            ("bleu".to_string(), 1),
            ("blatte".to_string(), 6)
        ]));
    }

    #[test]
    fn starts_with() {
        let t = bootstrap();

        assert!(eq_unordered(&t.starts_with("b"), &[
            ("bleu".to_string(), 1),
            ("blanc".to_string(), 2),
            ("blanche".to_string(), 3),
            ("bol".to_string(), 4),
            ("blâme".to_string(), 5),
            ("blatte".to_string(), 6)
        ]));
        assert!(eq_unordered(&t.starts_with("bla"), &[
            ("blanc".to_string(), 2),
            ("blanche".to_string(), 3),
            ("blatte".to_string(), 6)
        ]));
        assert!(eq_unordered(&t.starts_with("bol"), &[("bol".to_string(), 4)]));
        assert!(eq_unordered(&t.starts_with("z"), &[]));
    }
}

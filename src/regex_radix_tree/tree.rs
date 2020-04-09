use crate::regex_radix_tree::{Node, Item};
use crate::regex_radix_tree::regex_node::RegexNode;
use std::fmt::Debug;

#[derive(Debug)]
pub struct RegexRadixTree<T> where T: Item + Debug {
    root: RegexNode<T>,
}

impl<T> RegexRadixTree<T> where T: Item + Debug {
    pub fn new() -> RegexRadixTree<T> {
        RegexRadixTree {
            root: RegexNode::new_empty(),
        }
    }

    pub fn insert(&mut self, item: T) {
        self.root.insert(item, 0)
    }

    pub fn remove(&mut self, id: &str) {
        self.root.remove(id);
    }

    pub fn find(&self, value: &str) -> Option<Vec<&T>> {
        self.root.find(value)
    }

    pub fn cache(&mut self, limit: u64, level: u64) -> u64 {
        self.root.cache(limit, level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestItem {
        regex: String,
        id: String,
    }

    impl Item for TestItem {
        fn node_regex(&self) -> &str {
            self.regex.as_str()
        }

        fn id(&self) -> &str {
            self.id.as_str()
        }
    }

    impl TestItem {
        pub fn new(regex: String) -> TestItem {
            TestItem {
                id: regex.clone(),
                regex,
            }
        }
    }

    #[test]
    fn test_find_no_rule() {
        let tree: RegexRadixTree<TestItem> = RegexRadixTree::new();

        assert_eq!(tree.find("tata").is_some(), false);
        assert_eq!(tree.find("test").is_some(), false);
    }

    #[test]
    fn test_find_one_rule() {
        let item1 = TestItem::new("tata".to_string());
        let mut tree = RegexRadixTree::new();

        tree.insert(item1);

        assert_eq!(tree.find("tata").is_some(), true);
        assert_eq!(tree.find("test").is_some(), false);
    }

    #[test]
    fn test_find_emoji_rule_regex() {
        let mut tree = RegexRadixTree::new();

        tree.insert(TestItem::new("/emoji/(([\\p{Ll}]|\\-|➡️|🤘)+?)".to_string()));

        assert_eq!(tree.find("/emoji/test").is_some(), true);
        assert_eq!(tree.find("/emoji/➡️").is_some(), true);
        assert_eq!(tree.find("/emoji/🤘").is_some(), true);
        assert_eq!(tree.find("/not-emoji").is_some(), false);
    }

    #[test]
    fn test_find_multiple_rule() {
        let mut tree = RegexRadixTree::new();

        tree.insert(TestItem::new("/a/b".to_string()));
        tree.insert(TestItem::new("/a/b/c".to_string()));
        tree.insert(TestItem::new("/a/b/d".to_string()));
        tree.insert(TestItem::new("/b/a".to_string()));

        assert_eq!(tree.find("/a/b").is_some(), true);
        assert_eq!(tree.find("/a/b/c").is_some(), true);
        assert_eq!(tree.find("/a/b/d").is_some(), true);
        assert_eq!(tree.find("/b/a").is_some(), true);

        assert_eq!(tree.find("/b").is_some(), false);
        assert_eq!(tree.find("/a").is_some(), false);
        assert_eq!(tree.find("/no-match").is_some(), false);
    }

    #[test]
    fn test_find_rule_with_regex() {
        let mut tree = RegexRadixTree::new();

        tree.insert(TestItem::new("/a/(.+?)/c".to_string()));

        assert_eq!(tree.find("/a/b/c").is_some(), true);
        assert_eq!(tree.find("/a/b/d").is_some(), false);
        assert_eq!(tree.find("/a/b").is_some(), false);
    }

    #[test]
    fn test_find_multiple_rule_with_regex() {
        let mut tree = RegexRadixTree::new();

        tree.insert(TestItem::new("/a/(.+?)/c".to_string()));
        tree.insert(TestItem::new("/a/(.+?)/b".to_string()));

        assert_eq!(tree.find("/a/b/c").is_some(), true);
        assert_eq!(tree.find("/a/b/d").is_some(), false);
        assert_eq!(tree.find("/a/b").is_some(), false);
        assert_eq!(tree.find("/a/b/b").is_some(), true);
        assert_eq!(tree.find("/a/c/b").is_some(), true);
        assert_eq!(tree.find("/a/c/d").is_some(), false);
        assert_eq!(tree.find("/a/c/").is_some(), false);
    }

    #[test]
    fn test_find_multiple_rule_after_remove() {
        let mut tree = RegexRadixTree::new();

        tree.insert(TestItem::new("/a/b".to_string()));
        tree.insert(TestItem::new("/a/b/c".to_string()));
        tree.insert(TestItem::new("/a/b/d".to_string()));
        tree.insert(TestItem::new("/b/a".to_string()));

        assert_eq!(tree.find("/a/b").is_some(), true);
        assert_eq!(tree.find("/a/b/c").is_some(), true);

        tree.remove("/a/b");

        assert_eq!(tree.find("/a/b").is_some(), false);
        assert_eq!(tree.find("/a/b/c").is_some(), true);
    }


    #[test]
    fn test_find_emoji_weird_rule_regex() {
        let mut tree = RegexRadixTree::new();

        tree.insert(TestItem::new("/string/from/(?:)".to_string()));
        tree.insert(TestItem::new("/string\\-uppercase/from/(?:([\\p{Lu}\\p{Lt}])+?)".to_string()));
        tree.insert(TestItem::new("/string\\-ending/from/(?:([\\p{Ll}]|\\-)+?JOHN\\-SNOW)".to_string()));
        tree.insert(TestItem::new("/string\\-lowercase/from/(?:([\\p{Ll}])+?)".to_string()));
        tree.insert(TestItem::new("/string\\-starting/from/(?:JOHN\\-SNOW([\\p{Ll}]|\\-)+?)".to_string()));
        tree.insert(TestItem::new("/string\\-lowercase\\-uppercase\\-digits/from/(?:([\\p{Ll}\\p{Lu}\\p{Lt}0-9])+?)".to_string()));
        tree.insert(TestItem::new("/string\\-lowercase\\-uppercase\\-digits\\-allowPercentEncodedChars\\-specificCharacters/from/(?:([\\p{Ll}\\p{Lu}\\p{Lt}0-9]|\\-|\\.|\\(|\\)|%[0-9A-Z]{2})+?)".to_string()));
        tree.insert(TestItem::new("/string\\-starting\\-shit/from/(?:\\(\\[A\\-Z\\]\\)\\+([\\p{Ll}]|\\-)+?)".to_string()));
        tree.insert(TestItem::new("/string\\-lowercase\\-specificCharacters\\-emoji/from/(?:([\\p{Ll}]|\\-|🤘)+?)".to_string()));
        tree.insert(TestItem::new("/string\\-lowercase\\-digits\\-allowPercentEncodedChars/from/(?:([\\p{Ll}0-9]|%[0-9A-Z]{2})+?)".to_string()));
        tree.insert(TestItem::new("/string\\-allowLowercaseAlphabet\\-specificCharacters\\-starting\\-containing/from/(?:JOHN\\-SNOW(([\\p{Ll}]|\\-)*?L33T([\\p{Ll}]|\\-)*?)+?)".to_string()));
        tree.insert(TestItem::new("/string\\-allowPercentEncodedChars/from/(?:(%[0-9A-Z]{2})+?)".to_string()));
        tree.insert(TestItem::new("/string\\-containing/from/(?:(L33T)+?)".to_string()));
        tree.insert(TestItem::new("/string\\-specificCharacters/from/(?:(\\.|\\-|\\+|_|/)+?)".to_string()));
        tree.insert(TestItem::new("/string\\-specificCharacters\\-other/from/(?:(a|\\-|z)+?)".to_string()));

        assert_eq!(tree.find("/string-lowercase/from/coucou").is_some(), true);
        assert_eq!(tree.find("/string-lowercase-specificCharacters-emoji/from/you-rock-dude-🤘").is_some(), true);
    }
}
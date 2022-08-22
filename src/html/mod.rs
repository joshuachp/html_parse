//! Html struct to parse html documents or fragments and get the result

use std::borrow::Cow;

use forest_ds::tree::Tree;
use html5ever::{
    driver, local_name, namespace_url, ns, tendril::TendrilSink, tree_builder::QuirksMode, QualName,
};

use self::node::Node;

mod node;
#[cfg(feature = "serde")]
mod serde;
mod tree_sink;

/// Create by parsing a document or fragment
#[derive(Debug, Clone)]
pub struct Html {
    /// The tree of the parsed HTML
    pub tree: Tree<Node>,
    /// QuirksMode
    pub quirks_mode: QuirksMode,
    /// Errors during parsing
    pub errors: Vec<Cow<'static, str>>,
}

impl Html {
    /// Creates a new document
    pub fn new_document() -> Self {
        let mut tree = Tree::new();
        tree.append_child(Node::Document);

        Html {
            tree,
            quirks_mode: QuirksMode::NoQuirks,
            errors: Vec::new(),
        }
    }

    /// Creates a new fragment
    pub fn new_fragment() -> Self {
        let mut tree = Tree::new();
        tree.append_child(Node::Fragment);

        Html {
            errors: Vec::new(),
            quirks_mode: QuirksMode::NoQuirks,
            tree,
        }
    }

    /// Parses a string of HTML as a document.
    pub fn parse_document(document: &str) -> Self {
        let parser = driver::parse_document(Self::new_document(), Default::default());

        parser.one(document)
    }

    /// Parses a string of HTML as a fragment.
    pub fn parse_fragment(fragment: &str) -> Self {
        let parser = driver::parse_fragment(
            Self::new_fragment(),
            Default::default(),
            QualName::new(None, ns!(html), local_name!("body")),
            Vec::new(),
        );

        parser.one(fragment)
    }
}

#[cfg(test)]
mod test {
    use super::Html;

    #[test]
    fn should_parse_document() {
        let example = include_str!("../../assets/example.html");

        Html::parse_document(example);
    }
}

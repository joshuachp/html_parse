//! Html struct to parse html documents or fragments and get the result

use std::borrow::Cow;

use forest_ds::tree::Tree;
use html5ever::tree_builder::QuirksMode;

use self::node::Node;

mod node;
mod tree_sink;

/// Create by parsing a document or fragment
#[derive(Debug, Clone)]
pub struct Html {
    tree: Tree<Node>,
    quirks_mode: QuirksMode,
    errors: Vec<Cow<'static, str>>,
}

impl Html {
    pub fn new_document() -> Self {
        Html {
            tree: Tree::new(),
            quirks_mode: QuirksMode::NoQuirks,
            errors: Vec::new(),
        }
    }

    pub fn parse_document(document: &str) {}
}

//! Html struct to parse html documents or fragments and get the result

mod tree_sink;

/// Create by parsing a document or fragment
#[derive(Debug, Clone, Copy)]
pub struct Html {}

impl Html {
    pub fn new_document() -> Self {
        Html {}
    }

    pub fn parse_document(document: &str) {}
}

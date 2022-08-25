use std::ops::Deref;

use html5ever::{tendril::StrTendril, Attribute, LocalName, QualName};
use indexmap::IndexSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    /// The document root.
    Document,

    /// The fragment root.
    Fragment,

    /// A doctype.
    Doctype(Doctype),

    /// A comment.
    Comment(Comment),

    /// Text.
    Text(Text),

    /// An element.
    Element(Element),

    /// A processing instruction.
    ProcessingInstruction(ProcessingInstruction),
}

impl Node {
    /// Returns self as an element.
    pub fn as_element(&self) -> Option<&Element> {
        match *self {
            Node::Element(ref e) => Some(e),
            _ => None,
        }
    }
}

/// A doctype.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Doctype {
    /// The doctype name.
    pub name: StrTendril,

    /// The doctype public ID.
    pub public_id: StrTendril,

    /// The doctype system ID.
    pub system_id: StrTendril,
}

// An HTML comment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    /// The comment text.
    pub comment: StrTendril,
}

/// HTML text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    /// The text.
    pub text: StrTendril,
}

pub type Attributes = indexmap::IndexMap<QualName, StrTendril>;

/// An HTML element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    /// The element name.
    pub name: QualName,

    /// The element ID.
    pub id: Option<LocalName>,

    /// The element classes.
    pub classes: IndexSet<LocalName>,

    /// The element attributes.
    pub attrs: Attributes,
}

impl Element {
    #[doc(hidden)]
    pub fn new(name: QualName, attrs: Vec<Attribute>) -> Self {
        let id = attrs
            .iter()
            .find(|a| a.name.local.deref() == "id")
            .map(|a| LocalName::from(a.value.deref()));

        let classes: IndexSet<LocalName> = attrs
            .iter()
            .find(|a| a.name.local.deref() == "class")
            .map_or(IndexSet::new(), |a| {
                a.value
                    .deref()
                    .split_whitespace()
                    .map(LocalName::from)
                    .collect()
            });

        Element {
            attrs: attrs.into_iter().map(|a| (a.name, a.value)).collect(),
            name,
            id,
            classes,
        }
    }
}

/// HTML Processing Instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessingInstruction {
    /// The PI target.
    pub target: StrTendril,

    /// The PI data.
    pub data: StrTendril,
}

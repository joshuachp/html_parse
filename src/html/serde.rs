use indexmap::IndexMap;
use serde::{ser::SerializeStructVariant, Serialize};

use super::{node::Node, Html};

impl Serialize for Html {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.tree.serialize(serializer)
    }
}

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Node::Document => serializer.serialize_unit_variant("Node", 0, "Document"),
            Node::Fragment => serializer.serialize_unit_variant("Node", 1, "Fragment"),
            Node::Doctype(doctype) => {
                let mut doctype_struct =
                    serializer.serialize_struct_variant("Node", 2, "Doctype", 3)?;
                doctype_struct.serialize_field("name", &doctype.name.to_string())?;
                doctype_struct.serialize_field("public_id", &doctype.public_id.to_string())?;
                doctype_struct.serialize_field("system_id", &doctype.system_id.to_string())?;

                doctype_struct.end()
            }
            Node::Comment(comment) => serializer.serialize_newtype_variant(
                "Node",
                3,
                "Comment",
                &comment.comment.to_string(),
            ),
            Node::Text(text) => {
                serializer.serialize_newtype_variant("Node", 4, "Text", &text.text.to_string())
            }
            Node::Element(element) => {
                let mut element_struct =
                    serializer.serialize_struct_variant("Node", 5, "Element", 4)?;

                element_struct.serialize_field("name", &element.name.local)?;
                element_struct.serialize_field("id", &element.id)?;
                element_struct.serialize_field("classes", &element.classes)?;

                let attributes: IndexMap<String, String> = element
                    .attrs
                    .iter()
                    .map(|(name, value)| (name.local.to_string(), value.to_string()))
                    .collect();

                element_struct.serialize_field("attributes", &attributes)?;

                element_struct.end()
            }
            Node::ProcessingInstruction(pi) => {
                let mut pi_struct =
                    serializer.serialize_struct_variant("Node", 6, "ProcessingInstruction", 2)?;
                pi_struct.serialize_field("target", &pi.target.to_string())?;
                pi_struct.serialize_field("data", &pi.data.to_string())?;

                pi_struct.end()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::html::Html;

    #[test]
    fn should_serialize() {
        let example = include_str!("../../assets/example.html");

        let html = Html::parse_document(example);

        serde_json::to_string(&html).unwrap();
    }
}
